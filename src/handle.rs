use crate::{
    formatter::Formatter, subtitle_combiner::SubtitleCombiner,
    subtitle_extractor::SubtitleExtractor, text_splitter::TextSplitter, GROUP_SIZE,
};
use std::path::PathBuf;

pub async fn handle_openai_translate(
    path: PathBuf,
    source_language: String,
    target_language: String,
) {
    let subtitle_entries = crate::parse::parse_file(&path).expect("Failed to parse file");
    eprintln!(
        "Translating {} entries from {} to {}",
        subtitle_entries.len(),
        source_language,
        target_language
    );
    let subtitle_extractor =
        SubtitleExtractor::extractor(&subtitle_entries).expect("Failed to extract subtitle");
    let text_splitter =
        TextSplitter::split_text(&subtitle_extractor.text_info).expect("Failed to split text");
    let translator = crate::translator::Translator::new();
    let mut current_index = 0;
    let mut final_srt_content = String::new();
    for index in 0..(subtitle_entries.len() / GROUP_SIZE) {
        // TODO: use loop to iterate over text_splitter.split_result
        let formatter = Formatter::format(index, &text_splitter.split_result);
        let mut translator = translator
            .translate(
                &source_language,
                &target_language,
                formatter.tagged_text,
                formatter.chunk_to_translate.clone(),
            )
            .await;
        let translator = translator.format_translated_result();
        let subtitle_combiner = SubtitleCombiner::combine(
            formatter.chunk_to_translate,
            translator.translated_result,
            subtitle_extractor.time_info.clone(),
            current_index,
            subtitle_extractor.number_info.clone(),
        );
        current_index = subtitle_combiner.current_index;
        eprintln!("Translated {} entries", current_index);
        final_srt_content.push_str(&subtitle_combiner.srt_content);
    }
    eprintln!("Final srt content: {}", final_srt_content);
}
