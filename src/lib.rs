use std::sync::Mutex;

use config::Config;
use once_cell::sync::Lazy;

#[macro_use]
extern crate lazy_static;
pub mod config;
pub mod formatter;
pub mod handler;
pub mod mock;
pub mod openai;
pub mod parse;
pub mod subtitle_combiner;
pub mod subtitle_extractor;
pub mod text_splitter;
pub mod translator;
pub mod writer;
/// global constants
const GROUP_SIZE: usize = 5;
static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
    let config = Config::read_config_from_file("config.json").expect("Failed to read config");
    Mutex::new(config)
});
lazy_static! {
    pub static ref TEMPLATES: tera::Tera = {
        let tera = match tera::Tera::new("src/templates/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}

#[cfg(test)]
mod test {
    use crate::CONFIG;

    #[test]
    fn test_tera() {
        let mut context = tera::Context::new();
        context.insert("source_lang", "en");
        context.insert("target_lang", "ja");
        match crate::TEMPLATES.render("prompt.txt", &context) {
            Ok(s) => println!("{:?}", s),
            Err(e) => {
                println!("Error: {}", e);
                let mut cause = std::error::Error::source(&e);
                while let Some(e) = cause {
                    println!("Reason: {}", e);
                    cause = e.source();
                }
            }
        };
    }
    #[test]
    fn test_indicatif() {
        let pb = indicatif::ProgressBar::new(1024);
        pb.set_style(
            indicatif::ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .with_key(
                "eta",
                |state: &indicatif::ProgressState, w: &mut dyn std::fmt::Write| {
                    write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
                },
            )
            .progress_chars("#>-"),
        );
        pb.set_position(0);
        for _ in 0..1024 {
            std::thread::sleep(std::time::Duration::from_millis(1));
            pb.inc(1);
        }
        pb.finish_with_message("done");
        assert!(true);
    }
    #[test]
    fn test_config() {
        let config = CONFIG.lock().unwrap();
        println!("{:?}", config);
        assert!(!config.api_key.is_empty());
    }
}
