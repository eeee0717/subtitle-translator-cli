// processor.rs
use crate::{SrtFile, SubtitleFile};
use std::error::Error;
use std::fs;

fn read_file_trim_bom(contents: &str) -> String {
    let bom = "\u{FEFF}";
    if contents.starts_with(bom) {
        contents[bom.len()..].to_string()
    } else {
        contents.to_string()
    }
}

pub fn process_file(
    file_path: String,
    _input_language: String,
    _output_language: String,
) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(&file_path).expect("Something went wrong reading the file");
    let contents = read_file_trim_bom(&contents);
    let file_extension = file_path.split('.').last().unwrap_or("");

    let file_struct = match file_extension {
        "srt" => Box::new(SrtFile {}),
        _ => return Err("Unsupported file type".into()),
    };

    let split_contents = file_struct.split_contents(&contents).unwrap();

    println!("{:?}", split_contents);
    let mut translated_combined_text = Vec::new();
    for content in split_contents {
        let translated_text =
            crate::translate(content, _input_language.clone(), _output_language.clone())?;
        translated_combined_text.push(translated_text);
    }

    Ok(translated_combined_text)
}
