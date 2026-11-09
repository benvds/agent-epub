use agent_epub::{
    BuildOptions, Error, ErrorKind, Metadata, Result, Summary,
    agent::{self, RequestOptions},
};
use clap::{Parser, Subcommand};
use std::{
    io::Read,
    path::{Path, PathBuf},
    time::Instant,
};

#[derive(Parser)]
#[command(version, about = "Focused EPUB generation for AI agents")]
struct Cli {
    #[arg(
        long,
        global = true,
        help = "Return one structured JSON result on stdout"
    )]
    json: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a checked EPUB from ordered files or a versioned book request.
    Build {
        #[arg(
            short = 'i',
            long = "input",
            required_unless_present = "request",
            conflicts_with = "request"
        )]
        inputs: Vec<PathBuf>,
        #[arg(long, conflicts_with_all = ["inputs", "title", "language", "author"], help = "Book request JSON file, or - for stdin")]
        request: Option<PathBuf>,
        #[arg(
            long,
            requires = "request",
            help = "Base directory for file references in a streamed request"
        )]
        base_dir: Option<PathBuf>,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(long, required_unless_present = "request")]
        title: Option<String>,
        #[arg(long, required_unless_present = "request")]
        language: Option<String>,
        #[arg(long)]
        author: Option<String>,
        #[arg(long)]
        overwrite: bool,
        #[arg(
            long,
            help = "EPUBCheck executable wrapper; failure prevents output publication"
        )]
        epubcheck: Option<PathBuf>,
    },
    /// Check this tool's generated profile, not full EPUB conformance.
    Validate {
        file: PathBuf,
        #[arg(long)]
        epubcheck: Option<PathBuf>,
    },
    /// Discover supported inputs, features, limits and the book-request schema.
    Capabilities,
}

fn report(summary: &Summary) {
    println!(
        "{} chapters; structural profile checks passed.",
        summary.chapters
    );
    if summary.independent_validation {
        println!("EPUBCheck passed; review its warnings separately.");
    } else {
        println!(
            "EPUBCheck not run. Full conformance, accessibility and reader appearance are not certified."
        );
    }
}

fn request_text(path: &Path) -> Result<(String, Option<PathBuf>)> {
    let mut bytes = Vec::new();
    let request_path = if path == Path::new("-") {
        std::io::stdin()
            .take(64 * 1024 * 1024 + 1)
            .read_to_end(&mut bytes)
            .map_err(|e| Error::new(ErrorKind::Io, e.to_string()))?;
        None
    } else {
        let file = std::fs::File::open(path)
            .map_err(|e| Error::new(ErrorKind::Io, format!("{}: {e}", path.display())))?;
        file.take(64 * 1024 * 1024 + 1)
            .read_to_end(&mut bytes)
            .map_err(|e| Error::new(ErrorKind::Io, e.to_string()))?;
        Some(
            path.canonicalize()
                .map_err(|e| Error::new(ErrorKind::Io, e.to_string()))?,
        )
    };
    if bytes.len() > 64 * 1024 * 1024 {
        return Err(Error::new(ErrorKind::Input, "Request limit: 64 MiB."));
    }
    let text = String::from_utf8(bytes)
        .map_err(|e| Error::new(ErrorKind::Input, format!("Expected a UTF-8 request: {e}")))?;
    Ok((text, request_path))
}

fn main() {
    let started = Instant::now();
    let args = std::env::args_os().collect::<Vec<_>>();
    let wants_json = args.iter().any(|a| a == "--json");
    let cli = match Cli::try_parse_from(args) {
        Ok(cli) => cli,
        Err(error) if wants_json && error.use_stderr() => {
            let error = Error::new(ErrorKind::Input, error.to_string());
            println!(
                "{}",
                agent::result("parse", &Err(error), None, started.elapsed().as_millis())
            );
            std::process::exit(2);
        }
        Err(error) => error.exit(),
    };
    let (operation, output, outcome) = match cli.command {
        Commands::Capabilities => {
            println!("{}", agent::capabilities());
            return;
        }
        Commands::Build {
            inputs,
            request,
            base_dir,
            output,
            title,
            language,
            author,
            overwrite,
            epubcheck,
        } => {
            let outcome = if let Some(path) = request {
                request_text(&path).and_then(|(text, request_path)| {
                    agent::build_request(
                        &text,
                        &RequestOptions {
                            output: output.clone(),
                            base_dir,
                            request_path,
                            overwrite,
                            epubcheck,
                        },
                    )
                })
            } else {
                agent_epub::build(&BuildOptions {
                    inputs,
                    output: output.clone(),
                    metadata: Metadata {
                        title: title.unwrap_or_default(),
                        language: language.unwrap_or_default(),
                        author,
                    },
                    overwrite,
                    epubcheck,
                })
            };
            ("build", Some(output), outcome)
        }
        Commands::Validate { file, epubcheck } => {
            let outcome = agent_epub::validate_with_epubcheck(&file, epubcheck.as_deref());
            ("validate", Some(file), outcome)
        }
    };
    let code = outcome.as_ref().err().map(|e| e.kind as i32).unwrap_or(0);
    if cli.json {
        println!(
            "{}",
            agent::result(
                operation,
                &outcome,
                output.as_deref(),
                started.elapsed().as_millis()
            )
        );
    } else {
        match outcome {
            Ok(summary) => {
                if operation == "build" {
                    println!("Created {}", output.unwrap().display());
                }
                report(&summary);
            }
            Err(error) => eprintln!("error: {error}"),
        }
    }
    std::process::exit(code);
}
