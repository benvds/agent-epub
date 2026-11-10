use serde_json::{Value, json};
use std::{
    fs,
    io::Write,
    process::{Command, Stdio},
};

fn run_request(
    request: &Value,
    base: Option<&std::path::Path>,
    output: &std::path::Path,
) -> (i32, Value) {
    let mut command = Command::new(env!("CARGO_BIN_EXE_agent-epub"));
    command
        .args(["build", "--request", "-", "--json", "--output"])
        .arg(output);
    if let Some(base) = base {
        command.arg("--base-dir").arg(base);
    }
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(request.to_string().as_bytes())
        .unwrap();
    let result = child.wait_with_output().unwrap();
    (
        result.status.code().unwrap(),
        serde_json::from_slice(&result.stdout)
            .unwrap_or_else(|e| panic!("{e}: {}", String::from_utf8_lossy(&result.stdout))),
    )
}

fn request(inputs: Value) -> Value {
    json!({"schema_version": 1, "metadata": {"title": "Agent book", "language": "en"}, "inputs": inputs})
}

#[test]
fn milestone_three_content_is_equivalent_in_direct_and_streamed_inline_requests() {
    use std::io::Read;
    let dir = tempfile::tempdir().unwrap();
    let content = include_str!("../examples/research-report.md");
    let input = dir.path().join("report.md");
    fs::write(&input, content).unwrap();
    let direct = dir.path().join("direct.epub");
    let inline = dir.path().join("inline.epub");
    let output = Command::new(env!("CARGO_BIN_EXE_agent-epub"))
        .args(["build", "--input"])
        .arg(input)
        .arg("--output")
        .arg(&direct)
        .args(["--title", "Agent book", "--language", "en", "--json"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    let book = request(json!([{"name": "report.md", "format": "markdown", "content": content}]));
    let (code, result) = run_request(&book, None, &inline);
    assert_eq!(code, 0, "{result}");
    assert_eq!(result["chapters"], 2);
    let mut a = zip::ZipArchive::new(fs::File::open(direct).unwrap()).unwrap();
    let mut b = zip::ZipArchive::new(fs::File::open(inline).unwrap()).unwrap();
    for name in [
        "EPUB/nav.xhtml",
        "EPUB/chapter-0001.xhtml",
        "EPUB/chapter-0002.xhtml",
    ] {
        let mut left = String::new();
        let mut right = String::new();
        a.by_name(name).unwrap().read_to_string(&mut left).unwrap();
        b.by_name(name).unwrap().read_to_string(&mut right).unwrap();
        assert_eq!(left, right, "{name}");
    }
}

#[test]
fn inline_requests_work_without_creating_source_files() {
    let dir = tempfile::tempdir().unwrap();
    let book = request(json!([
        {"name": "one.md", "format": "markdown", "content": "# One\n[next](two.md#two)"},
        {"name": "two.md", "format": "markdown", "content": "# Two\n[back](one.md#one)"},
        {"format": "text", "content": "Literal <text> & Unicode café."}
    ]));
    let output = dir.path().join("book.epub");
    let (code, result) = run_request(&book, Some(dir.path()), &output);
    assert_eq!(code, 0, "{result}");
    assert_eq!(result["chapters"], 3);
    assert_eq!(result["checks"]["structural_profile"], "passed");
    assert_eq!(result["checks"]["epubcheck"], "not_run");
    assert!(std::path::Path::new(result["output"].as_str().unwrap()).is_absolute());
    assert!(!dir.path().join("one.md").exists());
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
}

#[test]
fn file_requests_resolve_paths_from_the_request_not_the_callers_directory() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("source.md"), "# One\nText.\n\n# Two\nMore.").unwrap();
    let book = request(json!([{"path": "source.md"}]));
    let request_path = dir.path().join("request.json");
    fs::write(&request_path, book.to_string()).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_agent-epub"))
        .current_dir(std::env::temp_dir())
        .args(["build", "--request"])
        .arg(&request_path)
        .arg("--output")
        .arg(dir.path().join("book.epub"))
        .arg("--json")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stdout)
    );
    let result: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(result["chapters"], 2);
}

#[test]
fn streamed_file_references_require_an_explicit_base() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("source.txt"), "Text.").unwrap();
    let book = request(json!([{"path": "source.txt"}]));
    let output = dir.path().join("book.epub");
    let (code, result) = run_request(&book, None, &output);
    assert_eq!(code, 2);
    assert!(
        result["errors"][0]["message"]
            .as_str()
            .unwrap()
            .contains("--base-dir")
    );
    assert_eq!(run_request(&book, Some(dir.path()), &output).0, 0);
}

#[test]
fn rejects_ambiguous_requests_and_preserves_previous_results() {
    let dir = tempfile::tempdir().unwrap();
    let output = dir.path().join("book.epub");
    fs::write(&output, "previous").unwrap();
    let variants = [
        request(json!([{"path": "file.md", "content": "Text", "format": "text"}])),
        request(json!([{"content": "Text"}])),
        request(json!([{"content": "Text", "format": "text", "name": "../escape.txt"}])),
        request(json!([{"content": "Text", "format": "text", "path": null}])),
        json!({"schema_version": 99, "metadata": {"title": "Book", "language": "en"}, "inputs": []}),
        json!({"schema_version": 1, "metadata": {"title": "Book", "language": "en"}, "inputs": [], "unknown": true}),
    ];
    for variant in variants {
        let (code, result) = run_request(&variant, Some(dir.path()), &output);
        assert_eq!(code, 2, "{result}");
        assert_eq!(result["status"], "failed");
        assert!(result["output"].is_null());
        assert_eq!(fs::read_to_string(&output).unwrap(), "previous");
    }
}

#[test]
fn discovers_the_same_schema_and_limits_that_the_tool_uses() {
    let output = Command::new(env!("CARGO_BIN_EXE_agent-epub"))
        .arg("capabilities")
        .output()
        .unwrap();
    assert!(output.status.success());
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        result["request_schema_document"],
        serde_json::from_str::<Value>(include_str!("../schemas/book-request.v1.json")).unwrap()
    );
    assert_eq!(result["limits"]["chapters"], 1000);
    assert_eq!(result["cover_images"], true);
}

#[test]
fn emits_parse_errors_as_json_when_requested() {
    let output = Command::new(env!("CARGO_BIN_EXE_agent-epub"))
        .args(["build", "--json", "--unknown"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["operation"], "parse");
    assert_eq!(result["errors"][0]["code"], "invalid_input");
}

#[test]
fn detailed_issues_preserve_categories_and_exact_source_locations() {
    let dir = tempfile::tempdir().unwrap();
    let cases = [
        (
            "# Guide\r\n\r\nCafé 日本 [missing](#absent)\r\n",
            "unresolved_link",
            3,
            9,
        ),
        (
            "# Guide\n\n[bad](javascript:alert)\n",
            "unsupported_link",
            3,
            1,
        ),
        (
            "# Guide\n\n![alt](image.png)\n",
            "unsupported_inline_image",
            3,
            1,
        ),
        (
            "# Guide\n\n<span>raw</span>\n",
            "unsupported_raw_html",
            3,
            1,
        ),
        ("# Guide\n\n- [x] Task\n", "unsupported_task_list", 3, 3),
        (
            "# Guide\n\n| A | B |\n|---|---|\n| A |\n",
            "invalid_table_shape",
            5,
            1,
        ),
        (
            "# Guide\n\nSee[^n].\n\n[^n]: A [bad](#absent).\n",
            "unresolved_link",
            5,
            9,
        ),
        (
            "# Guide\n\n[missing][ref]\n\n[ref]: missing.md\n",
            "unresolved_link",
            3,
            1,
        ),
    ];
    let output = dir.path().join("protected.epub");
    fs::write(&output, b"preserve previous output").unwrap();
    for (text, issue, line, column) in cases {
        let book = request(json!([{"name": "source.md", "format": "markdown", "content": text}]));
        let (code, result) = run_request(&book, Some(dir.path()), &output);
        assert_eq!(code, 2, "{result}");
        assert_eq!(result["output"], Value::Null);
        let error = &result["errors"][0];
        assert_eq!(error["code"], "invalid_input");
        assert_eq!(error["issue_code"], issue, "{result}");
        assert_eq!(error["line"], line, "{result}");
        assert_eq!(error["column"], column, "{result}");
        assert!(error["source"].as_str().unwrap().ends_with("source.md"));
        assert!(!error["remedy"].as_str().unwrap().is_empty());
        assert_eq!(fs::read(&output).unwrap(), b"preserve previous output");
    }
}

#[test]
fn front_matter_has_an_explicit_boundary_and_literal_extensions_preserve_text() {
    use std::io::Read;
    let dir = tempfile::tempdir().unwrap();
    for text in [
        "---\ntitle: YAML\n---\n# Book\nText.",
        "\u{feff}---\r\ntitle: YAML\r\n...\r\nText.",
        "+++\ntitle = 'TOML'\n+++\n# Book\nText.",
        "---\ntitle: Unclosed\nText.",
    ] {
        let book = request(json!([{"name": "source.md", "format": "markdown", "content": text}]));
        let (code, result) = run_request(&book, None, &dir.path().join("bad.epub"));
        assert_eq!(code, 2);
        assert_eq!(
            result["errors"][0]["issue_code"],
            "unsupported_front_matter"
        );
        assert_eq!(result["errors"][0]["line"], 1);
        assert!(!dir.path().join("bad.epub").exists());
    }
    let text = "# Literal extensions\n\nPrice $5, math $x^2$ and $$y=2$$, ~~strike~~ and ~home.\n\n> [!WARNING]\n> Keep a copy.\n\n```yaml\n---\ntitle: Visible\n---\n```\n\n---\n";
    let book = request(json!([{"name": "source.md", "format": "markdown", "content": text}]));
    let path = dir.path().join("literal.epub");
    let (code, result) = run_request(&book, None, &path);
    assert_eq!(code, 0, "{result}");
    let mut zip = zip::ZipArchive::new(fs::File::open(path).unwrap()).unwrap();
    let mut body = String::new();
    zip.by_name("EPUB/chapter-0001.xhtml")
        .unwrap()
        .read_to_string(&mut body)
        .unwrap();
    for literal in [
        "$5",
        "$x^2$",
        "$$y=2$$",
        "~~strike~~",
        "~home",
        "[!WARNING]",
        "title: Visible",
    ] {
        assert!(body.contains(literal), "{body}");
    }
    assert!(!body.contains("<del>"));
    assert!(!body.contains("<math"));
}

#[test]
fn split_levels_preserve_sources_preambles_links_and_all_request_modes() {
    use std::io::Read;
    let dir = tempfile::tempdir().unwrap();
    for level in 1..=6 {
        let marks = "#".repeat(level);
        let text = format!(
            "Preamble.\n\n{marks} Repeat\n\n[forward](#repeat-2) and [other](other.txt).\n\n###### Deep\n\n> {marks} Quoted\n> Text.\n\n{marks} Repeat\n\n[back](#repeat) and a note[^n].\n\n[^n]: Preserved note.\n"
        );
        // At H6 the Deep heading is itself an exact-level boundary.
        let chapters = if level == 6 { 5 } else { 4 };
        fs::write(dir.path().join("book.md"), &text).unwrap();
        fs::write(dir.path().join("other.txt"), "Literal other source.").unwrap();
        let mut book = request(json!([
            {"name": "book.md", "format": "markdown", "content": text},
            {"name": "other.txt", "format": "text", "content": "Literal other source."}
        ]));
        book["split_level"] = json!(level);
        let inline = dir.path().join(format!("inline-{level}.epub"));
        let (code, result) = run_request(&book, Some(dir.path()), &inline);
        assert_eq!(code, 0, "level {level}: {result}");
        assert_eq!(result["chapters"], chapters);
        let mut file_book = request(json!([{"path": "book.md"}, {"path": "other.txt"}]));
        file_book["split_level"] = json!(level);
        fs::write(dir.path().join("request.json"), file_book.to_string()).unwrap();
        let mut reference = zip::ZipArchive::new(fs::File::open(&inline).unwrap()).unwrap();
        for mode in ["direct", "file"] {
            let output = dir.path().join(format!("{mode}-{level}.epub"));
            let mut command = Command::new(env!("CARGO_BIN_EXE_agent-epub"));
            command
                .arg("build")
                .arg("--output")
                .arg(&output)
                .arg("--json");
            if mode == "direct" {
                command
                    .arg("--input")
                    .arg(dir.path().join("book.md"))
                    .arg("--input")
                    .arg(dir.path().join("other.txt"))
                    .args([
                        "--title",
                        "Agent book",
                        "--language",
                        "en",
                        "--split-level",
                        &level.to_string(),
                    ]);
            } else {
                command
                    .arg("--request")
                    .arg(dir.path().join("request.json"));
            }
            let result = command.output().unwrap();
            assert!(
                result.status.success(),
                "{}",
                String::from_utf8_lossy(&result.stdout)
            );
            let mut actual = zip::ZipArchive::new(fs::File::open(output).unwrap()).unwrap();
            for name in std::iter::once("EPUB/nav.xhtml".into())
                .chain((1..=chapters).map(|n| format!("EPUB/chapter-{n:04}.xhtml")))
            {
                let mut expected = String::new();
                let mut body = String::new();
                reference
                    .by_name(&name)
                    .unwrap()
                    .read_to_string(&mut expected)
                    .unwrap();
                actual
                    .by_name(&name)
                    .unwrap()
                    .read_to_string(&mut body)
                    .unwrap();
                assert_eq!(body, expected, "{mode} H{level} {name}");
            }
        }
    }
    for level in [
        json!(0),
        json!(7),
        json!(-1),
        json!(2.5),
        Value::Null,
        json!("2"),
    ] {
        let mut book = request(json!([{"content": "# Book\nText.", "format": "markdown"}]));
        book["split_level"] = level;
        assert_eq!(
            run_request(&book, None, &dir.path().join("invalid.epub")).0,
            2
        );
    }
    let args = Command::new(env!("CARGO_BIN_EXE_agent-epub"))
        .args([
            "build",
            "--request",
            "-",
            "--split-level",
            "2",
            "--output",
            "unused.epub",
            "--json",
        ])
        .output()
        .unwrap();
    assert_eq!(args.status.code(), Some(2));
}

#[test]
fn title_plus_h2_chapters_keep_preamble_and_do_not_duplicate_title_entries() {
    use std::io::Read;
    let dir = tempfile::tempdir().unwrap();
    let mut book = request(
        json!([{"name": "guide.md", "format": "markdown", "content": include_str!("../examples/title-h2.md")}]),
    );
    book["split_level"] = json!(2);
    let output = dir.path().join("guide.epub");
    let (code, result) = run_request(&book, None, &output);
    assert_eq!(code, 0, "{result}");
    assert_eq!(result["chapters"], 4);
    let mut zip = zip::ZipArchive::new(fs::File::open(&output).unwrap()).unwrap();
    let mut nav = String::new();
    zip.by_name("EPUB/nav.xhtml")
        .unwrap()
        .read_to_string(&mut nav)
        .unwrap();
    assert_eq!(nav.matches("<a ").count(), 5);
    assert!(nav.contains("Field guide"));
    assert!(!nav.contains("Quoted heading"));
    let mut body = String::new();
    zip.by_name("EPUB/chapter-0001.xhtml")
        .unwrap()
        .read_to_string(&mut body)
        .unwrap();
    assert!(body.contains("Introduction before the first chapter."));
    assert!(body.contains("chapter-0003.xhtml#results"));
    body.clear();
    zip.by_name("EPUB/chapter-0002.xhtml")
        .unwrap()
        .read_to_string(&mut body)
        .unwrap();
    assert!(body.contains("<h2 id=\"methods\""));
    assert!(body.contains("<h4 id=\"detail\""));
    assert!(body.contains("chapter-0003.xhtml#noteref:1:2"));
}

#[test]
fn historical_markdown_probes_have_current_declared_outcomes() {
    let dir = tempfile::tempdir().unwrap();
    let historical: Value = serde_json::from_str(include_str!(
        "../docs/research/market-research-probes-2026-11-10.json"
    ))
    .unwrap();
    for probe in historical["probes"].as_array().unwrap() {
        let name = probe["case"].as_str().unwrap();
        let book = request(
            json!([{"name": "source.md", "format": "markdown", "content": probe["input"]}]),
        );
        let (code, result) = run_request(&book, None, &dir.path().join(format!("{name}.epub")));
        let expected = match name {
            "inline_image" => Some("unsupported_inline_image"),
            "task_list" => Some("unsupported_task_list"),
            "raw_html" => Some("unsupported_raw_html"),
            "yaml_frontmatter" => Some("unsupported_front_matter"),
            "broken_link" => Some("unresolved_link"),
            _ => None,
        };
        if let Some(issue) = expected {
            assert_eq!(code, 2, "{name}: {result}");
            assert_eq!(result["errors"][0]["issue_code"], issue);
        } else {
            assert_eq!(code, 0, "{name}: {result}");
        }
    }
    let book = request(
        json!([{"format": "markdown", "content": "---\n\n# Guide\n\nKey: ordinary prose after a heading.\n"}]),
    );
    assert_eq!(
        run_request(&book, None, &dir.path().join("thematic.epub")).0,
        0
    );
}

#[test]
fn links_to_sources_without_chapters_fail_as_localized_json_instead_of_panicking() {
    let dir = tempfile::tempdir().unwrap();
    let output = dir.path().join("protected.epub");
    fs::write(&output, b"previous book").unwrap();
    let book = request(json!([
        {"name": "main.md", "format": "markdown", "content": "# Main\n\n[Notes](notes.md)\n"},
        {"name": "notes.md", "format": "markdown", "content": "[^unused]: No chapter text.\n"}
    ]));
    let (code, result) = run_request(&book, None, &output);
    assert_eq!(code, 2, "{result}");
    assert_eq!(result["status"], "failed");
    assert_eq!(result["output"], Value::Null);
    assert_eq!(result["errors"][0]["code"], "invalid_input");
    assert_eq!(result["errors"][0]["issue_code"], "unresolved_link");
    assert_eq!(result["errors"][0]["line"], 3);
    assert_eq!(result["errors"][0]["column"], 1);
    assert!(
        result["errors"][0]["source"]
            .as_str()
            .unwrap()
            .ends_with("main.md")
    );
    assert_eq!(fs::read(output).unwrap(), b"previous book");
}
