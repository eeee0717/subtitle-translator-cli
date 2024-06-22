use std::{error::Error, fs};

use crate::config::Config;
use rustlate;

pub fn translate(config: Config) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let translator_struct = rustlate::Translator {
        to: config.output_language.leak(),
        from: config.input_language.leak(),
    };
    let translated_text = translator_struct.translate(contents.leak())?;

    Ok(translated_text)
}
