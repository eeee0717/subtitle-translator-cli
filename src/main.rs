use subtitle_translator_cli::{process_file, Config};

fn main() {
    let config = Config::build(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    match process_file(
        config.file_path,
        config.input_language,
        config.output_language,
    ) {
        Ok(_) => println!("Finished"),
        Err(e) => eprintln!("Translation error: {}", e),
    }
}
