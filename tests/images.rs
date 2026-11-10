use agent_epub::{
    BuildOptions, CoverOptions, ErrorKind, Metadata,
    agent::{self, RequestOptions},
    build, build_with_cover, validate,
};
use image::{DynamicImage, ImageFormat, RgbImage};
use serde_json::{Value, json};
use std::{
    fs,
    io::{Cursor, Read, Write},
    path::Path,
    process::{Command, Stdio},
};
use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};

fn pixels(format: ImageFormat) -> Vec<u8> {
    let mut bytes = Cursor::new(Vec::new());
    DynamicImage::ImageRgb8(RgbImage::from_fn(48, 32, |x, y| {
        image::Rgb([(x * 5) as u8, (y * 7) as u8, 120])
    }))
    .write_to(&mut bytes, format)
    .unwrap();
    bytes.into_inner()
}
fn options(dir: &Path, text: &str) -> BuildOptions {
    let source = dir.join("source.md");
    fs::write(&source, text).unwrap();
    BuildOptions {
        inputs: vec![source],
        output: dir.join("book.epub"),
        metadata: Metadata {
            title: "Illustrated book".into(),
            language: "en".into(),
            author: None,
        },
        split_level: 2,
        overwrite: false,
        epubcheck: None,
    }
}
fn resource(path: &Path, name: &str) -> Vec<u8> {
    let mut zip = ZipArchive::new(fs::File::open(path).unwrap()).unwrap();
    let mut bytes = Vec::new();
    zip.by_name(name).unwrap().read_to_end(&mut bytes).unwrap();
    bytes
}
fn rewrite(path: &Path, target: &str, bytes: Option<Vec<u8>>) {
    let mut zip = ZipArchive::new(fs::File::open(path).unwrap()).unwrap();
    let entries = (0..zip.len())
        .map(|i| {
            let mut entry = zip.by_index(i).unwrap();
            let mut bytes = Vec::new();
            entry.read_to_end(&mut bytes).unwrap();
            (entry.name().to_string(), entry.compression(), bytes)
        })
        .collect::<Vec<_>>();
    drop(zip);
    let mut zip = ZipWriter::new(fs::File::create(path).unwrap());
    for (name, compression, original) in entries {
        let data = if name == target {
            if let Some(bytes) = &bytes {
                bytes.clone()
            } else {
                continue;
            }
        } else {
            original
        };
        zip.start_file(
            name,
            SimpleFileOptions::default().compression_method(compression),
        )
        .unwrap();
        zip.write_all(&data).unwrap();
    }
    zip.finish().unwrap();
}

#[test]
fn packages_original_images_reuses_paths_and_keeps_notes_tables_and_heading_ranks() {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir(dir.path().join("assets")).unwrap();
    let png = pixels(ImageFormat::Png);
    let jpeg = pixels(ImageFormat::Jpeg);
    fs::write(dir.path().join("assets/café image.data"), &png).unwrap();
    fs::write(dir.path().join("photo.png"), &jpeg).unwrap();
    let text = "# Illustrated book\nPreamble.\n\n## Diagram\n![A **blue** & `red` diagram](assets/caf%C3%A9%20image.data \"A <title>\")\n\n| Item | Figure |\n|---|---|\n| Image | ![Repeated](assets/café%20image.data) |\n\nA note[^n].\n\n## Photo\n![A photo](photo.png)\n\n[Back](#diagram).\n\n[^n]: Another ![note image](assets/café%20image.data).\n";
    let options = options(dir.path(), text);
    let summary = build_with_cover(
        &options,
        Some(&CoverOptions {
            path: dir.path().join("photo.png"),
            alt: None,
        }),
    )
    .unwrap();
    assert_eq!(summary.images, 2);
    assert!(summary.cover);
    assert_eq!(summary.chapters, 3);
    assert_eq!(resource(&options.output, "EPUB/images/image-0001.png"), png);
    assert_eq!(
        resource(&options.output, "EPUB/images/image-0002.jpg"),
        jpeg
    );
    let chapter = String::from_utf8(resource(&options.output, "EPUB/chapter-0002.xhtml")).unwrap();
    assert_eq!(chapter.matches("src=\"images/image-0001.png\"").count(), 3);
    assert!(chapter.contains("alt=\"A blue &amp; red diagram\""));
    assert!(chapter.contains("title=\"A &lt;title&gt;\""));
    assert!(chapter.contains("<h2 id=\"diagram\""));
    assert!(chapter.contains("<th scope=\"col\""));
    assert!(chapter.contains("<table class=\"with-images\">"));
    let later = String::from_utf8(resource(&options.output, "EPUB/chapter-0003.xhtml")).unwrap();
    assert!(later.contains("href=\"chapter-0002.xhtml#diagram\""));
    assert!(
        String::from_utf8(resource(&options.output, "EPUB/style.css"))
            .unwrap()
            .contains("img { max-width: 100%; height: auto; }")
    );
    let package = String::from_utf8(resource(&options.output, "EPUB/package.opf")).unwrap();
    assert!(package.contains("href=\"images/image-0002.jpg\" media-type=\"image/jpeg\""));
    assert_eq!(validate(&options.output).unwrap().images, 2);
}

#[test]
fn direct_file_request_and_streamed_inline_images_are_equivalent() {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir(dir.path().join("chapters")).unwrap();
    fs::create_dir(dir.path().join("assets")).unwrap();
    fs::write(
        dir.path().join("assets/image.png"),
        pixels(ImageFormat::Png),
    )
    .unwrap();
    let text = "# Book\nPreamble.\n\n## Figure\n![Description](../assets/image.png)\n\n## Again\n![](../assets/image.png \"decorative\")\n";
    let mut options = options(&dir.path().join("chapters"), text);
    build(&options).unwrap();
    let original = options.output.clone();
    let file_request = json!({"schema_version":1,"metadata":{"title":"Illustrated book","language":"en"},"split_level":2,"inputs":[{"path":"chapters/source.md"}]});
    let request_path = dir.path().join("request.json");
    fs::write(&request_path, file_request.to_string()).unwrap();
    let file_output = dir.path().join("requested.epub");
    let out = Command::new(env!("CARGO_BIN_EXE_agent-epub"))
        .args(["build", "--request"])
        .arg(&request_path)
        .arg("--output")
        .arg(&file_output)
        .arg("--json")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );
    let result: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(result["images"], 1);
    let inline = json!({"schema_version":1,"metadata":{"title":"Illustrated book","language":"en"},"split_level":2,"inputs":[{"name":"source.md","format":"markdown","content":text}]});
    options.output = dir.path().join("inline.epub");
    let mut child = Command::new(env!("CARGO_BIN_EXE_agent-epub"))
        .args(["build", "--request", "-", "--base-dir"])
        .arg(dir.path().join("chapters"))
        .arg("--output")
        .arg(&options.output)
        .arg("--json")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(inline.to_string().as_bytes())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );
    for name in [
        "EPUB/nav.xhtml",
        "EPUB/chapter-0001.xhtml",
        "EPUB/chapter-0002.xhtml",
        "EPUB/chapter-0003.xhtml",
        "EPUB/images/image-0001.png",
    ] {
        let expected = resource(&original, name);
        assert_eq!(resource(&file_output, name), expected);
        assert_eq!(resource(&options.output, name), expected);
    }
    let body = String::from_utf8(resource(&original, "EPUB/chapter-0003.xhtml")).unwrap();
    assert!(body.contains("alt=\"\" role=\"presentation\""));
    assert!(!body.contains("title=\"decorative\""));
    let no_base = agent::build_request(
        &inline.to_string(),
        &RequestOptions {
            output: dir.path().join("unbased.epub"),
            base_dir: None,
            request_path: None,
            overwrite: false,
            epubcheck: None,
        },
    )
    .unwrap_err();
    assert_eq!(no_base.issue_code, "image_base_required");
    assert_eq!(no_base.line, Some(5));
}

#[test]
fn invalid_images_have_localized_diagnostics_and_preserve_previous_output() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("good.png"), pixels(ImageFormat::Png)).unwrap();
    fs::write(dir.path().join("bad.png"), b"not an image").unwrap();
    let cases = [
        (
            "![Description](missing.png)",
            "image_not_found",
            ErrorKind::Io,
        ),
        ("![Description](bad.png)", "invalid_image", ErrorKind::Input),
        ("![](good.png)", "image_alt_required", ErrorKind::Input),
        (
            "![Description](good.png) [Full image](good.png)",
            "unsupported_link",
            ErrorKind::Input,
        ),
        (
            "![   ](good.png \"a title\")",
            "image_alt_required",
            ErrorKind::Input,
        ),
        (
            "![Description](https://example.org/image.png)",
            "unsupported_image_source",
            ErrorKind::Input,
        ),
        (
            "![Description](data:image/png;base64,AAAA)",
            "unsupported_image_source",
            ErrorKind::Input,
        ),
        (
            "![Description](file:///tmp/image.png)",
            "unsupported_image_source",
            ErrorKind::Input,
        ),
        (
            "![Description](/tmp/image.png)",
            "unsupported_image_source",
            ErrorKind::Input,
        ),
        (
            "![Description](good.png#crop)",
            "unsupported_image_source",
            ErrorKind::Input,
        ),
        (
            "![Description](good.png?size=small)",
            "unsupported_image_source",
            ErrorKind::Input,
        ),
        (
            "![Description](%2Ftmp%2Fimage.png)",
            "unsupported_image_source",
            ErrorKind::Input,
        ),
        ("![Description](%ZZ.png)", "image_not_found", ErrorKind::Io),
        (
            "![outer ![inner](good.png)](good.png)",
            "invalid_image_alt",
            ErrorKind::Input,
        ),
    ];
    for (image, issue, kind) in cases {
        let mut options = options(dir.path(), &format!("# Book\r\n\r\nCafé {image}\r\n"));
        options.overwrite = true;
        fs::write(&options.output, b"old book").unwrap();
        let error = build(&options).unwrap_err();
        assert_eq!(error.issue_code, issue, "{image}: {error}");
        assert_eq!(error.kind, kind);
        assert_eq!(error.line, Some(3));
        assert_eq!(
            error.column,
            Some(if issue == "unsupported_link" { 31 } else { 6 })
        );
        assert!(error.source.unwrap().ends_with("source.md"));
        assert!(!error.remedy.is_empty());
        assert_eq!(fs::read(&options.output).unwrap(), b"old book");
    }
}

#[test]
fn corrupted_image_packages_fail_structural_validation() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("good.png"), pixels(ImageFormat::Png)).unwrap();
    let mut options = options(
        dir.path(),
        "# Book\n![Description](good.png)\n\n![](good.png \"decorative\")",
    );
    options.overwrite = true;
    for (target, from, to) in [
        (
            "EPUB/package.opf",
            "media-type=\"image/png\"",
            "media-type=\"image/jpeg\"",
        ),
        ("EPUB/chapter-0001.xhtml", "alt=\"Description\"", ""),
        ("EPUB/chapter-0001.xhtml", " role=\"presentation\"", ""),
        (
            "EPUB/chapter-0001.xhtml",
            "src=\"images/image-0001.png\"",
            "src=\"https://example.org/image.png\"",
        ),
        (
            "EPUB/chapter-0001.xhtml",
            "src=\"images/image-0001.png\"",
            "src=\"style.css\"",
        ),
    ] {
        build(&options).unwrap();
        let bytes = String::from_utf8(resource(&options.output, target))
            .unwrap()
            .replace(from, to)
            .into_bytes();
        rewrite(&options.output, target, Some(bytes));
        assert_eq!(
            validate(&options.output).unwrap_err().kind,
            ErrorKind::Validation
        );
    }
    for bytes in [Some(b"corrupt image".to_vec()), None] {
        build(&options).unwrap();
        rewrite(&options.output, "EPUB/images/image-0001.png", bytes);
        assert_eq!(
            validate(&options.output).unwrap_err().kind,
            ErrorKind::Validation
        );
    }
}

#[test]
fn image_inputs_cannot_be_overwritten_by_the_book() {
    let dir = tempfile::tempdir().unwrap();
    let bytes = pixels(ImageFormat::Png);
    fs::write(dir.path().join("book.epub"), &bytes).unwrap();
    let mut options = options(dir.path(), "# Book\n![Description](book.epub)");
    options.overwrite = true;
    assert_eq!(build(&options).unwrap_err().kind, ErrorKind::Input);
    assert_eq!(fs::read(&options.output).unwrap(), bytes);
    #[cfg(unix)]
    {
        fs::hard_link(&options.output, dir.path().join("hard.png")).unwrap();
        std::os::unix::fs::symlink(&options.output, dir.path().join("symbolic.png")).unwrap();
        for image in ["hard.png", "symbolic.png"] {
            fs::write(
                &options.inputs[0],
                format!("# Book\n![Description]({image})"),
            )
            .unwrap();
            assert_eq!(build(&options).unwrap_err().kind, ErrorKind::Input);
            assert_eq!(fs::read(&options.output).unwrap(), bytes);
        }
    }
}

#[test]
#[ignore = "Requires EPUBCHECK pointing to the official EPUBCheck executable wrapper"]
fn independent_inline_image_corpus() {
    let validator = std::env::var_os("EPUBCHECK").expect("Set EPUBCHECK");
    for format in [ImageFormat::Png, ImageFormat::Jpeg] {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("figure.data"), pixels(format)).unwrap();
        let mut options = options(
            dir.path(),
            "# Illustrated book\nPreamble.\n\n## Figure\n![A useful diagram](figure.data)\n\n![](figure.data \"decorative\")\n\n| A | B |\n|---|---|\n| Picture | ![Cell image](figure.data) |\n\nReference[^n].\n\n## More\n![Again](figure.data)\n\n[^n]: Note ![Note image](figure.data).\n",
        );
        options.epubcheck = Some(validator.clone().into());
        let summary = build_with_cover(
            &options,
            Some(&CoverOptions {
                path: dir.path().join("figure.data"),
                alt: None,
            }),
        )
        .unwrap();
        assert!(summary.independent_validation);
        assert_eq!(summary.images, 1);
    }
    // Validate the shipped illustrated fixture too, including its real links.
    let dir = tempfile::tempdir().unwrap();
    let summary = agent::build_request(
        include_str!("../examples/illustrated-request.json"),
        &RequestOptions {
            output: dir.path().join("illustrated.epub"),
            request_path: Some(
                Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/illustrated-request.json"),
            ),
            base_dir: None,
            overwrite: false,
            epubcheck: Some(validator.into()),
        },
    )
    .unwrap();
    assert_eq!(summary.images, 3);
    assert!(summary.independent_validation);
}

fn crc(bytes: &[u8]) -> u32 {
    let mut value = !0u32;
    for &byte in bytes {
        value ^= u32::from(byte);
        for _ in 0..8 {
            value = if value & 1 != 0 {
                (value >> 1) ^ 0xedb88320
            } else {
                value >> 1
            };
        }
    }
    !value
}

#[test]
fn byte_dimension_animation_and_book_wide_limits_fail_before_publication() {
    let dir = tempfile::tempdir().unwrap();
    let png = pixels(ImageFormat::Png);
    let mut over_side = png.clone();
    over_side[16..20].copy_from_slice(&8001u32.to_be_bytes());
    let sum = crc(&over_side[12..29]);
    over_side[29..33].copy_from_slice(&sum.to_be_bytes());
    let mut over_pixels = png.clone();
    over_pixels[16..20].copy_from_slice(&4001u32.to_be_bytes());
    over_pixels[20..24].copy_from_slice(&4000u32.to_be_bytes());
    let sum = crc(&over_pixels[12..29]);
    over_pixels[29..33].copy_from_slice(&sum.to_be_bytes());
    let mut animated = png.clone();
    let mut chunk = 8u32.to_be_bytes().to_vec();
    chunk.extend_from_slice(b"acTL");
    chunk.extend_from_slice(&1u32.to_be_bytes());
    chunk.extend_from_slice(&0u32.to_be_bytes());
    chunk.extend_from_slice(&crc(&chunk[4..]).to_be_bytes());
    animated.splice(33..33, chunk);
    for bytes in [
        vec![0; 10 * 1024 * 1024 + 1],
        over_side,
        over_pixels,
        animated,
        b"GIF89a\x01\x00\x01\x00".to_vec(),
    ] {
        fs::write(dir.path().join("bad.data"), bytes).unwrap();
        let mut options = options(dir.path(), "# Book\n![Description](bad.data)");
        options.overwrite = true;
        fs::write(&options.output, b"old book").unwrap();
        let error = build(&options).unwrap_err();
        assert_eq!(error.issue_code, "invalid_image", "{error}");
        assert_eq!(fs::read(&options.output).unwrap(), b"old book");
    }
    for (label, count, padded) in [("count", 129, false), ("total", 7, true)] {
        let base = dir.path().join(label);
        fs::create_dir(&base).unwrap();
        let mut text = "# Book\n".to_string();
        let mut bytes = png.clone();
        // Trailing bytes let this rejected-input fixture exercise the encoded
        // byte budget without allocating large decoded pixel buffers.
        if padded {
            bytes.resize(10 * 1024 * 1024, 0);
        }
        for index in 0..count {
            fs::write(base.join(format!("image-{index}.png")), &bytes).unwrap();
            text.push_str(&format!("![Image {index}](image-{index}.png)\n"));
        }
        let mut options = options(&base, &text);
        options.overwrite = true;
        fs::write(&options.output, b"old book").unwrap();
        let error = build(&options).unwrap_err();
        assert_eq!(error.issue_code, "image_limit_exceeded", "{error}");
        assert_eq!(fs::read(&options.output).unwrap(), b"old book");
    }
}

#[test]
fn literal_markers_do_not_load_files_and_image_heading_uses_alternative_text() {
    let dir = tempfile::tempdir().unwrap();
    let options = options(
        dir.path(),
        "# Literal\n`![code](missing.png)`\n\n```md\n![fenced](missing.png)\n```",
    );
    assert_eq!(build(&options).unwrap().images, 0);
    fs::write(dir.path().join("good.png"), pixels(ImageFormat::Png)).unwrap();
    fs::write(
        &options.inputs[0],
        "# ![Café diagram](good.png)\n\n[Self](#café-diagram)",
    )
    .unwrap();
    let mut options = options;
    options.overwrite = true;
    assert_eq!(build(&options).unwrap().images, 1);
    let body = String::from_utf8(resource(&options.output, "EPUB/chapter-0001.xhtml")).unwrap();
    assert!(body.contains("id=\"café-diagram\""));
    assert!(body.contains("alt=\"Café diagram\""));
}

#[test]
fn multiple_sources_with_same_image_names_keep_distinct_assets_and_reuse_aliases() {
    let dir = tempfile::tempdir().unwrap();
    let a = dir.path().join("a");
    let b = dir.path().join("b");
    fs::create_dir(&a).unwrap();
    fs::create_dir(&b).unwrap();
    let png = pixels(ImageFormat::Png);
    let jpeg = pixels(ImageFormat::Jpeg);
    fs::write(a.join("diagram.png"), &png).unwrap();
    fs::write(b.join("diagram.png"), &jpeg).unwrap();
    let mut options = options(&a, "# One\n![First](diagram.png)\n![Again](./diagram.png)");
    fs::write(b.join("source.md"), "# Two\n![Second](diagram.png)").unwrap();
    options.inputs.push(b.join("source.md"));
    options.output = dir.path().join("book.epub");
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(a.join("diagram.png"), a.join("alias.png")).unwrap();
        fs::write(
            &options.inputs[0],
            "# One\n![First](diagram.png)\n![Alias](alias.png)",
        )
        .unwrap();
    }
    assert_eq!(build(&options).unwrap().images, 2);
    assert_eq!(resource(&options.output, "EPUB/images/image-0001.png"), png);
    assert_eq!(
        resource(&options.output, "EPUB/images/image-0002.jpg"),
        jpeg
    );
}

#[test]
fn inline_content_in_a_request_file_uses_its_directory_and_missing_files_return_json() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("figure.png"), pixels(ImageFormat::Png)).unwrap();
    let request = json!({"schema_version":1,"metadata":{"title":"Book","language":"en"},"inputs":[{"name":"book.md","format":"markdown","content":"# Book\n![Description](figure.png)"}]});
    let path = dir.path().join("request.json");
    fs::write(&path, request.to_string()).unwrap();
    let output = dir.path().join("book.epub");
    let result = Command::new(env!("CARGO_BIN_EXE_agent-epub"))
        .args(["build", "--request"])
        .arg(&path)
        .arg("--output")
        .arg(&output)
        .arg("--json")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stdout)
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&result.stdout).unwrap()["images"],
        1
    );
    fs::remove_file(dir.path().join("figure.png")).unwrap();
    let before = fs::read(&output).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_agent-epub"))
        .args(["build", "--request"])
        .arg(&path)
        .arg("--output")
        .arg(&output)
        .args(["--json", "--overwrite"])
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(3));
    let result: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(result["errors"][0]["issue_code"], "image_not_found");
    assert_eq!(result["errors"][0]["line"], 2);
    assert_eq!(result["output"], Value::Null);
    assert_eq!(fs::read(output).unwrap(), before);
}
