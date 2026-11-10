use agent_epub::{
    BuildOptions, CoverOptions, ErrorKind, Metadata,
    agent::{self, RequestOptions},
    build_with_cover, validate,
};
use image::{DynamicImage, ImageFormat, RgbImage};
use std::{
    fs,
    io::{Cursor, Read, Write},
    path::Path,
};
use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};

fn image_bytes(format: ImageFormat, width: u32, height: u32) -> Vec<u8> {
    let mut buffer = Cursor::new(Vec::new());
    let pixels = RgbImage::from_fn(width, height, |x, y| {
        image::Rgb([(x % 256) as u8, (y % 256) as u8, 120])
    });
    DynamicImage::ImageRgb8(pixels)
        .write_to(&mut buffer, format)
        .unwrap();
    buffer.into_inner()
}

fn options(dir: &Path) -> BuildOptions {
    let input = dir.join("chapter.md");
    fs::write(&input, "# One\nText.\n\n# Two\n[back](#one)").unwrap();
    BuildOptions {
        inputs: vec![input],
        output: dir.join("book.epub"),
        metadata: Metadata {
            title: "Cover & book".into(),
            language: "en".into(),
            author: None,
        },
        split_level: 1,
        overwrite: false,
        epubcheck: None,
    }
}

fn resource(path: &Path, name: &str) -> Vec<u8> {
    let mut zip = ZipArchive::new(fs::File::open(path).unwrap()).unwrap();
    let mut bytes = vec![];
    zip.by_name(name).unwrap().read_to_end(&mut bytes).unwrap();
    bytes
}

fn rewrite(path: &Path, target: &str, change: impl Fn(&str) -> String) {
    let mut zip = ZipArchive::new(fs::File::open(path).unwrap()).unwrap();
    let entries = (0..zip.len())
        .map(|i| {
            let mut file = zip.by_index(i).unwrap();
            let mut bytes = vec![];
            file.read_to_end(&mut bytes).unwrap();
            (file.name().to_string(), file.compression(), bytes)
        })
        .collect::<Vec<_>>();
    drop(zip);
    let mut zip = ZipWriter::new(fs::File::create(path).unwrap());
    for (name, compression, bytes) in entries {
        zip.start_file(
            &name,
            SimpleFileOptions::default().compression_method(compression),
        )
        .unwrap();
        if name == target {
            zip.write_all(change(std::str::from_utf8(&bytes).unwrap()).as_bytes())
                .unwrap();
        } else {
            zip.write_all(&bytes).unwrap();
        }
    }
    zip.finish().unwrap();
}

#[test]
fn identifies_actual_image_type_preserves_bytes_and_keeps_chapter_navigation() {
    for (format, filename, media) in [
        (ImageFormat::Png, "cover.png", "image/png"),
        (ImageFormat::Jpeg, "cover.jpg", "image/jpeg"),
    ] {
        let dir = tempfile::tempdir().unwrap();
        let options = options(dir.path());
        let bytes = image_bytes(format, 32, 48);
        let path = dir.path().join("intentionally-misnamed.data");
        fs::write(&path, &bytes).unwrap();
        let summary = build_with_cover(
            &options,
            Some(&CoverOptions {
                path,
                alt: Some("An <image> & cover".into()),
            }),
        )
        .unwrap();
        assert!(summary.cover);
        assert_eq!(summary.chapters, 2);
        assert_eq!(
            resource(&options.output, &format!("EPUB/{filename}")),
            bytes
        );
        let package = String::from_utf8(resource(&options.output, "EPUB/package.opf")).unwrap();
        let package = roxmltree::Document::parse(&package).unwrap();
        let cover = package
            .descendants()
            .find(|n| n.attribute("properties") == Some("cover-image"))
            .unwrap();
        assert_eq!(cover.attribute("media-type"), Some(media));
        let spine = package
            .descendants()
            .filter(|n| n.tag_name().name() == "itemref")
            .map(|n| n.attribute("idref").unwrap())
            .collect::<Vec<_>>();
        assert_eq!(spine, ["cover-page", "nav", "ch-1", "ch-2"]);
        let page = String::from_utf8(resource(&options.output, "EPUB/cover.xhtml")).unwrap();
        let page = roxmltree::Document::parse(&page).unwrap();
        assert_eq!(
            page.descendants()
                .find(|n| n.tag_name().name() == "img")
                .unwrap()
                .attribute("alt"),
            Some("An <image> & cover")
        );
        assert_eq!(validate(&options.output).unwrap().chapters, 2);
    }
}

#[test]
fn direct_and_structured_file_requests_produce_equivalent_reading_resources() {
    let dir = tempfile::tempdir().unwrap();
    let options = options(dir.path());
    fs::write(
        dir.path().join("cover.png"),
        image_bytes(ImageFormat::Png, 32, 48),
    )
    .unwrap();
    build_with_cover(
        &options,
        Some(&CoverOptions {
            path: dir.path().join("cover.png"),
            alt: None,
        }),
    )
    .unwrap();
    let request = serde_json::json!({"schema_version": 1, "metadata": {"title": options.metadata.title, "language": "en"}, "inputs": [{"path": "chapter.md"}], "cover": {"path": "cover.png"}});
    let target = dir.path().join("request.epub");
    let summary = agent::build_request(
        &request.to_string(),
        &RequestOptions {
            output: target.clone(),
            base_dir: Some(dir.path().into()),
            request_path: None,
            overwrite: false,
            epubcheck: None,
        },
    )
    .unwrap();
    assert!(summary.cover);
    for name in [
        "EPUB/cover.png",
        "EPUB/cover.xhtml",
        "EPUB/nav.xhtml",
        "EPUB/chapter-0001.xhtml",
        "EPUB/chapter-0002.xhtml",
    ] {
        assert_eq!(
            resource(&options.output, name),
            resource(&target, name),
            "{name}"
        );
    }
    let mut streamed_without_base = RequestOptions {
        output: dir.path().join("bad.epub"),
        base_dir: None,
        request_path: None,
        overwrite: false,
        epubcheck: None,
    };
    assert!(
        agent::build_request(&request.to_string(), &streamed_without_base)
            .unwrap_err()
            .message
            .contains("--base-dir")
    );
    streamed_without_base.base_dir = Some(dir.path().into());
    assert!(agent::build_request(&request.to_string(), &streamed_without_base).is_ok());
}

#[test]
fn invalid_images_and_alt_text_preserve_existing_output() {
    let dir = tempfile::tempdir().unwrap();
    let mut options = options(dir.path());
    fs::write(&options.output, "previous edition").unwrap();
    options.overwrite = true;
    let cover = dir.path().join("bad.png");
    let valid = image_bytes(ImageFormat::Png, 32, 48);
    let cases = [
        vec![],
        b"not a PNG".to_vec(),
        b"GIF89a\x01\x00\x01\x00".to_vec(),
        valid[..valid.len() / 2].to_vec(),
        vec![0; 10 * 1024 * 1024 + 1],
        image_bytes(ImageFormat::Png, 8001, 1),
    ];
    for bytes in cases {
        fs::write(&cover, bytes).unwrap();
        assert_eq!(
            build_with_cover(
                &options,
                Some(&CoverOptions {
                    path: cover.clone(),
                    alt: None
                })
            )
            .unwrap_err()
            .kind,
            ErrorKind::Input
        );
        assert_eq!(
            fs::read_to_string(&options.output).unwrap(),
            "previous edition"
        );
    }
    fs::write(&cover, valid).unwrap();
    for alt in ["", "bad\u{1}"] {
        assert!(
            build_with_cover(
                &options,
                Some(&CoverOptions {
                    path: cover.clone(),
                    alt: Some(alt.into())
                })
            )
            .is_err()
        );
    }
}

#[test]
fn detects_damaged_cover_metadata_and_missing_alternative_text() {
    for (file, from, to) in [
        ("EPUB/package.opf", "image/png", "image/jpeg"),
        ("EPUB/package.opf", "properties=\"cover-image\"", ""),
        ("EPUB/cover.xhtml", "Cover of Cover &amp; book", ""),
    ] {
        let dir = tempfile::tempdir().unwrap();
        let options = options(dir.path());
        let cover = dir.path().join("cover.png");
        fs::write(&cover, image_bytes(ImageFormat::Png, 32, 48)).unwrap();
        build_with_cover(
            &options,
            Some(&CoverOptions {
                path: cover,
                alt: None,
            }),
        )
        .unwrap();
        rewrite(&options.output, file, |s| {
            assert!(s.contains(from));
            s.replace(from, to)
        });
        assert_eq!(
            validate(&options.output).unwrap_err().kind,
            ErrorKind::Validation
        );
    }
}

#[test]
fn accepts_covers_through_both_cli_modes_and_reports_them_in_json() {
    use std::process::Command;
    let dir = tempfile::tempdir().unwrap();
    let options = options(dir.path());
    let cover = dir.path().join("cover with spaces.png");
    fs::write(&cover, image_bytes(ImageFormat::Png, 32, 48)).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_agent-epub"))
        .args(["build", "--input"])
        .arg(&options.inputs[0])
        .arg("--output")
        .arg(&options.output)
        .args(["--title", "CLI book", "--language", "en", "--cover"])
        .arg(&cover)
        .args(["--cover-alt", "A supplied cover", "--json"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap()["cover"],
        true
    );
    let request = serde_json::json!({"schema_version": 1, "metadata": {"title": "Request book", "language": "en"}, "inputs": [{"path": "chapter.md"}], "cover": {"path": "cover with spaces.png"}});
    let request_file = dir.path().join("request.json");
    fs::write(&request_file, request.to_string()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_agent-epub"))
        .current_dir(std::env::temp_dir())
        .args(["build", "--request"])
        .arg(&request_file)
        .arg("--output")
        .arg(dir.path().join("request.epub"))
        .arg("--json")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap()["cover"],
        true
    );
}

#[test]
#[ignore = "Requires EPUBCHECK pointing to the official EPUBCheck executable wrapper"]
fn independent_cover_corpus() {
    let executable = std::env::var_os("EPUBCHECK").expect("Set EPUBCHECK to a validator wrapper");
    for format in [ImageFormat::Png, ImageFormat::Jpeg] {
        let dir = tempfile::tempdir().unwrap();
        let mut options = options(dir.path());
        options.epubcheck = Some(executable.clone().into());
        let path = dir.path().join("cover.data");
        fs::write(&path, image_bytes(format, 512, 768)).unwrap();
        assert!(
            build_with_cover(
                &options,
                Some(&CoverOptions {
                    path: path.clone(),
                    alt: None
                })
            )
            .unwrap()
            .independent_validation
        );
        fs::write(
            &options.inputs[0],
            include_str!("../examples/research-report.md"),
        )
        .unwrap();
        options.output = dir.path().join("report-with-cover.epub");
        assert!(
            build_with_cover(&options, Some(&CoverOptions { path, alt: None }))
                .unwrap()
                .independent_validation
        );
    }
}
