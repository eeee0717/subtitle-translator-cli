use clap::{Parser, Subcommand};
use std::{error::Error, path::PathBuf};
use subtitle_translator_cli::handler::handle_openai_translate;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match args.command {
        Command::OpenAI {
            path,
            source_language,
            target_language,
        } => handle_openai_translate(path, source_language, target_language)
            .await
            .expect("Failed to handle OpenAI translation"),
    }
    Ok(())
}
