use std::path::PathBuf;

use clap::{Parser, Subcommand};
use subtitle_translator_cli::handle::handle_google_translate;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
#[clap(rename_all = "snake_case")]
enum Command {
    OpenAI {
        #[arg(short)]
        path: PathBuf,
        #[arg(short)]
        source_language: String,
        #[arg(short)]
        target_language: String,
    },
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::OpenAI {
            path,
            source_language,
            target_language,
        } => handle_google_translate(path, source_language, target_language),
    }
}
