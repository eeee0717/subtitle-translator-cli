use subparse::SubtitleEntry;

use crate::{
    formatter::Formatter, subtitle_combiner::SubtitleCombiner,
    subtitle_extractor::SubtitleExtractor, text_splitter::TextSplitter, translator::Translator,
    GROUP_SIZE,
};
use std::path::PathBuf;

pub struct Handler {
    pub subtitle_entries: Vec<SubtitleEntry>,
    pub subtitle_extractor: SubtitleExtractor,
    pub text_splitter: TextSplitter,
    pub translator: Translator,
    pub subtitle_combiner: SubtitleCombiner,
}

impl Handler {
    pub fn setup(path: PathBuf) -> Self {
        let subtitle_entries = crate::parse::parse_file(&path).expect("Failed to parse file");
        let subtitle_extractor =
            SubtitleExtractor::extractor(&subtitle_entries).expect("Failed to extract subtitle");
        let text_splitter =
            TextSplitter::split_text(&subtitle_extractor.text_info).expect("Failed to split text");
        let translator = Translator::new();
        let subtitle_combiner = SubtitleCombiner::new();
        Self {
            subtitle_entries,
            subtitle_extractor,
            text_splitter,
            translator,
            subtitle_combiner,
        }
    }
    pub async fn handle_translator(
        &mut self,
        source_language: String,
        target_language: String,
    ) -> String {
        let mut final_srt_content = String::new();

        for index in 0..(self.subtitle_entries.len() / GROUP_SIZE) {
            // TODO: use loop to iterate over text_splitter.split_result
            let formatter = Formatter::format(index, &self.text_splitter.split_result);
            self.translator
                .translate(
                    &source_language,
                    &target_language,
                    formatter.tagged_text,
                    formatter.chunk_to_translate.clone(),
                )
                .await
                .expect("Failed to translate");
            let formatted_result = self.translator.format_translated_result();

            let input = crate::subtitle_combiner::CombineInput {
                combined_text: formatter.chunk_to_translate,
                translated_text: formatted_result,
                time_info: self.subtitle_extractor.time_info.clone(),
                number_info: self.subtitle_extractor.number_info.clone(),
            };

            self.subtitle_combiner
                .combine(input)
                .expect("Combine should succeed");

            eprintln!(
                "Translated {} entries",
                self.subtitle_combiner.get_current_index()
            );
            final_srt_content.push_str(&self.subtitle_combiner.get_content());
            eprintln!("Final srt content: {}", final_srt_content);
        }
        final_srt_content
    }
}
pub async fn handle_openai_translate(
    path: PathBuf,
    source_language: String,
    target_language: String,
) {
    let mut handler = Handler::setup(path);
    let final_srt_content = handler
        .handle_translator(source_language, target_language)
        .await;
    eprintln!("Final srt content:\n{}", final_srt_content);
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_handle() {
        let path = std::path::PathBuf::from("test2.srt");
        crate::handler::handle_openai_translate(path, "en".to_string(), "zh_CN".to_string()).await;
    }
}
