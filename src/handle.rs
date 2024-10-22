use crate::{openai::OpenAI, parse::*};
use std::path::PathBuf;

pub fn handle_openai_translate(path: PathBuf, source_language: String, target_language: String) {
    let subtitle_entries = parse_file(&path);
    eprintln!(
        "Translating {} entries from {} to {}",
        subtitle_entries.len(),
        source_language,
        target_language
    );
    let openai = OpenAI::new();
    eprintln!("OpenAI initialized!\nOpenai: {:?}", openai);
}
