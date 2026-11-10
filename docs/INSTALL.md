# Install and update Agent EPUB

Version 0.5.1 ships MIT-licensed native binaries. Generation runs offline and
needs no Rust, Python, Java or AI account. Downloading from this private
repository requires a GitHub account with repository access, plus a browser
or the GitHub CLI (`gh auth login`). Repository visibility is unchanged.

| Archive target | Native installation check | Runtime boundary |
| --- | --- | --- |
| `aarch64-apple-darwin` | macOS 15, Apple Silicon | macOS system libraries; Intel Macs are not included. |
| `x86_64-unknown-linux-gnu` | Ubuntu 24.04, x86_64 | GNU/Linux with glibc 2.39 or later; not a static or Alpine/musl binary. |
| `x86_64-pc-windows-msvc` | Windows Server 2025, x86_64 | Windows system libraries; the C runtime is statically linked. |

These are the release verification environments, not a claim that every
compatible OS release was exercised. Each release attaches the native smoke
reports with archive hashes. Checksums detect transfer corruption; they are
not a code signature. Download archives and checksums from the same trusted
release. The macOS executable is not signed or notarized.

## macOS Apple Silicon

In a new empty download directory:

```sh
gh release download v0.5.1 --repo benvds/epub-rust \
  --pattern 'agent-epub-0.5.1-aarch64-apple-darwin.zip*'
shasum -a 256 -c agent-epub-0.5.1-aarch64-apple-darwin.zip.sha256
unzip agent-epub-0.5.1-aarch64-apple-darwin.zip
cd agent-epub-0.5.1-aarch64-apple-darwin
chmod +x agent-epub
./agent-epub --version
export PATH="$PWD:$PATH"
```

Keep the extracted directory for its examples, licenses and skill. To make the
executable available in future terminals, place it in a directory already on
your PATH, or add this extracted directory's absolute path to your shell setup.

## Linux x86_64

```sh
gh release download v0.5.1 --repo benvds/epub-rust \
  --pattern 'agent-epub-0.5.1-x86_64-unknown-linux-gnu.zip*'
sha256sum --check agent-epub-0.5.1-x86_64-unknown-linux-gnu.zip.sha256
unzip agent-epub-0.5.1-x86_64-unknown-linux-gnu.zip
cd agent-epub-0.5.1-x86_64-unknown-linux-gnu
chmod +x agent-epub
./agent-epub --version
export PATH="$PWD:$PATH"
```

Keep the extracted directory or copy the executable to a directory on PATH.
Use a source build on systems outside the declared binary boundary.

## Windows x86_64

Use PowerShell in a new empty download directory:

```powershell
gh release download v0.5.1 --repo benvds/epub-rust --pattern 'agent-epub-0.5.1-x86_64-pc-windows-msvc.zip*'
if ($LASTEXITCODE -ne 0) { throw 'Download failed' }
$archive = 'agent-epub-0.5.1-x86_64-pc-windows-msvc.zip'
$expected = ((Get-Content "$archive.sha256") -split '\s+')[0]
if ((Get-FileHash $archive -Algorithm SHA256).Hash.ToLowerInvariant() -ne $expected) { throw 'Checksum mismatch' }
Expand-Archive $archive -DestinationPath .
Set-Location agent-epub-0.5.1-x86_64-pc-windows-msvc
.\agent-epub.exe --version
$env:PATH = "$PWD;$env:PATH"
```

Keep this directory and add its absolute path to your user PATH through Windows
Environment Variables for future terminals. The ZIP includes documentation,
examples and licenses alongside the executable. No Rust toolchain or separate
Visual C++ runtime installer is needed.

## Try the installed binary

From the extracted directory, with its executable on PATH:

```sh
agent-epub capabilities
agent-epub build --request examples/report-request.json --output report.epub --json
agent-epub validate report.epub --json
```

Open `report.epub` in an EPUB reader. The archive also includes the generated
`examples/sample-report.epub` and its `examples/research-report.md` source.
Generation requires only the executable; the bundled files are optional helpers.

## Agent skill

Copy the complete `skills/agent-epub` folder into a skill directory supported by
your agent runtime. For Codex project use, the destination is
`.agents/skills/agent-epub`; for personal use it is `~/.codex/skills/agent-epub`.
On Windows the personal directory is under your user profile. The binary must
be on the agent's PATH or invoked by its full path, with permission to read the
sources and write the destination. The skill does not install the executable.

The skill covers discovery, ordered files, request files, streamed inline
requests and result inspection. It does not establish live testing of every
provider. Claude and Ollama live evaluations remain deferred.

## Updates and removal

Download the new version into a separate directory, verify its checksum, and run
its `--version` and sample build before changing PATH or replacing your installed
executable. Keep the old version for rollback. Version 0.5 retains request and
result schema version 1. Do not overwrite manuscripts or EPUB outputs during an
update. Remove the extracted directory and its PATH entry to uninstall; no
background service or model configuration is installed.

## Optional independent validation

EPUBCheck is separate from the binary. Download the official distribution and
install its supported Java runtime. On macOS/Linux use the bundled
`scripts/epubcheck.sh`, setting `EPUBCHECK_JAR` to the official JAR path, and pass
that wrapper through `--epubcheck`. On Windows supply an executable wrapper that
accepts the EPUB path as its single argument and returns EPUBCheck's exit code;
the POSIX wrapper requires a POSIX shell. A shell command string or bare JAR is
not an executable wrapper. See `docs/USAGE.md` for details.

Missing or failed requested validators fail the operation. Without the option,
JSON explicitly reports `epubcheck: not_run`. Built-in validation covers only
the generated structural profile; accessibility and reading appearance need
separate review.

## Source build

Rust 1.88 or later is required for a source build; release CI uses Rust 1.95.0.

```sh
cargo build --release --locked
target/release/agent-epub --version
```

On Windows use `target/release/agent-epub.exe`. Release packaging and smoke tests
use Python 3.11 or later as development tooling. The packaging allowlist in
`scripts/release.py` excludes manuscripts and local evaluation outputs.
