use agent_epub::{BuildOptions, Metadata, Summary};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "Focused EPUB generation for AI agents")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a structurally checked EPUB from ordered .txt/.md sources.
    Build {
        #[arg(short = 'i', long = "input", required = true)]
        inputs: Vec<PathBuf>,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(long)]
        title: String,
        #[arg(long, help = "Publication language tag, for example en, nl or en-US")]
        language: String,
        #[arg(long)]
        author: Option<String>,
        #[arg(long)]
        overwrite: bool,
        #[arg(
            long,
            help = "Optional EPUBCheck executable wrapper; failure prevents output publication"
        )]
        epubcheck: Option<PathBuf>,
    },
    /// Check this tool's generated EPUB profile, not full EPUB conformance.
    Validate {
        file: PathBuf,
        #[arg(long, help = "Also run an EPUBCheck executable wrapper")]
        epubcheck: Option<PathBuf>,
    },
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

fn main() {
    let result = match Cli::parse().command {
        Commands::Build {
            inputs,
            output,
            title,
            language,
            author,
            overwrite,
            epubcheck,
        } => agent_epub::build(&BuildOptions {
            inputs,
            output: output.clone(),
            metadata: Metadata {
                title,
                language,
                author,
            },
            overwrite,
            epubcheck,
        })
        .map(|summary| {
            println!("Created {}", output.display());
            report(&summary);
        }),
        Commands::Validate { file, epubcheck } => {
            agent_epub::validate_with_epubcheck(&file, epubcheck.as_deref())
                .map(|summary| report(&summary))
        }
    };
    if let Err(error) = result {
        eprintln!("error: {error}");
        std::process::exit(error.kind as i32);
    }
}
