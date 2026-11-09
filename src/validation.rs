use crate::{Error, ErrorKind, Result, Summary};
use percent_encoding::percent_decode_str;
use roxmltree::Document;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
    path::Path,
};
use url::Url;
use zip::{CompressionMethod, ZipArchive};

const OPF: &str = "http://www.idpf.org/2007/opf";
const XHTML: &str = "http://www.w3.org/1999/xhtml";
const DC: &str = "http://purl.org/dc/elements/1.1/";
const EPUB: &str = "http://www.idpf.org/2007/ops";
fn failure(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::Validation, message)
}

pub fn validate(path: &Path) -> Result<Summary> {
    let file = File::open(path)
        .map_err(|e| Error::new(ErrorKind::Io, format!("{}: {e}", path.display())))?;
    let mut archive = ZipArchive::new(file).map_err(|e| failure(format!("Invalid ZIP: {e}")))?;
    if archive.is_empty() || archive.len() > 4096 {
        return Err(failure("Expected 1 to 4,096 archive entries."));
    }
    let mut files = HashMap::new();
    let mut total = 0u64;
    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .map_err(|e| failure(e.to_string()))?;
        let name = entry.name().to_string();
        if index == 0 && (name != "mimetype" || entry.compression() != CompressionMethod::Stored) {
            return Err(failure(
                "mimetype must be the first, uncompressed ZIP entry.",
            ));
        }
        if name.starts_with('/')
            || name.contains('\\')
            || name.split('/').any(|p| matches!(p, ".." | "." | ""))
            || files.contains_key(&name)
        {
            return Err(failure(format!(
                "Unsafe or duplicate archive entry: {name}"
            )));
        }
        total = total
            .checked_add(entry.size())
            .ok_or_else(|| failure("Archive size overflow."))?;
        if total > 256 * 1024 * 1024 {
            return Err(failure("Profile validation limit: 256 MiB uncompressed."));
        }
        let mut bytes = Vec::new();
        entry
            .by_ref()
            .take(256 * 1024 * 1024 + 1)
            .read_to_end(&mut bytes)
            .map_err(|e| failure(format!("{name}: ZIP integrity failure: {e}")))?;
        if bytes.len() as u64 != entry.size() {
            return Err(failure(format!("{name}: invalid declared size.")));
        }
        files.insert(name, bytes);
    }
    if files.get("mimetype").map(Vec::as_slice) != Some(b"application/epub+zip".as_slice()) {
        return Err(failure("Incorrect mimetype content."));
    }
    let mut texts = HashMap::new();
    for (name, bytes) in &files {
        if name.ends_with(".xml") || name.ends_with(".opf") || name.ends_with(".xhtml") {
            texts.insert(
                name.as_str(),
                std::str::from_utf8(bytes)
                    .map_err(|e| failure(format!("{name}: invalid UTF-8: {e}")))?,
            );
        }
    }
    let documents = texts
        .iter()
        .map(|(name, text)| {
            Ok((
                *name,
                Document::parse(text).map_err(|e| failure(format!("{name}: invalid XML: {e}")))?,
            ))
        })
        .collect::<Result<HashMap<_, _>>>()?;
    let container = documents
        .get("META-INF/container.xml")
        .ok_or_else(|| failure("Missing container document."))?;
    const CONTAINER: &str = "urn:oasis:names:tc:opendocument:xmlns:container";
    if !container
        .root_element()
        .has_tag_name((CONTAINER, "container"))
    {
        return Err(failure("Incorrect container namespace."));
    }
    let roots = container
        .descendants()
        .filter(|n| n.has_tag_name((CONTAINER, "rootfile")))
        .collect::<Vec<_>>();
    if roots.len() != 1 {
        return Err(failure("This profile requires one rootfile."));
    }
    let package_path = roots[0]
        .attribute("full-path")
        .ok_or_else(|| failure("Missing package path."))?;
    let package = documents
        .get(package_path)
        .ok_or_else(|| failure("Missing package document."))?;
    let root = package.root_element();
    if !root.has_tag_name((OPF, "package")) || root.attribute("version") != Some("3.0") {
        return Err(failure("Expected an EPUB 3 package namespace and version."));
    }
    let metadata = root
        .children()
        .find(|n| n.has_tag_name((OPF, "metadata")))
        .ok_or_else(|| failure("Missing metadata."))?;
    for field in ["title", "language", "identifier"] {
        if !metadata
            .children()
            .any(|n| n.has_tag_name((DC, field)) && n.text().is_some_and(|s| !s.trim().is_empty()))
        {
            return Err(failure(format!("Missing publication {field}.")));
        }
    }
    let identifier = root
        .attribute("unique-identifier")
        .ok_or_else(|| failure("Missing unique-identifier."))?;
    if !metadata
        .children()
        .any(|n| n.has_tag_name((DC, "identifier")) && n.attribute("id") == Some(identifier))
    {
        return Err(failure(
            "unique-identifier does not identify publication metadata.",
        ));
    }
    if !metadata.children().any(|n| {
        n.has_tag_name((OPF, "meta"))
            && n.attribute("property") == Some("dcterms:modified")
            && n.text().is_some_and(|s| s.len() == 20 && s.ends_with('Z'))
    }) {
        return Err(failure("Missing UTC modification metadata."));
    }
    let manifest = root
        .children()
        .find(|n| n.has_tag_name((OPF, "manifest")))
        .ok_or_else(|| failure("Missing manifest."))?;
    let mut items = HashMap::new();
    let mut resources = HashSet::new();
    let mut navs = Vec::new();
    for item in manifest
        .children()
        .filter(|n| n.has_tag_name((OPF, "item")))
    {
        let id = item
            .attribute("id")
            .ok_or_else(|| failure("Manifest item missing id."))?;
        let href = item
            .attribute("href")
            .ok_or_else(|| failure("Manifest item missing href."))?;
        let media = item
            .attribute("media-type")
            .ok_or_else(|| failure("Manifest item missing media-type."))?;
        let (target, fragment) = local_target(package_path, href)?;
        if fragment.is_some()
            || !files.contains_key(&target)
            || !resources.insert(target.clone())
            || items.insert(id, (target.clone(), media)).is_some()
        {
            return Err(failure(format!(
                "Missing or duplicate manifest resource: {href}"
            )));
        }
        if item
            .attribute("properties")
            .unwrap_or("")
            .split_whitespace()
            .any(|p| p == "nav")
        {
            if media != "application/xhtml+xml" {
                return Err(failure("Navigation resource must be XHTML."));
            }
            navs.push(target);
        }
    }
    if navs.len() != 1 {
        return Err(failure("Expected exactly one navigation resource."));
    }
    let spine = root
        .children()
        .find(|n| n.has_tag_name((OPF, "spine")))
        .ok_or_else(|| failure("Missing spine."))?;
    let mut order = Vec::new();
    for item in spine
        .children()
        .filter(|n| n.has_tag_name((OPF, "itemref")))
    {
        let id = item
            .attribute("idref")
            .ok_or_else(|| failure("Spine item missing idref."))?;
        let (target, media) = items
            .get(id)
            .ok_or_else(|| failure(format!("Unknown spine id: {id}")))?;
        if *media != "application/xhtml+xml" || order.contains(target) {
            return Err(failure("Spine must contain unique XHTML resources."));
        }
        order.push(target.clone());
    }
    if order.first() != Some(&navs[0]) || order.len() < 2 {
        return Err(failure(
            "This profile requires visible contents followed by at least one chapter.",
        ));
    }
    for (name, document) in &documents {
        if !name.ends_with(".xhtml") {
            continue;
        }
        if !document.root_element().has_tag_name((XHTML, "html"))
            || document.root_element().attribute("lang").is_none()
        {
            return Err(failure(format!(
                "{name}: missing XHTML namespace or language."
            )));
        }
        let mut ids = HashSet::new();
        for element in document.descendants().filter(|n| n.is_element()) {
            if let Some(id) = element.attribute("id")
                && !ids.insert(id)
            {
                return Err(failure(format!("{name}: duplicate id {id}")));
            }
            for attr in ["href", "src"] {
                let Some(href) = element.attribute(attr) else {
                    continue;
                };
                if let Ok(url) = Url::parse(href) {
                    if matches!(url.scheme(), "http" | "https" | "mailto") {
                        continue;
                    }
                    return Err(failure(format!(
                        "{name}: unsupported external reference {href}"
                    )));
                }
                let (target, fragment) = local_target(name, href)?;
                if !files.contains_key(&target) {
                    return Err(failure(format!("{name}: missing resource {href}")));
                }
                if let Some(fragment) = fragment {
                    let destination = documents
                        .get(target.as_str())
                        .ok_or_else(|| failure(format!("{name}: fragment target is not XML.")))?;
                    if !destination
                        .descendants()
                        .any(|n| n.attribute("id") == Some(fragment.as_str()))
                    {
                        return Err(failure(format!("{name}: missing fragment {href}")));
                    }
                }
            }
        }
    }
    for (target, media) in items.values() {
        if *media == "application/xhtml+xml"
            && !documents
                .get(target.as_str())
                .is_some_and(|d| d.root_element().has_tag_name((XHTML, "html")))
        {
            return Err(failure(format!(
                "{target}: manifest XHTML type does not match content."
            )));
        }
    }
    let nav = documents
        .get(navs[0].as_str())
        .ok_or_else(|| failure("Navigation document is missing XML."))?;
    let toc = nav
        .descendants()
        .find(|n| {
            n.has_tag_name((XHTML, "nav"))
                && n.attribute((EPUB, "type"))
                    .is_some_and(|s| s.split_whitespace().any(|p| p == "toc"))
        })
        .ok_or_else(|| failure("Missing EPUB table of contents."))?;
    let links = toc
        .descendants()
        .filter(|n| n.has_tag_name((XHTML, "a")))
        .map(|n| {
            let href = n
                .attribute("href")
                .ok_or_else(|| failure("Contents entry missing href."))?;
            Ok(local_target(&navs[0], href)?.0)
        })
        .collect::<Result<Vec<_>>>()?;
    if links != order[1..] {
        return Err(failure(
            "Contents order does not match chapter reading order.",
        ));
    }
    for (index, chapter) in order[1..].iter().enumerate() {
        let document = &documents[chapter.as_str()];
        for (relation, expected) in [
            ("prev", index.checked_sub(1).map(|i| &order[i + 1])),
            ("next", order.get(index + 2)),
        ] {
            let links = document
                .descendants()
                .filter(|n| n.has_tag_name((XHTML, "a")) && n.attribute("rel") == Some(relation))
                .collect::<Vec<_>>();
            if links.len() != usize::from(expected.is_some()) {
                return Err(failure(format!(
                    "{chapter}: incorrect {relation} chapter navigation."
                )));
            }
            if let Some(expected) = expected
                && local_target(chapter, links[0].attribute("href").unwrap_or(""))?.0 != *expected
            {
                return Err(failure(format!(
                    "{chapter}: incorrect {relation} destination."
                )));
            }
        }
        if !document.descendants().any(|n| {
            n.has_tag_name((XHTML, "a"))
                && n.attribute("href")
                    .is_some_and(|h| local_target(chapter, h).is_ok_and(|(p, _)| p == navs[0]))
        }) {
            return Err(failure(format!(
                "{chapter}: missing return-to-contents link."
            )));
        }
    }
    Ok(Summary {
        chapters: order.len() - 1,
        independent_validation: false,
    })
}

fn local_target(from: &str, href: &str) -> Result<(String, Option<String>)> {
    if href.starts_with('/') || href.contains('\\') || href.contains('?') {
        return Err(failure(format!("Unsupported local reference: {href}")));
    }
    let base =
        Url::parse(&format!("https://epub.invalid/{from}")).map_err(|e| failure(e.to_string()))?;
    let url = base.join(href).map_err(|e| failure(e.to_string()))?;
    if url.host_str() != Some("epub.invalid") || url.scheme() != "https" {
        return Err(failure(format!("Expected local reference: {href}")));
    }
    let path = percent_decode_str(url.path())
        .decode_utf8()
        .map_err(|e| failure(e.to_string()))?
        .trim_start_matches('/')
        .to_string();
    if path.split('/').any(|p| matches!(p, ".." | "." | "")) {
        return Err(failure(format!("Unsafe local reference: {href}")));
    }
    let fragment = url
        .fragment()
        .filter(|s| !s.is_empty())
        .map(|s| {
            percent_decode_str(s)
                .decode_utf8()
                .map(|s| s.into_owned())
                .map_err(|e| failure(e.to_string()))
        })
        .transpose()?;
    Ok((path, fragment))
}
