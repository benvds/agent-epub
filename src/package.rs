use crate::{
    BuildOptions, CoverOptions, Error, ErrorKind, Result, Summary,
    source::{Book, chapter_file, prepare},
    validate, xml_text,
};
use std::{fs, io::Write, path::Path, process::Command};
use tempfile::Builder;
use time::OffsetDateTime;
use uuid::Uuid;
use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

pub fn build(options: &BuildOptions) -> Result<Summary> {
    build_with_cover(options, None)
}

pub fn build_with_cover(options: &BuildOptions, cover: Option<&CoverOptions>) -> Result<Summary> {
    let mut book = prepare(
        &options.inputs,
        options.metadata.clone(),
        options.split_level,
    )?;
    let mut options = options.clone();
    if let Some(cover) = cover {
        book.cover = Some(crate::cover::load(cover, &book.metadata.title)?);
        options.inputs.push(cover.path.clone());
    }
    publish(&options, &book)
}

pub(crate) fn publish(options: &BuildOptions, book: &Book) -> Result<Summary> {
    let parent = options
        .output
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let parent = parent.canonicalize().map_err(|e| {
        Error::new(
            ErrorKind::Io,
            format!("Output directory {}: {e}", parent.display()),
        )
    })?;
    let name = options.output.file_name().ok_or_else(|| {
        Error::new(
            ErrorKind::Input,
            "Supply an output filename ending in .epub.",
        )
    })?;
    if options.output.extension().and_then(|s| s.to_str()) != Some("epub") {
        return Err(Error::new(
            ErrorKind::Input,
            "Output filename must end in .epub.",
        ));
    }
    let output = parent.join(name);
    let existing = match fs::symlink_metadata(&output) {
        Ok(meta) => Some(meta),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => {
            return Err(Error::new(
                ErrorKind::Io,
                format!("{}: {e}", output.display()),
            ));
        }
    };
    for input in options
        .inputs
        .iter()
        .chain(book.images.iter().map(|image| &image.source))
    {
        if let (Ok(a), Ok(b)) = (input.canonicalize(), output.canonicalize())
            && (a == b || same_file(input, &output))
        {
            return Err(Error::new(
                ErrorKind::Input,
                "Output must not replace an input source.",
            ));
        }
    }
    if existing.as_ref().is_some_and(|m| m.is_dir()) {
        return Err(Error::new(
            ErrorKind::Input,
            "Output is a directory; supply a file path.",
        ));
    }
    if existing.is_some() && !options.overwrite {
        return Err(Error::new(
            ErrorKind::Input,
            format!(
                "{} already exists; use --overwrite to replace it.",
                output.display()
            ),
        ));
    }
    let mut temporary = Builder::new()
        .prefix(".agent-epub-")
        .suffix(".epub")
        .tempfile_in(&parent)
        .map_err(|e| Error::new(ErrorKind::Io, format!("Cannot create output: {e}")))?;
    write_package(temporary.as_file_mut(), book)?;
    temporary
        .as_file_mut()
        .sync_all()
        .map_err(|e| Error::new(ErrorKind::Io, e.to_string()))?;
    let mut summary = validate(temporary.path())?;
    if let Some(executable) = &options.epubcheck {
        run_epubcheck(executable, temporary.path())?;
        summary.independent_validation = true;
    }
    let published = if options.overwrite {
        temporary.persist(&output)
    } else {
        temporary.persist_noclobber(&output)
    };
    published.map_err(|e| {
        Error::new(
            ErrorKind::Io,
            format!("Cannot publish {}: {}", output.display(), e.error),
        )
    })?;
    Ok(summary)
}

#[cfg(unix)]
fn same_file(a: &Path, b: &Path) -> bool {
    use std::os::unix::fs::MetadataExt;
    match (fs::metadata(a), fs::metadata(b)) {
        (Ok(a), Ok(b)) => a.dev() == b.dev() && a.ino() == b.ino(),
        _ => false,
    }
}
#[cfg(windows)]
fn same_file(a: &Path, b: &Path) -> bool {
    same_file::is_same_file(a, b).unwrap_or(false)
}
#[cfg(not(any(unix, windows)))]
fn same_file(_: &Path, _: &Path) -> bool {
    false
}

pub(crate) fn run_epubcheck(executable: &Path, file: &Path) -> Result<()> {
    let result = Command::new(executable).arg(file).output().map_err(|e| Error::new(ErrorKind::Validator, format!("Cannot run EPUBCheck executable {}: {e}. Supply an executable wrapper if using a JAR.", executable.display())))?;
    if !result.status.success() {
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
        return Err(Error::new(
            ErrorKind::Validator,
            format!(
                "EPUBCheck did not pass ({}): {}",
                result.status,
                text.chars().take(4000).collect::<String>()
            ),
        ));
    }
    eprint!(
        "{}{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    Ok(())
}

fn page(title: &str, language: &str, body: &str) -> String {
    let language = xml_text(language);
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:epub=\"http://www.idpf.org/2007/ops\" lang=\"{language}\" xml:lang=\"{language}\"><head><title>{}</title><link rel=\"stylesheet\" type=\"text/css\" href=\"style.css\" /></head><body>{body}</body></html>\n",
        xml_text(title)
    )
}

fn write_package(file: &mut fs::File, book: &Book) -> Result<()> {
    let mut archive = ZipWriter::new(file);
    let stored = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .unix_permissions(0o644);
    let deflated = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o644);
    let mut put = |name: &str, contents: &str, options| -> Result<()> {
        archive
            .start_file(name, options)
            .map_err(|e| Error::new(ErrorKind::Io, e.to_string()))?;
        archive
            .write_all(contents.as_bytes())
            .map_err(|e| Error::new(ErrorKind::Io, e.to_string()))
    };
    put("mimetype", "application/epub+zip", stored)?;
    put(
        "META-INF/container.xml",
        "<?xml version=\"1.0\"?><container xmlns=\"urn:oasis:names:tc:opendocument:xmlns:container\" version=\"1.0\"><rootfiles><rootfile full-path=\"EPUB/package.opf\" media-type=\"application/oebps-package+xml\" /></rootfiles></container>",
        deflated,
    )?;
    put(
        "EPUB/style.css",
        "body { margin: 5%; line-height: 1.5; }\nh1, h2, h3 { line-height: 1.2; }\np { margin: 0.7em 0; }\npre { white-space: pre-wrap; word-wrap: break-word; }\na, code { word-wrap: break-word; }\nnav { margin: 1em 0; }\ntable { border-collapse: collapse; max-width: 100%; }\ntable.with-images { width: 100%; table-layout: fixed; }\ntable.with-images img { max-width: 100% !important; }\nth, td { border: 1px solid; padding: 0.3em; overflow-wrap: anywhere; }\n.footnote { margin: 1em 0; }\nimg { max-width: 100%; height: auto; }\n.cover { text-align: center; }\n.cover img { max-width: 100%; max-height: 90vh; width: auto; height: auto; }\n",
        deflated,
    )?;
    if let Some(cover) = &book.cover {
        let body = format!(
            "<div epub:type=\"cover\" class=\"cover\"><img src=\"{}\" alt=\"{}\" /></div>",
            cover.filename,
            xml_text(&cover.alt)
        );
        put(
            "EPUB/cover.xhtml",
            &page(
                &format!("{}: Cover", book.metadata.title),
                &book.metadata.language,
                &body,
            ),
            deflated,
        )?;
    }
    let mut contents = String::from(
        "<nav epub:type=\"toc\" id=\"toc\" data-agent-epub-toc=\"sections\"><h1>Contents</h1><ol>",
    );
    for (index, chapter) in book.chapters.iter().enumerate() {
        contents.push_str(&format!(
            "<li><a href=\"{}\">{}</a>",
            chapter_file(index),
            xml_text(&chapter.title)
        ));
        section_contents(
            &mut contents,
            &chapter.sections,
            &mut 0,
            1,
            &chapter_file(index),
        );
        contents.push_str("</li>\n");
    }
    contents.push_str("</ol></nav>");
    put(
        "EPUB/nav.xhtml",
        &page(
            &format!("{}: Contents", book.metadata.title),
            &book.metadata.language,
            &contents,
        ),
        deflated,
    )?;
    for (index, chapter) in book.chapters.iter().enumerate() {
        let mut body = chapter.body.clone();
        body.push_str(
            "<nav aria-label=\"Chapter navigation\"><p><a href=\"nav.xhtml\">Contents</a>",
        );
        if index > 0 {
            body.push_str(&format!(
                " · <a rel=\"prev\" href=\"{}\">Previous chapter</a>",
                chapter_file(index - 1)
            ));
        }
        if index + 1 < book.chapters.len() {
            body.push_str(&format!(
                " · <a rel=\"next\" href=\"{}\">Next chapter</a>",
                chapter_file(index + 1)
            ));
        }
        body.push_str("</p></nav>");
        put(
            &format!("EPUB/{}", chapter_file(index)),
            &page(&chapter.title, &book.metadata.language, &body),
            deflated,
        )?;
    }
    let identifier = format!("urn:uuid:{}", Uuid::new_v4());
    let now = OffsetDateTime::now_utc();
    let modified = format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        now.year(),
        now.month() as u8,
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    );
    let mut package = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><package xmlns=\"http://www.idpf.org/2007/opf\" version=\"3.0\" unique-identifier=\"book-id\"><metadata xmlns:dc=\"http://purl.org/dc/elements/1.1/\"><dc:identifier id=\"book-id\">{identifier}</dc:identifier><dc:title>{}</dc:title><dc:language>{}</dc:language><meta property=\"dcterms:modified\">{modified}</meta>",
        xml_text(&book.metadata.title),
        xml_text(&book.metadata.language)
    );
    if let Some(author) = &book.metadata.author {
        package.push_str(&format!("<dc:creator>{}</dc:creator>", xml_text(author)));
    }
    package.push_str("</metadata><manifest><item id=\"nav\" href=\"nav.xhtml\" media-type=\"application/xhtml+xml\" properties=\"nav\" /><item id=\"style\" href=\"style.css\" media-type=\"text/css\" />");
    if let Some(cover) = &book.cover {
        package.push_str(&format!("<item id=\"cover-image\" href=\"{}\" media-type=\"{}\" properties=\"cover-image\" /><item id=\"cover-page\" href=\"cover.xhtml\" media-type=\"application/xhtml+xml\" />", cover.filename, cover.media_type));
    }
    for (index, image) in book.images.iter().enumerate() {
        package.push_str(&format!(
            "<item id=\"image-{}\" href=\"{}\" media-type=\"{}\" />",
            index + 1,
            image.filename,
            image.media_type
        ));
    }
    for index in 0..book.chapters.len() {
        package.push_str(&format!(
            "<item id=\"ch-{}\" href=\"{}\" media-type=\"application/xhtml+xml\" />",
            index + 1,
            chapter_file(index)
        ));
    }
    package.push_str("</manifest><spine>");
    if book.cover.is_some() {
        package.push_str("<itemref idref=\"cover-page\" />");
    }
    package.push_str("<itemref idref=\"nav\" />");
    for index in 0..book.chapters.len() {
        package.push_str(&format!("<itemref idref=\"ch-{}\" />", index + 1));
    }
    package.push_str("</spine></package>");
    put("EPUB/package.opf", &package, deflated)?;
    if let Some(cover) = &book.cover {
        archive
            .start_file(format!("EPUB/{}", cover.filename), stored)
            .map_err(|e| Error::new(ErrorKind::Io, e.to_string()))?;
        archive
            .write_all(&cover.bytes)
            .map_err(|e| Error::new(ErrorKind::Io, e.to_string()))?;
    }
    for image in &book.images {
        archive
            .start_file(format!("EPUB/{}", image.filename), stored)
            .map_err(|e| Error::new(ErrorKind::Io, e.to_string()))?;
        archive
            .write_all(&image.bytes)
            .map_err(|e| Error::new(ErrorKind::Io, e.to_string()))?;
    }
    archive
        .finish()
        .map_err(|e| Error::new(ErrorKind::Io, e.to_string()))?;
    Ok(())
}

fn section_contents(
    contents: &mut String,
    sections: &[crate::source::Section],
    cursor: &mut usize,
    parent_level: u8,
    file: &str,
) {
    if sections
        .get(*cursor)
        .is_none_or(|s| s.level <= parent_level)
    {
        return;
    }
    contents.push_str("<ol>");
    while let Some(section) = sections.get(*cursor).filter(|s| s.level > parent_level) {
        contents.push_str(&format!(
            "<li><a href=\"{file}#{}\">{}</a>",
            xml_text(&section.id),
            xml_text(&section.title)
        ));
        *cursor += 1;
        section_contents(contents, sections, cursor, section.level, file);
        contents.push_str("</li>");
    }
    contents.push_str("</ol>");
}
