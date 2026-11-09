//! Provider-neutral requests and structured results for agent callers.
use crate::{
    BuildOptions, CoverOptions, Error, ErrorKind, Metadata, Result, Summary, package,
    source::{self, Source},
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BookRequest {
    pub schema_version: u32,
    pub metadata: Metadata,
    pub inputs: Vec<RequestInput>,
    #[serde(default, deserialize_with = "present")]
    pub cover: Option<CoverOptions>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestInput {
    #[serde(default, deserialize_with = "present")]
    pub path: Option<PathBuf>,
    #[serde(default, deserialize_with = "present")]
    pub content: Option<String>,
    #[serde(default, deserialize_with = "present")]
    pub format: Option<Format>,
    #[serde(default, deserialize_with = "present")]
    pub name: Option<String>,
}

fn present<'de, D, T>(deserializer: D) -> std::result::Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Text,
    Markdown,
}

impl Format {
    fn extension(self) -> &'static str {
        match self {
            Self::Text => "txt",
            Self::Markdown => "md",
        }
    }
}

pub struct RequestOptions {
    pub output: PathBuf,
    pub base_dir: Option<PathBuf>,
    pub request_path: Option<PathBuf>,
    pub overwrite: bool,
    pub epubcheck: Option<PathBuf>,
}

fn invalid(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::Input, message)
}

pub fn build_request(text: &str, options: &RequestOptions) -> Result<Summary> {
    if text.len() > 64 * 1024 * 1024 {
        return Err(invalid("Request limit: 64 MiB."));
    }
    let request: BookRequest =
        serde_json::from_str(text).map_err(|e| invalid(format!("Invalid book request: {e}")))?;
    if request.schema_version != 1 {
        return Err(invalid(format!(
            "Unsupported schema_version {}; expected 1.",
            request.schema_version
        )));
    }
    if request.inputs.is_empty() || request.inputs.len() > 1000 {
        return Err(invalid("Supply 1 to 1,000 ordered inputs."));
    }
    let base = match (&options.base_dir, &options.request_path) {
        (Some(base), _) => base.clone(),
        (None, Some(file)) => file.parent().unwrap_or(Path::new(".")).to_path_buf(),
        (None, None)
            if request.inputs.iter().any(|i| i.path.is_some()) || request.cover.is_some() =>
        {
            return Err(invalid(
                "Streamed requests with file references require --base-dir.",
            ));
        }
        (None, None) => {
            std::env::current_dir().map_err(|e| Error::new(ErrorKind::Io, e.to_string()))?
        }
    }
    .canonicalize()
    .map_err(|e| Error::new(ErrorKind::Io, format!("Request base directory: {e}")))?;
    let mut sources = Vec::new();
    let mut total = 0usize;
    let mut names = HashSet::new();
    for (index, input) in request.inputs.into_iter().enumerate() {
        let source = match (input.path, input.content) {
            (Some(path), None) => {
                if input.name.is_some() {
                    return Err(invalid(format!(
                        "inputs[{index}]: name is only for inline content."
                    )));
                }
                let mut source = source::load(&base.join(path))?;
                if let Some(format) = input.format {
                    source.format = format.extension().into();
                }
                source
            }
            (None, Some(text)) => {
                let format = input.format.ok_or_else(|| {
                    invalid(format!(
                        "inputs[{index}]: inline content requires format (text or markdown)."
                    ))
                })?;
                let name = input
                    .name
                    .unwrap_or_else(|| format!("input-{}.{}", index + 1, format.extension()));
                let path = Path::new(&name);
                if path.components().count() != 1
                    || !matches!(
                        path.components().next(),
                        Some(std::path::Component::Normal(_))
                    )
                    || name.contains('\\')
                    || name.contains(':')
                {
                    return Err(invalid(format!(
                        "inputs[{index}]: name must be a simple filename."
                    )));
                }
                Source {
                    path: base.join(path),
                    text,
                    format: format.extension().into(),
                }
            }
            _ => {
                return Err(invalid(format!(
                    "inputs[{index}]: supply exactly one of path or content."
                )));
            }
        };
        total += source.text.len();
        if total > 64 * 1024 * 1024 {
            return Err(invalid("Input limit: 64 MiB total text."));
        }
        if !names.insert(source.path.clone()) {
            return Err(invalid(format!(
                "inputs[{index}]: duplicate input name or source path."
            )));
        }
        sources.push(source);
    }
    let mut book = source::prepare_sources(&sources, request.metadata.clone())?;
    let mut protected = sources.iter().map(|s| s.path.clone()).collect::<Vec<_>>();
    if let Some(mut cover) = request.cover {
        if cover.path.as_os_str().is_empty() {
            return Err(invalid("Supply a cover image path."));
        }
        cover.path = base.join(cover.path);
        book.cover = Some(crate::cover::load(&cover, &book.metadata.title)?);
        protected.push(cover.path);
    }
    if let Some(path) = &options.request_path {
        protected.push(path.clone());
    }
    package::publish(
        &BuildOptions {
            inputs: protected,
            output: options.output.clone(),
            metadata: request.metadata,
            overwrite: options.overwrite,
            epubcheck: options.epubcheck.clone(),
        },
        &book,
    )
}

pub fn capabilities() -> Value {
    json!({
        "schema_version": 1, "tool": "agent-epub", "tool_version": env!("CARGO_PKG_VERSION"),
        "request_schema": "book-request.v1", "request_schema_document": include_str!("../schemas/book-request.v1.json").parse::<Value>().expect("embedded schema is valid JSON"),
        "commands": ["build", "validate", "capabilities"], "formats": ["text", "markdown"],
        "inputs": ["ordered_files", "request_file", "streamed_request", "inline_content"],
        "navigation": ["contents", "chapter_contents", "previous_next", "return_to_contents"],
        "cover_images": true, "cover_formats": ["jpeg", "png"],
        "unsupported": ["inline_images", "tables", "notes", "task_lists", "raw_html", "fixed_layout", "multimedia", "drm"],
        "limits": {"source_bytes": 20971520, "total_source_bytes": 67108864, "request_bytes": 67108864, "chapters": 1000, "cover_bytes": 10485760, "cover_max_side": 8000, "cover_max_pixels": 16000000},
        "checks": {"structural_profile": "always", "epubcheck": "optional_external", "reader_review": "not_automated", "accessibility_assessment": "not_automated"}
    })
}

pub fn result(
    operation: &str,
    outcome: &Result<Summary>,
    output: Option<&Path>,
    elapsed_ms: u128,
) -> Value {
    match outcome {
        Ok(summary) => json!({
            "schema_version": 1, "tool_version": env!("CARGO_PKG_VERSION"), "operation": operation, "status": "success",
            "output": output.and_then(|p| p.canonicalize().ok()), "chapters": summary.chapters, "cover": summary.cover,
            "checks": {"structural_profile": "passed", "epubcheck": if summary.independent_validation {"passed"} else {"not_run"}},
            "review_items": ["Reader appearance and accessibility require separate review."], "errors": [], "elapsed_ms": elapsed_ms
        }),
        Err(error) => json!({
            "schema_version": 1, "tool_version": env!("CARGO_PKG_VERSION"), "operation": operation, "status": "failed", "output": null,
            "checks": {"structural_profile": "not_claimed", "epubcheck": "not_claimed"},
            "errors": [{"code": match error.kind { ErrorKind::Input => "invalid_input", ErrorKind::Io => "io_error", ErrorKind::Validation => "invalid_epub", ErrorKind::Validator => "validator_failed" }, "message": error.message}], "elapsed_ms": elapsed_ms
        }),
    }
}
