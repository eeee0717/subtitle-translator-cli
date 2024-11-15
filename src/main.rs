use clap::{Parser, Subcommand};
use std::path::PathBuf;
use subtitle_translator_cli::{config::Config, handler::handle_openai_translate};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Openai {
        #[arg(short)]
        path: PathBuf,
        #[arg(short)]
        source_language: String,
        #[arg(short)]
        target_language: String,
    },
}

#[tokio::main]
async fn main() {
    if let Err(e) = Config::init() {
        eprintln!("配置初始化失败: {}", e);
        std::process::exit(1);
    }
    let args = Args::parse();
    match args.command {
        Command::Openai {
            path,
            source_language,
            target_language,
        } => handle_openai_translate(path, source_language, target_language)
            .await
            .expect("Failed to handle OpenAI translation"),
    }
}
