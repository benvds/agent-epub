use std::{collections::HashMap, path::Path};

use pulldown_cmark::{Event, Tag, TagEnd};
use unicase::UniCase;

use crate::{Error, ErrorKind, Result, source::chapter_file};

#[derive(Default)]
pub(crate) struct Notes {
    definitions: Vec<Note>,
    lookup: HashMap<(usize, UniCase<String>), usize>,
    numbered: usize,
}

pub(crate) struct Note {
    pub source: usize,
    pub chapter: usize,
    pub number: usize,
    pub events: Vec<Event<'static>>,
    label: String,
    references: Vec<(usize, String)>,
}

fn invalid(path: &Path, message: impl std::fmt::Display) -> Error {
    Error::new(ErrorKind::Input, format!("{}: {message}", path.display()))
}

impl Notes {
    pub fn extract(
        &mut self,
        source: usize,
        events: Vec<Event<'static>>,
        path: &Path,
    ) -> Result<Vec<Event<'static>>> {
        let mut output = Vec::new();
        let mut current: Option<Note> = None;
        let mut depth = 0;
        for event in events {
            match event {
                Event::Start(Tag::FootnoteDefinition(label)) => {
                    if current.is_some() || depth != 0 {
                        return Err(invalid(
                            path,
                            "footnote definitions must be outside other containers.",
                        ));
                    }
                    let key = (source, UniCase::new(label.to_string()));
                    if self.lookup.insert(key, self.definitions.len()).is_some() {
                        return Err(invalid(
                            path,
                            format!("duplicate footnote definition {label:?}."),
                        ));
                    }
                    current = Some(Note {
                        source,
                        chapter: 0,
                        number: 0,
                        events: Vec::new(),
                        label: label.to_string(),
                        references: Vec::new(),
                    });
                }
                Event::End(TagEnd::FootnoteDefinition) => {
                    self.definitions
                        .push(current.take().expect("paired footnote events"));
                }
                Event::FootnoteReference(_) if current.is_some() => {
                    return Err(invalid(
                        path,
                        "references inside footnotes are unsupported.",
                    ));
                }
                _ => {
                    match &event {
                        Event::Start(_) => depth += 1,
                        Event::End(_) => depth -= 1,
                        _ => {}
                    }
                    if let Some(note) = &mut current {
                        note.events.push(event);
                    } else {
                        output.push(event);
                    }
                }
            }
        }
        Ok(output)
    }

    pub fn references(
        &mut self,
        source: usize,
        chapter: usize,
        events: &mut [Event<'static>],
        path: &Path,
    ) -> Result<()> {
        for event in events {
            if let Event::FootnoteReference(label) = event {
                let index = self
                    .lookup
                    .get(&(source, UniCase::new(label.to_string())))
                    .ok_or_else(|| invalid(path, format!("undefined footnote {label:?}.")))?;
                let note = &mut self.definitions[*index];
                if note.references.is_empty() {
                    self.numbered += 1;
                    note.number = self.numbered;
                    note.chapter = chapter;
                }
                let id = format!("noteref:{}:{}", note.number, note.references.len() + 1);
                note.references.push((chapter, id.clone()));
                *event = Event::Html(format!(
                    "<sup><a epub:type=\"noteref\" role=\"doc-noteref\" id=\"{id}\" href=\"{}#note:{}\">{}</a></sup>",
                    chapter_file(note.chapter), note.number, note.number
                ).into());
            }
        }
        Ok(())
    }

    pub fn finish(mut self, paths: &[std::path::PathBuf]) -> Result<Vec<Note>> {
        for note in &self.definitions {
            if note.references.is_empty() {
                return Err(invalid(
                    &paths[note.source],
                    format!(
                        "unused footnote definition {:?}; add a reference or remove it.",
                        note.label
                    ),
                ));
            }
            if note.events.is_empty() {
                return Err(invalid(
                    &paths[note.source],
                    format!("empty footnote definition {:?}.", note.label),
                ));
            }
        }
        self.definitions.sort_by_key(|note| note.number);
        Ok(self.definitions)
    }
}

impl Note {
    pub fn wrap(&self, body: &str) -> String {
        let backlinks = self.references.iter().enumerate().map(|(index, (chapter, id))| {
            format!("<a href=\"{}#{id}\" aria-label=\"Back to reference {} for note {}\">Return {}</a>", chapter_file(*chapter), index + 1, self.number, index + 1)
        }).collect::<Vec<_>>().join(" · ");
        format!(
            "<aside class=\"footnote\" epub:type=\"footnote\" role=\"doc-footnote\" id=\"note:{}\"><p><strong>Note {}</strong></p>{body}<p>{backlinks}</p></aside>\n",
            self.number, self.number
        )
    }
}
