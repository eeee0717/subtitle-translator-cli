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
    let mut translator = crate::translator::Translator::new();
    let mut final_srt_content = String::new();
    let mut subtitle_combiner = SubtitleCombiner::new();

    for index in 0..(subtitle_entries.len() / GROUP_SIZE) {
        // TODO: use loop to iterate over text_splitter.split_result
        let formatter = Formatter::format(index, &text_splitter.split_result);
        translator
            .translate(
                &source_language,
                &target_language,
                formatter.tagged_text,
                formatter.chunk_to_translate.clone(),
            )
            .await
            .expect("Failed to translate");
        let formatted_result = translator.format_translated_result();

        let input = crate::subtitle_combiner::CombineInput {
            combined_text: formatter.chunk_to_translate,
            translated_text: formatted_result,
            time_info: subtitle_extractor.time_info.clone(),
            number_info: subtitle_extractor.number_info.clone(),
        };

        subtitle_combiner
            .combine(input)
            .expect("Combine should succeed");

        eprintln!(
            "Translated {} entries",
            subtitle_combiner.get_current_index()
        );
        final_srt_content.push_str(&subtitle_combiner.get_content());
        eprintln!("Final srt content: {}", final_srt_content);
    }
    eprintln!("Final srt content: {}", final_srt_content);
}
