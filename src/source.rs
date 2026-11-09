use std::{collections::HashMap, fs, path::PathBuf};

use percent_encoding::percent_decode_str;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd, html};
use url::Url;

use crate::{Error, ErrorKind, Metadata, Result, valid_xml_text, xml_text};

#[derive(Debug)]
pub(crate) struct Book {
    pub metadata: Metadata,
    pub chapters: Vec<Chapter>,
    pub cover: Option<crate::cover::Cover>,
}

#[derive(Debug)]
pub(crate) struct Chapter {
    pub title: String,
    pub body: String,
}

struct Draft {
    title: String,
    events: Vec<Event<'static>>,
    source: usize,
    plain: Option<String>,
    has_title: bool,
}

pub(crate) fn chapter_file(index: usize) -> String {
    format!("chapter-{:04}.xhtml", index + 1)
}

fn input_error(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::Input, message)
}

pub(crate) struct Source {
    pub path: PathBuf,
    pub text: String,
    pub format: String,
}

pub(crate) fn load(input: &std::path::Path) -> Result<Source> {
    let path = input
        .canonicalize()
        .map_err(|e| Error::new(ErrorKind::Io, format!("{}: {e}", input.display())))?;
    let size = fs::metadata(&path)
        .map_err(|e| Error::new(ErrorKind::Io, e.to_string()))?
        .len();
    if size > 20 * 1024 * 1024 {
        return Err(input_error("Input limit: 20 MiB per source."));
    }
    let text = fs::read_to_string(&path).map_err(|e| {
        Error::new(
            if e.kind() == std::io::ErrorKind::InvalidData {
                ErrorKind::Input
            } else {
                ErrorKind::Io
            },
            format!("{}: expected readable UTF-8 text: {e}", input.display()),
        )
    })?;
    let format = input
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    Ok(Source { path, text, format })
}

pub(crate) fn prepare(inputs: &[PathBuf], metadata: Metadata) -> Result<Book> {
    if inputs.len() > 1000 {
        return Err(input_error("Supply at most 1,000 source files."));
    }
    let mut sources = Vec::new();
    let mut total = 0;
    for input in inputs {
        let source = load(input)?;
        total += source.text.len();
        if total > 64 * 1024 * 1024 {
            return Err(input_error("Input limit: 64 MiB total text."));
        }
        sources.push(source);
    }
    prepare_sources(&sources, metadata)
}

pub(crate) fn prepare_sources(inputs: &[Source], metadata: Metadata) -> Result<Book> {
    if metadata.title.trim().is_empty() || !valid_xml_text(&metadata.title) {
        return Err(input_error(
            "Supply a nonempty title containing valid XML characters.",
        ));
    }
    let language = language_tags::LanguageTag::parse(&metadata.language)
        .map_err(|e| input_error(format!("Invalid language tag: {e}. Use en, nl or en-US.")))?;
    language
        .validate()
        .map_err(|e| input_error(format!("Invalid language tag: {e}.")))?;
    if metadata
        .author
        .as_ref()
        .is_some_and(|s| s.trim().is_empty() || !valid_xml_text(s))
    {
        return Err(input_error(
            "Author must be nonempty and contain valid XML characters.",
        ));
    }
    if inputs.is_empty() {
        return Err(input_error(
            "Supply at least one .txt, .md or .markdown input.",
        ));
    }
    let mut paths = Vec::new();
    let mut drafts = Vec::new();
    let mut total = 0;
    for (source, source_input) in inputs.iter().enumerate() {
        let input = &source_input.path;
        let path = input.clone();
        if paths.contains(&path) {
            return Err(input_error(format!(
                "{}: source supplied more than once.",
                input.display()
            )));
        }
        let size = source_input.text.len();
        total += size;
        if size > 20 * 1024 * 1024 || total > 64 * 1024 * 1024 {
            return Err(input_error(
                "Input limit: 20 MiB per source and 64 MiB total.",
            ));
        }
        let text = &source_input.text;
        let text = text
            .trim_start_matches('\u{feff}')
            .replace("\r\n", "\n")
            .replace('\r', "\n");
        if text.trim().is_empty() || !valid_xml_text(&text) {
            return Err(input_error(format!(
                "{}: source is empty or contains invalid XML characters.",
                input.display()
            )));
        }
        let label = input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Chapter")
            .to_string();
        if !valid_xml_text(&label) {
            return Err(input_error(
                "Source filename contains invalid XML characters.",
            ));
        }
        match source_input.format.as_str() {
            "txt" => drafts.push(Draft {
                title: label,
                events: vec![],
                source,
                plain: Some(text),
                has_title: false,
            }),
            "md" | "markdown" => {
                let options =
                    Options::ENABLE_TABLES | Options::ENABLE_FOOTNOTES | Options::ENABLE_TASKLISTS;
                let mut groups: Vec<Vec<Event<'static>>> = vec![vec![]];
                let mut depth = 0usize;
                for (event, range) in Parser::new_ext(&text, options).into_offset_iter() {
                    let unsupported = match &event {
                        Event::Html(_) | Event::InlineHtml(_) => Some("raw HTML"),
                        Event::Start(Tag::Image { .. }) => {
                            Some("inline images (use the separate cover option for a cover)")
                        }
                        Event::Start(Tag::Table(_)) => Some("tables"),
                        Event::Start(Tag::FootnoteDefinition(_)) | Event::FootnoteReference(_) => {
                            Some("notes")
                        }
                        Event::TaskListMarker(_) => Some("task lists"),
                        _ => None,
                    };
                    if let Some(feature) = unsupported {
                        let line = text[..range.start].bytes().filter(|&b| b == b'\n').count() + 1;
                        return Err(input_error(format!(
                            "{}:{line}: unsupported {feature}; supply plain text or supported Markdown instead.",
                            input.display()
                        )));
                    }
                    if depth == 0
                        && matches!(
                            event,
                            Event::Start(Tag::Heading {
                                level: HeadingLevel::H1,
                                ..
                            })
                        )
                        && !groups.last().unwrap().is_empty()
                    {
                        groups.push(vec![]);
                    }
                    match &event {
                        Event::Start(_) => depth += 1,
                        Event::End(_) => depth -= 1,
                        _ => {}
                    }
                    groups.last_mut().unwrap().push(event.into_static());
                }
                for events in groups {
                    let has_title = matches!(
                        events.first(),
                        Some(Event::Start(Tag::Heading {
                            level: HeadingLevel::H1,
                            ..
                        }))
                    );
                    let title = if has_title {
                        heading_label(&events[1..])
                    } else {
                        label.clone()
                    };
                    let title = if title.trim().is_empty() {
                        format!("Chapter {}", drafts.len() + 1)
                    } else {
                        title
                    };
                    drafts.push(Draft {
                        title,
                        events,
                        source,
                        plain: None,
                        has_title,
                    });
                }
            }
            _ => {
                return Err(input_error(format!(
                    "{}: unsupported extension; use .txt, .md or .markdown.",
                    input.display()
                )));
            }
        }
        paths.push(path);
    }
    if drafts.len() > 1000 {
        return Err(input_error("This profile supports at most 1,000 chapters."));
    }
    let mut targets = HashMap::new();
    let mut counts: HashMap<(usize, String), usize> = HashMap::new();
    let mut used = std::collections::HashSet::new();
    for (chapter, draft) in drafts.iter_mut().enumerate() {
        let mut events = Vec::new();
        for (index, event) in draft.events.iter().enumerate() {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    let label = heading_label(&draft.events[index + 1..]);
                    let empty = label.trim().is_empty();
                    let label = if empty { draft.title.clone() } else { label };
                    let base = slug(&label);
                    let count = counts.entry((draft.source, base.clone())).or_default();
                    let mut id;
                    loop {
                        *count += 1;
                        id = if *count == 1 {
                            base.clone()
                        } else {
                            format!("{base}-{count}")
                        };
                        if used.insert((draft.source, id.clone())) {
                            break;
                        }
                    }
                    targets.insert((draft.source, id.clone()), (chapter, id.clone()));
                    events.push(Event::Html(
                        format!("<h{} id=\"{}\">", heading_number(*level), xml_text(&id)).into(),
                    ));
                    if empty {
                        events.push(Event::Text(label.into()));
                    }
                }
                Event::End(TagEnd::Heading(level)) => events.push(Event::Html(
                    format!("</h{}>\n", heading_number(*level)).into(),
                )),
                _ => events.push(event.clone()),
            }
        }
        draft.events = events;
    }
    let mut first = HashMap::new();
    for (index, draft) in drafts.iter().enumerate() {
        first.entry(draft.source).or_insert(index);
    }
    let mut chapters = Vec::new();
    for draft in drafts {
        let body = if let Some(text) = draft.plain {
            let paragraphs = text
                .split("\n\n")
                .filter(|s| !s.trim().is_empty())
                .map(|s| format!("<p>{}</p>\n", xml_text(s).replace('\n', "<br />\n")))
                .collect::<String>();
            format!("<h1>{}</h1>\n{paragraphs}", xml_text(&draft.title))
        } else {
            let mut events = Vec::new();
            for event in draft.events {
                match event {
                    Event::Start(Tag::Link {
                        link_type,
                        dest_url,
                        title,
                        id,
                    }) => {
                        let destination =
                            resolve_link(&dest_url, draft.source, &paths, &first, &targets)?;
                        events.push(Event::Start(Tag::Link {
                            link_type,
                            dest_url: destination.into(),
                            title,
                            id,
                        }));
                    }
                    _ => events.push(event),
                }
            }
            let mut body = if draft.has_title {
                String::new()
            } else {
                format!("<h1>{}</h1>\n", xml_text(&draft.title))
            };
            html::push_html(&mut body, events.into_iter());
            body
        };
        chapters.push(Chapter {
            title: draft.title,
            body,
        });
    }
    Ok(Book {
        metadata,
        chapters,
        cover: None,
    })
}

fn heading_number(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

fn heading_label(events: &[Event<'_>]) -> String {
    let mut title = String::new();
    for event in events {
        match event {
            Event::End(TagEnd::Heading(_)) => break,
            Event::Text(s) | Event::Code(s) => title.push_str(s),
            Event::SoftBreak | Event::HardBreak => title.push(' '),
            _ => {}
        }
    }
    title
}

fn slug(label: &str) -> String {
    let mut result = String::new();
    for c in label.to_lowercase().chars() {
        if c.is_alphanumeric() || c == '_' || c == '-' {
            result.push(c);
        } else if c.is_whitespace() && !result.ends_with('-') {
            result.push('-');
        }
    }
    let result = result.trim_matches('-');
    if result.is_empty() {
        "heading".into()
    } else {
        result.into()
    }
}

fn resolve_link(
    destination: &str,
    source: usize,
    paths: &[PathBuf],
    first: &HashMap<usize, usize>,
    targets: &HashMap<(usize, String), (usize, String)>,
) -> Result<String> {
    let error = || {
        input_error(format!(
            "{}: unresolved or unsupported link {destination:?}; use http(s), mailto, or a supplied source and heading fragment.",
            paths[source].display()
        ))
    };
    if let Ok(url) = Url::parse(destination) {
        return if matches!(url.scheme(), "http" | "https" | "mailto") {
            Ok(destination.into())
        } else {
            Err(error())
        };
    }
    if destination.contains(':')
        || destination.starts_with('/')
        || destination.contains('?')
        || destination.contains('\\')
    {
        return Err(error());
    }
    let (path, fragment) = destination
        .split_once('#')
        .map_or((destination, None), |(p, f)| (p, Some(f)));
    let path = percent_decode_str(path)
        .decode_utf8()
        .map_err(|_| error())?;
    let target_source = if path.is_empty() {
        source
    } else {
        let candidate = normalized_path(&paths[source].parent().unwrap().join(path.as_ref()));
        paths
            .iter()
            .position(|p| *p == candidate)
            .or_else(|| {
                candidate
                    .canonicalize()
                    .ok()
                    .and_then(|actual| paths.iter().position(|p| *p == actual))
            })
            .ok_or_else(error)?
    };
    if let Some(fragment) = fragment.filter(|s| !s.is_empty()) {
        let fragment = percent_decode_str(fragment)
            .decode_utf8()
            .map_err(|_| error())?;
        let (chapter, id) = targets
            .get(&(target_source, fragment.into_owned()))
            .ok_or_else(error)?;
        Ok(format!("{}#{id}", chapter_file(*chapter)))
    } else {
        Ok(chapter_file(first[&target_source]))
    }
}

pub(crate) fn normalized_path(path: &std::path::Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                out.pop();
            }
            _ => out.push(component.as_os_str()),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn book(sources: &[(&str, &str)]) -> Result<Book> {
        let dir = tempfile::tempdir().unwrap();
        let paths = sources
            .iter()
            .map(|(name, text)| {
                let p = dir.path().join(name);
                fs::write(&p, text).unwrap();
                p
            })
            .collect::<Vec<_>>();
        prepare(
            &paths,
            Metadata {
                title: "Book".into(),
                language: "nl".into(),
                author: None,
            },
        )
    }

    #[test]
    fn preserves_preamble_duplicate_slugs_and_cross_chapter_links() {
        let b = book(&[(
            "book.md",
            "Preamble.\n\n# Intro\nFirst. [next](#intro-2)\n\n# Intro\nSecond. [back](#intro)\n",
        )])
        .unwrap();
        assert_eq!(
            b.chapters
                .iter()
                .map(|c| c.title.as_str())
                .collect::<Vec<_>>(),
            ["book", "Intro", "Intro"]
        );
        assert!(b.chapters[0].body.contains("Preamble."));
        assert!(b.chapters[1].body.contains("chapter-0003.xhtml#intro-2"));
        assert!(b.chapters[2].body.contains("chapter-0002.xhtml#intro"));
    }

    #[test]
    fn preserves_literal_text_and_mixed_source_order() {
        let b = book(&[
            ("literal.txt", "# Literal <tag> & café\nline two"),
            (
                "other.md",
                "# Next\n**Bold** and *emphasis*. [literal](literal.txt)",
            ),
        ])
        .unwrap();
        assert!(
            b.chapters[0]
                .body
                .contains("# Literal &lt;tag&gt; &amp; café<br />")
        );
        assert!(b.chapters[1].body.contains("<strong>Bold</strong>"));
        assert!(b.chapters[1].body.contains("chapter-0001.xhtml"));
    }

    #[test]
    fn quoted_heading_does_not_split_an_open_container() {
        let b = book(&[("b.md", "# One\n> # Quoted heading\n> Text.\n\n# Two\nEnd.")]).unwrap();
        assert_eq!(b.chapters.len(), 2);
        assert!(b.chapters[0].body.contains("</blockquote>"));
    }

    #[test]
    fn rejects_features_that_would_otherwise_be_lost() {
        for text in [
            "![alt](image.png)",
            "<script>x</script>",
            "| A | B |\n|---|---|\n| a | b |",
            "Text[^1]\n\n[^1]: note",
            "- [x] task",
        ] {
            assert!(
                book(&[("b.md", text)])
                    .unwrap_err()
                    .message
                    .contains("unsupported")
            );
        }
    }

    #[test]
    fn rejects_missing_destinations_instead_of_broken_navigation() {
        for text in [
            "# One\n[bad](#missing)",
            "[bad](missing.md)",
            "[bad](javascript:alert(1))",
        ] {
            assert!(book(&[("b.md", text)]).is_err());
        }
    }

    #[test]
    fn prevents_slug_collisions_and_preserves_empty_headings() {
        let b = book(&[("b.md", "# Test\n# Test-2\n# Test\n#\n")]).unwrap();
        assert!(b.chapters[2].body.contains("id=\"test-3\""));
        assert!(b.chapters[3].body.contains("Chapter 4"));
    }
}
