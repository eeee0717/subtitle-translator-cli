#[macro_use]
extern crate lazy_static;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use subtitle_translator_cli::handle::handle_openai_translate;
use tera::Tera;

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
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("templates/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.command {
        Command::OpenAI {
            path,
            source_language,
            target_language,
        } => handle_openai_translate(path, source_language, target_language),
    }
}
