use subtitle_translator_cli::{translate, Config};

fn main() {
    let config = Config::build(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    let translator = translate(&config);

    println!("{:?}", config.file_name);
}
