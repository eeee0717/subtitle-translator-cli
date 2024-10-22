use crate::{
    formatter::Formatter,
    openai::OpenAI,
    parse::*,
    subtitle_extractor::SubtitleExtractor,
    text_splitter::{self, TextSplitter},
};
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
    let subtitle_extractor = SubtitleExtractor::extractor(&subtitle_entries);
    let text_splitter = TextSplitter::split_text(&subtitle_extractor.text_info);
    // TODO: use loop to iterate over text_splitter.split_result
    let index = 0;
    let formatter = Formatter::format(index, &text_splitter.split_result);
}
