use indicatif::{ProgressState, ProgressStyle};
use subparse::SubtitleEntry;

use crate::{
    formatter::Formatter, subtitle_combiner::SubtitleCombiner,
    subtitle_extractor::SubtitleExtractor, text_splitter::TextSplitter, translator::Translator,
    writer::Writer, GROUP_SIZE,
};
use std::{
    fmt::Write,
    path::{self, PathBuf},
};

#[derive(Debug)]
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
    ) -> Result<String, Box<dyn std::error::Error>> {
        let chunk_count = self.subtitle_entries.len() / GROUP_SIZE;
        let pb = indicatif::ProgressBar::new(chunk_count.try_into().unwrap());
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}]  {pos}/{len} ({eta})",
            )
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
                write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
            })
            .progress_chars("#>-"),
        );
        pb.set_position(0);

        let mut final_srt_content = String::with_capacity(self.subtitle_entries.len());

        for index in 0..chunk_count {
            final_srt_content.push_str(
                &self
                    .process_chunk(index, &source_language, &target_language)
                    .await?,
            );
            pb.inc(1);
        }
        pb.finish_with_message("done");

        Ok(final_srt_content)
    }
    pub async fn process_chunk(
        &mut self,
        index: usize,
        source_language: &str,
        target_language: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let formatter = Formatter::format(index, &self.text_splitter.split_result);
        self.translator
            .translate(
                source_language,
                target_language,
                formatter.tagged_text,
                formatter.chunk_to_translate.clone(),
            )
            .await?;
        let formatted_result = self.translator.format_translated_result();

        let input = crate::subtitle_combiner::CombineInput {
            combined_text: formatter.chunk_to_translate,
            translated_text: formatted_result,
            time_info: self.subtitle_extractor.time_info.clone(),
            number_info: self.subtitle_extractor.number_info.clone(),
        };

        self.subtitle_combiner.combine(input)?;

        eprintln!(
            "Translated {} entries",
            self.subtitle_combiner.get_current_index()
        );

        let content = self.subtitle_combiner.get_content();
        eprintln!("Chunk content: {}", content);

        Ok(content.to_string())
    }
}
pub async fn handle_openai_translate(
    path: PathBuf,
    source_language: String,
    target_language: String,
) {
    let mut handler = Handler::setup(path.clone());
    let final_srt_content = handler
        .handle_translator(source_language, target_language.clone())
        .await
        .expect("Failed to handle translator");
    eprintln!("Final srt content:\n{}", final_srt_content);
    // get file name without extension
    let file_name = path.file_stem().unwrap().to_str().unwrap();
    let path = path.with_file_name(format!("{}_{}.srt", file_name, target_language));
    Writer::write_file(final_srt_content, path);
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_handle() {
        let path = std::path::PathBuf::from("test2.srt");

        crate::handler::handle_openai_translate(path, "en".to_string(), "zh_CN".to_string()).await;
    }
}
