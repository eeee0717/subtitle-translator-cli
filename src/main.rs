use clap::Parser;
use subtitle_translator_cli::{process_file, Args};

fn main() {
    let args = Args::parse();

    match process_file(args.path, args.source_language, args.target_language) {
        Ok(_) => println!("Finished"),
        Err(e) => eprintln!("Translation error: {}", e),
    }
}
