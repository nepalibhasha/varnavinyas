mod cmd_akshar;
mod cmd_check;
mod cmd_lipi;

use clap::{Parser, Subcommand, ValueEnum};
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "varnavinyas", about = "Nepali orthography toolkit")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Spell-check Nepali text
    Check {
        /// File to check (use - for stdin, default: stdin)
        input: Option<String>,

        /// Show rule explanations
        #[arg(long)]
        explain: bool,

        /// Enable optional grammar/samasa heuristic diagnostics
        #[arg(long)]
        grammar: bool,

        /// Output format
        #[arg(long, value_enum, default_value = "text")]
        format: OutputFormat,
    },

    /// Analyze Devanagari characters and syllables
    Akshar {
        /// Text to analyze
        text: String,
    },

    /// Transliterate between scripts
    Lipi {
        /// Text to transliterate
        text: String,

        /// Source script
        #[arg(long, default_value = "devanagari")]
        from: String,

        /// Target script
        #[arg(long, default_value = "iast")]
        to: String,
    },
}

#[derive(ValueEnum, Clone, Copy)]
enum OutputFormat {
    Text,
    Json,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check {
            input,
            explain,
            grammar,
            format,
        } => cmd_check::run(input, explain, grammar, format),
        Commands::Akshar { text } => {
            cmd_akshar::run(&text);
            ExitCode::SUCCESS
        }
        Commands::Lipi { text, from, to } => cmd_lipi::run(&text, &from, &to),
    }
}
