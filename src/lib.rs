use std::{fmt, path::PathBuf};

pub mod agent;
mod cover;
mod notes;
mod package;
mod source;
mod validation;

pub use package::{build, build_with_cover};
pub use validation::validate;

pub fn validate_with_epubcheck(
    path: &std::path::Path,
    executable: Option<&std::path::Path>,
) -> Result<Summary> {
    let mut summary = validate(path)?;
    if let Some(executable) = executable {
        package::run_epubcheck(executable, path)?;
        summary.independent_validation = true;
    }
    Ok(summary)
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    Input = 2,
    Io = 3,
    Validation = 4,
    Validator = 5,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
    pub issue_code: &'static str,
    pub source: Option<PathBuf>,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub remedy: &'static str,
}

impl Error {
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            issue_code: match kind {
                ErrorKind::Input => "invalid_input",
                ErrorKind::Io => "io_error",
                ErrorKind::Validation => "invalid_epub",
                ErrorKind::Validator => "validator_failed",
            },
            source: None,
            line: None,
            column: None,
            remedy: match kind {
                ErrorKind::Input => "Correct the input described in message and retry.",
                ErrorKind::Io => "Check file paths, permissions and available storage, then retry.",
                ErrorKind::Validation => {
                    "Rebuild from supported sources; report a reproduction if checks still fail."
                }
                ErrorKind::Validator => {
                    "Check the validator executable and its stderr report, then retry."
                }
            },
        }
    }
}

impl Error {
    pub(crate) fn issue(mut self, code: &'static str, remedy: &'static str) -> Self {
        self.issue_code = code;
        self.remedy = remedy;
        self
    }

    pub(crate) fn at(mut self, path: &std::path::Path, text: &str, offset: usize) -> Self {
        self.source = Some(path.to_path_buf());
        let prefix = &text[..offset];
        self.line = Some(prefix.bytes().filter(|&b| b == b'\n').count() + 1);
        self.column = Some(prefix.rsplit('\n').next().unwrap_or("").chars().count() + 1);
        self
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)?;
        if let Some(source) = &self.source {
            write!(f, " [{}", source.display())?;
            if let (Some(line), Some(column)) = (self.line, self.column) {
                write!(f, ":{line}:{column}")?;
            }
            write!(f, "]")?;
        }
        write!(f, " ({}). {}", self.issue_code, self.remedy)
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    pub title: String,
    pub language: String,
    pub author: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BuildOptions {
    pub inputs: Vec<PathBuf>,
    pub output: PathBuf,
    pub metadata: Metadata,
    pub split_level: u8,
    pub overwrite: bool,
    pub epubcheck: Option<PathBuf>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoverOptions {
    pub path: PathBuf,
    pub alt: Option<String>,
}

#[derive(Debug)]
pub struct Summary {
    pub chapters: usize,
    pub cover: bool,
    pub independent_validation: bool,
}

pub(crate) fn xml_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

pub(crate) fn valid_xml_text(value: &str) -> bool {
    value.chars().all(|c| matches!(c, '\t' | '\n' | '\r' | '\u{20}'..='\u{d7ff}' | '\u{e000}'..='\u{fffd}' | '\u{10000}'..='\u{10ffff}'))
}
