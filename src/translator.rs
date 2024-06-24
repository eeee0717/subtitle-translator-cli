use rustlate;

pub fn translate(contents: String, input_language: String, output_language: String) -> String {
    let translator_struct = rustlate::Translator {
        to: output_language.leak(),
        from: input_language.leak(),
    };
    let translated_text = match translator_struct.translate(contents.clone().leak()) {
        Ok(text) => text,
        Err(e) => {
            println!("Translation error: {:?}", e);
            contents
        }
    };
    translated_text
}
