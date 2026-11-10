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
    assert!(result["output"].as_str().unwrap().starts_with('/'));
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
