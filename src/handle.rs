use crate::parse::*;
use std::path::PathBuf;

pub fn handle_google_translate(path: PathBuf, source_language: String, target_language: String) {
    let subtitle_entries = parse_file(&path);
    eprintln!(
        "Translating {} entries from {} to {}",
        subtitle_entries.len(),
        source_language,
        target_language
    );
}
