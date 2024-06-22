use crate::config::Config;
use rustlate;

pub fn translate(config: &'static Config) -> Result<String, &'static str> {
    let translator_struct = rustlate::Translator {
        to: &config.output_language,
        from: &config.input_language,
    };

    match translator_struct.translate("hello.") {
        Ok(translated) => Ok(translated),
        Err(_) => return Err("Something went wrong..."),
    }
}
