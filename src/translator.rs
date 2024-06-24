use rustlate;
use std::error::Error;

pub fn translate(
    contents: String,
    input_language: String,
    output_language: String,
) -> Result<String, Box<dyn Error>> {
    let translator_struct = rustlate::Translator {
        to: output_language.leak(),
        from: input_language.leak(),
    };
    let translated_text = translator_struct.translate(contents.leak())?;
    Ok(translated_text)
}
