use agent_epub::{BuildOptions, ErrorKind, Metadata, build, validate};
use std::{
    fs,
    io::{Read, Write},
    path::Path,
    process::Command,
};
use tempfile::TempDir;
use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};

fn fixture(name: &str, text: &str) -> (TempDir, BuildOptions) {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join(name);
    fs::write(&input, text).unwrap();
    let options = BuildOptions {
        inputs: vec![input],
        output: dir.path().join("book.epub"),
        metadata: Metadata {
            title: "A <book> & café".into(),
            language: "en".into(),
            author: Some("O'Writer".into()),
        },
        split_level: 1,
        overwrite: false,
        epubcheck: None,
    };
    (dir, options)
}

fn resource(path: &Path, name: &str) -> String {
    let mut z = ZipArchive::new(fs::File::open(path).unwrap()).unwrap();
    let mut value = String::new();
    z.by_name(name).unwrap().read_to_string(&mut value).unwrap();
    value
}

fn rewrite(path: &Path, name: &str, change: impl Fn(&str) -> String) {
    let mut z = ZipArchive::new(fs::File::open(path).unwrap()).unwrap();
    let entries = (0..z.len())
        .map(|i| {
            let mut f = z.by_index(i).unwrap();
            let mut bytes = vec![];
            f.read_to_end(&mut bytes).unwrap();
            (f.name().to_string(), f.compression(), bytes)
        })
        .collect::<Vec<_>>();
    drop(z);
    let mut out = ZipWriter::new(fs::File::create(path).unwrap());
    for (key, compression, bytes) in entries {
        out.start_file(
            &key,
            SimpleFileOptions::default().compression_method(compression),
        )
        .unwrap();
        if key == name {
            out.write_all(change(std::str::from_utf8(&bytes).unwrap()).as_bytes())
                .unwrap();
        } else {
            out.write_all(&bytes).unwrap();
        }
    }
    out.finish().unwrap();
}

#[test]
fn tables_preserve_semantics_alignment_and_escaped_cell_content() {
    let (_dir, options) = fixture(
        "b.md",
        "# Report\n\n| Name | Value |\n| :--- | ---: |\n| **Café** & tea | 2 \\| 3 |\n| [link](#report) | `code` |\n",
    );
    build(&options).unwrap();
    let html = resource(&options.output, "EPUB/chapter-0001.xhtml");
    let doc = roxmltree::Document::parse(&html).unwrap();
    let headers = doc
        .descendants()
        .filter(|n| n.has_tag_name("th"))
        .collect::<Vec<_>>();
    assert_eq!(headers.len(), 2);
    assert!(headers.iter().all(|h| h.attribute("scope") == Some("col")));
    assert_eq!(headers[1].attribute("style"), Some("text-align: right"));
    assert!(html.contains("<strong>Café</strong> &amp; tea"));
    assert!(html.contains("2 | 3"));
    assert!(html.contains("chapter-0001.xhtml#report"));
}

#[test]
fn ragged_tables_fail_without_replacing_an_existing_book() {
    let (_dir, mut options) = fixture("b.md", "# Original\nKeep me.");
    build(&options).unwrap();
    let original = fs::read(&options.output).unwrap();
    options.overwrite = true;
    for row in ["| one |", "| one | two | lost |", "one | two | lost"] {
        fs::write(&options.inputs[0], format!("| A | B |\n|---|---|\n{row}\n")).unwrap();
        let error = build(&options).unwrap_err();
        assert_eq!(error.kind, ErrorKind::Input);
        assert!(
            error
                .message
                .contains(":3: table rows must have exactly 2 cells"),
            "{error}"
        );
        assert_eq!(fs::read(&options.output).unwrap(), original);
    }
}

#[test]
fn footnotes_share_definitions_across_chapters_and_return_to_each_reference() {
    let (_dir, mut options) = fixture(
        "b.md",
        "[^SOURCE]: A **shared** note. [next](#two)\n\n# One\nFirst[^source], again[^SOURCE].\n\n# Two\nThird[^source].\n",
    );
    let other = options.output.with_file_name("other.md");
    fs::write(
        &other,
        "# Other\nIndependent[^source].\n\n[^source]: Other source note.",
    )
    .unwrap();
    options.inputs.push(other);
    assert_eq!(build(&options).unwrap().chapters, 3);
    let first = resource(&options.output, "EPUB/chapter-0001.xhtml");
    let second = resource(&options.output, "EPUB/chapter-0002.xhtml");
    let third = resource(&options.output, "EPUB/chapter-0003.xhtml");
    assert_eq!(first.matches("epub:type=\"footnote\"").count(), 1);
    assert!(first.contains("A <strong>shared</strong> note."));
    assert!(first.contains("chapter-0002.xhtml#two"));
    assert!(first.contains("chapter-0001.xhtml#noteref:1:1"));
    assert!(first.contains("chapter-0001.xhtml#noteref:1:2"));
    assert!(first.contains("chapter-0002.xhtml#noteref:1:3"));
    assert!(second.contains("href=\"chapter-0001.xhtml#note:1\""));
    assert!(!second.contains("epub:type=\"footnote\""));
    assert!(third.contains("id=\"note:2\""));
    assert!(third.contains("Other source note."));
}

#[test]
fn footnote_failures_preserve_existing_output_and_unmatched_markers_stay_literal() {
    let (_dir, mut options) = fixture("b.md", "# Original\nKeep me.");
    build(&options).unwrap();
    let original = fs::read(&options.output).unwrap();
    options.overwrite = true;
    for (text, expected) in [
        (
            "Text[^a].\n\n[^a]: First.\n\n[^A]: Duplicate.",
            "duplicate footnote",
        ),
        ("# Book\nText.\n\n[^a]: Unused.", "unused footnote"),
        (
            "Text[^a].\n\n[^a]: Nested[^b].\n\n[^b]: Note.",
            "references inside footnotes",
        ),
        ("Text[^a].\n\n[^a]:", "empty footnote"),
    ] {
        fs::write(&options.inputs[0], text).unwrap();
        let error = build(&options).unwrap_err();
        assert_eq!(error.kind, ErrorKind::Input);
        assert!(error.message.contains(expected), "{error}");
        assert_eq!(fs::read(&options.output).unwrap(), original);
    }
    fs::write(
        &options.inputs[0],
        "# Book\nUnmatched[^missing] and `[^code]` stay literal.",
    )
    .unwrap();
    build(&options).unwrap();
    let text = resource(&options.output, "EPUB/chapter-0001.xhtml");
    assert!(text.contains("Unmatched[^missing]"));
    assert!(text.contains("<code>[^code]</code>"));
}

#[test]
fn catches_misdirected_note_references_and_missing_return_links() {
    for (before, after) in [("#note:1", "#one"), ("#noteref:1:1", "#one")] {
        let (_dir, options) = fixture("b.md", "# One\nClaim[^a].\n\n[^a]: Evidence.");
        build(&options).unwrap();
        rewrite(&options.output, "EPUB/chapter-0001.xhtml", |text| {
            text.replace(before, after)
        });
        assert_eq!(
            validate(&options.output).unwrap_err().kind,
            ErrorKind::Validation
        );
    }
}

#[test]
fn nested_contents_follow_section_hierarchy_and_skip_container_headings() {
    let (_dir, options) = fixture(
        "b.md",
        "# One\n## Repeat\n### Detail\n##### Skipped\n## Repeat\n> ## Quoted\n\n- Item\n\n  ## Listed\n\n# Two\n###### Deep\nClaim[^n].\n\n[^n]: Note.\n\n    ## Note heading\n",
    );
    assert_eq!(build(&options).unwrap().chapters, 2);
    let nav = resource(&options.output, "EPUB/nav.xhtml");
    let doc = roxmltree::Document::parse(&nav).unwrap();
    let links = doc
        .descendants()
        .filter(|n| n.has_tag_name("a"))
        .map(|n| {
            (
                n.attribute("href").unwrap(),
                n.ancestors().filter(|p| p.has_tag_name("ol")).count(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        links,
        [
            ("chapter-0001.xhtml", 1),
            ("chapter-0001.xhtml#repeat", 2),
            ("chapter-0001.xhtml#detail", 3),
            ("chapter-0001.xhtml#skipped", 4),
            ("chapter-0001.xhtml#repeat-2", 2),
            ("chapter-0002.xhtml", 1),
            ("chapter-0002.xhtml#deep", 2),
        ]
    );
    assert!(!nav.contains("Quoted") && !nav.contains("Listed") && !nav.contains("Note heading"));
}

#[test]
fn checks_section_targets_completeness_and_hierarchy_and_accepts_legacy_flat_contents() {
    let (_dir, options) = fixture("b.md", "# One\n## Section\n### Detail\nEnd.");
    build(&options).unwrap();
    let original = fs::read(&options.output).unwrap();
    for (before, after) in [
        ("#section", "#detail"),
        (
            "<ol><li><a href=\"chapter-0001.xhtml#detail\">Detail</a></li></ol>",
            "<li><a href=\"chapter-0001.xhtml#detail\">Detail</a></li>",
        ),
        (
            "<ol><li><a href=\"chapter-0001.xhtml#detail\">Detail</a></li></ol>",
            "",
        ),
    ] {
        fs::write(&options.output, &original).unwrap();
        rewrite(&options.output, "EPUB/nav.xhtml", |text| {
            assert!(text.contains(before));
            text.replace(before, after)
        });
        assert_eq!(
            validate(&options.output).unwrap_err().kind,
            ErrorKind::Validation
        );
    }
    fs::write(&options.output, &original).unwrap();
    rewrite(&options.output, "EPUB/nav.xhtml", |text| {
        let start = text.find("<ol>").unwrap();
        let end = text.rfind("</ol>").unwrap() + 5;
        format!(
            "{}<ol><li><a href=\"chapter-0001.xhtml\">One</a></li></ol>{}",
            &text[..start],
            &text[end..]
        )
        .replace(" data-agent-epub-toc=\"sections\"", "")
    });
    assert_eq!(validate(&options.output).unwrap().chapters, 1);
}

#[test]
fn generates_real_archive_with_consistent_contents_and_navigation() {
    let (_dir, options) = fixture(
        "b.md",
        "Preamble marker.\n\n# Repeat\nFirst marker.\n\n# Repeat\nSecond marker. [back](#repeat)",
    );
    assert_eq!(build(&options).unwrap().chapters, 3);
    assert_eq!(validate(&options.output).unwrap().chapters, 3);
    let nav = resource(&options.output, "EPUB/nav.xhtml");
    let doc = roxmltree::Document::parse(&nav).unwrap();
    let targets = doc
        .descendants()
        .filter(|n| n.tag_name().name() == "a")
        .map(|n| n.attribute("href").unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        targets,
        [
            "chapter-0001.xhtml",
            "chapter-0002.xhtml",
            "chapter-0003.xhtml"
        ]
    );
    assert!(resource(&options.output, "EPUB/chapter-0001.xhtml").contains("Preamble marker."));
    assert!(
        resource(&options.output, "EPUB/chapter-0003.xhtml").contains("chapter-0002.xhtml#repeat")
    );
    assert!(resource(&options.output, "EPUB/package.opf").contains("A &lt;book&gt; &amp; café"));
}

#[test]
fn handles_a_single_plain_text_chapter_without_phantom_neighbors() {
    let (_dir, options) = fixture(
        "literal.txt",
        "<literal> & café\n**not bold**\n\nLast paragraph.",
    );
    assert_eq!(build(&options).unwrap().chapters, 1);
    let html = resource(&options.output, "EPUB/chapter-0001.xhtml");
    assert!(html.contains("&lt;literal&gt; &amp; café"));
    assert!(html.contains("**not bold**"));
    assert!(!html.contains("rel=\"prev\"") && !html.contains("rel=\"next\""));
}

#[test]
fn failures_and_default_conflicts_preserve_existing_output() {
    let (_dir, mut options) = fixture("b.md", "# Good\nA previous reading edition.");
    build(&options).unwrap();
    let original = fs::read(&options.output).unwrap();
    assert_eq!(build(&options).unwrap_err().kind, ErrorKind::Input);
    options.overwrite = true;
    fs::write(&options.inputs[0], "![unsupported](cover.png)").unwrap();
    assert_eq!(build(&options).unwrap_err().kind, ErrorKind::Input);
    assert_eq!(fs::read(&options.output).unwrap(), original);
    fs::write(&options.inputs[0], "# Good\nA new edition.").unwrap();
    options.epubcheck = Some(options.output.with_file_name("missing-validator"));
    assert_eq!(build(&options).unwrap_err().kind, ErrorKind::Validator);
    assert_eq!(fs::read(&options.output).unwrap(), original);
}

#[test]
fn rejects_invalid_metadata_empty_sources_and_invalid_utf8() {
    let (_dir, mut options) = fixture("b.txt", "Content.");
    for tag in ["", "z", "en--US", "en-123456789", "en-abcde-abcde"] {
        options.metadata.language = tag.into();
        assert!(build(&options).is_err(), "accepted invalid language: {tag}");
    }
    options.metadata.language = "en".into();
    options.metadata.title = "Invalid\u{1}".into();
    assert!(build(&options).is_err());
    options.metadata.title = "Valid".into();
    for bytes in [b"  \n".as_slice(), b"bad\0text".as_slice(), &[0xff, 0xfe]] {
        fs::write(&options.inputs[0], bytes).unwrap();
        assert_eq!(build(&options).unwrap_err().kind, ErrorKind::Input);
    }
    assert!(!options.output.exists());
}

#[test]
fn catches_damaged_links_metadata_and_xml_in_completed_packages() {
    for (file, before, after) in [
        ("EPUB/nav.xhtml", "chapter-0001.xhtml", "missing.xhtml"),
        ("EPUB/package.opf", "idref=\"ch-1\"", "idref=\"unknown\""),
        (
            "EPUB/package.opf",
            "dcterms:modified",
            "unrecognized:modified",
        ),
        ("EPUB/chapter-0001.xhtml", "Good", "bad & XML"),
        ("EPUB/chapter-0001.xhtml", "#good", "#missing"),
    ] {
        let (_dir, options) = fixture("b.md", "# Good\n[here](#good)");
        build(&options).unwrap();
        rewrite(&options.output, file, |text| {
            assert!(text.contains(before));
            text.replace(before, after)
        });
        assert_eq!(
            validate(&options.output).unwrap_err().kind,
            ErrorKind::Validation
        );
    }
}

#[test]
fn reports_cli_success_and_distinct_failure_categories() {
    let (_dir, options) = fixture("b.md", "# Good\nContent.");
    let exe = env!("CARGO_BIN_EXE_agent-epub");
    let result = Command::new(exe)
        .args(["build", "--input"])
        .arg(&options.inputs[0])
        .arg("--output")
        .arg(&options.output)
        .args(["--title", "Book", "--language", "en"])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(String::from_utf8_lossy(&result.stdout).contains("EPUBCheck not run"));
    assert!(
        Command::new(exe)
            .arg("validate")
            .arg(&options.output)
            .output()
            .unwrap()
            .status
            .success()
    );
    assert_eq!(
        Command::new(exe)
            .args(["build"])
            .output()
            .unwrap()
            .status
            .code(),
        Some(2)
    );
    fs::write(&options.output, "not an archive").unwrap();
    assert_eq!(
        Command::new(exe)
            .arg("validate")
            .arg(&options.output)
            .output()
            .unwrap()
            .status
            .code(),
        Some(4)
    );
    assert_eq!(
        Command::new(exe)
            .arg("validate")
            .arg(options.output.with_file_name("missing.epub"))
            .output()
            .unwrap()
            .status
            .code(),
        Some(3)
    );
}

#[test]
fn protects_sources_reached_through_hard_links() {
    let (_dir, mut options) = fixture("source.txt", "Original source.");
    std::fs::hard_link(&options.inputs[0], &options.output).unwrap();
    let original = std::fs::read(&options.inputs[0]).unwrap();
    options.overwrite = true;
    let error = agent_epub::build(&options).unwrap_err();
    assert_eq!(error.kind, ErrorKind::Input);
    assert!(error.message.contains("input source"));
    assert_eq!(std::fs::read(&options.inputs[0]).unwrap(), original);
    assert_eq!(std::fs::read(&options.output).unwrap(), original);
}

#[cfg(unix)]
#[test]
fn protects_sources_reached_through_symbolic_and_hard_links() {
    let (_dir, mut options) = fixture("b.txt", "Source must survive.");
    options.overwrite = true;
    std::os::unix::fs::symlink(&options.inputs[0], &options.output).unwrap();
    assert_eq!(build(&options).unwrap_err().kind, ErrorKind::Input);
    fs::remove_file(&options.output).unwrap();
    fs::hard_link(&options.inputs[0], &options.output).unwrap();
    assert_eq!(build(&options).unwrap_err().kind, ErrorKind::Input);
    assert_eq!(
        fs::read_to_string(&options.inputs[0]).unwrap(),
        "Source must survive."
    );
}

#[cfg(unix)]
#[test]
fn failed_external_validation_is_atomic_and_cleans_temporary_output() {
    use std::os::unix::fs::PermissionsExt;
    let (dir, mut options) = fixture("b.md", "# Valid\nContent.");
    fs::write(&options.output, "previous edition").unwrap();
    let executable = dir.path().join("reject-validator");
    fs::write(
        &executable,
        "#!/bin/sh\nprintf 'deliberate validator rejection' >&2\nexit 1\n",
    )
    .unwrap();
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o755)).unwrap();
    options.overwrite = true;
    options.epubcheck = Some(executable);
    let error = build(&options).unwrap_err();
    assert_eq!(error.kind, ErrorKind::Validator);
    assert!(error.message.contains("deliberate validator rejection"));
    assert_eq!(
        fs::read_to_string(&options.output).unwrap(),
        "previous edition"
    );
    assert!(!fs::read_dir(dir.path()).unwrap().any(|e| {
        e.unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".agent-epub-")
    }));
}

#[test]
#[ignore = "Requires EPUBCHECK pointing to an official EPUBCheck executable wrapper"]
fn independent_epubcheck_corpus() {
    let executable =
        std::env::var_os("EPUBCHECK").expect("Set EPUBCHECK to the validator wrapper path");
    let cases = [
        ("report.md", include_str!("../examples/research-report.md")),
        (
            "sections.md",
            "# One\n## Repeat\n### Detail\n##### Skipped\n## Repeat\n> ## Quoted\n> Text.\n\n# Two\n###### Deep\nEnd.",
        ),
        (
            "notes.md",
            "[^n]: A **note** with a [chapter link](#two) and [web source](https://example.org).\n\n# One\nFirst[^n], again[^n].\n\n# Two\nThird[^n].\n",
        ),
        (
            "table.md",
            "# Report\n\n| Item | Value |\n| :--- | ---: |\n| **Café** & tea | 2 \\| 3 |\n| [source](https://example.org) | `code` |\n",
        ),
        (
            "plain.txt",
            "Literal <XML> & café.\nSecond line.\n\nAnother paragraph.",
        ),
        (
            "format.md",
            "# Formatting\n**Bold** *italic* `code`\n\n> Quote\n\n3. Three\n4. Four\n\n- Item\n  - Nested\n\nHard break  \nnext line\n\n---\n\n```rust\nlet x = 1 < 2;\n```\n\n[web](https://example.org/?a=1&b=2) &copy;",
        ),
        (
            "headings.md",
            "Preamble.\n\n# Same\n# Same-2\n# Same\n#\nEnd.",
        ),
        (
            "unicode.md",
            "# Café 日本語\nnaïef • Ελληνικά. [self](#café-日本語)",
        ),
        ("links.md", "# One\n[first](#two)\n\n# Two\n[back](#one)"),
        (
            "quote.md",
            "# One\n> # Nested heading\n> Preserve this quotation.\n\n# Two\nEnd.",
        ),
    ];
    for (name, text) in cases {
        let (_dir, mut options) = fixture(name, text);
        options.epubcheck = Some(executable.clone().into());
        assert!(
            build(&options)
                .unwrap_or_else(|e| panic!("{name}: {e}"))
                .independent_validation
        );
    }
    for level in 1..=6 {
        let (_dir, mut options) = fixture("split.md", include_str!("../examples/title-h2.md"));
        options.split_level = level;
        options.epubcheck = Some(executable.clone().into());
        assert!(
            build(&options)
                .unwrap_or_else(|e| panic!("H{level}: {e}"))
                .independent_validation
        );
    }
    let output = tempfile::tempdir().unwrap();
    let options = BuildOptions {
        inputs: vec![
            "examples/introduction.txt".into(),
            "examples/guide.md".into(),
        ],
        output: output.path().join("mixed.epub"),
        metadata: Metadata {
            title: "Mixed sources".into(),
            language: "en".into(),
            author: None,
        },
        split_level: 1,
        overwrite: false,
        epubcheck: Some(executable.into()),
    };
    assert_eq!(build(&options).unwrap().chapters, 5);
}
