use std::{fmt, path::PathBuf};

mod source;

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
}

impl Error {
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone)]
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
    pub overwrite: bool,
    pub epubcheck: Option<PathBuf>,
}

#[derive(Debug)]
pub struct Summary {
    pub chapters: usize,
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
