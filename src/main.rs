use subtitle_translator_cli::{translate, Config};

fn main() {
    let config = Config::build(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    let translated_text = translate(config);

    println!("{:?}", translated_text);
}
