use indicatif::{ProgressState, ProgressStyle};
use subparse::SubtitleEntry;

use crate::{
    formatter::Formatter, subtitle_combiner::SubtitleCombiner,
    subtitle_extractor::SubtitleExtractor, text_splitter::TextSplitter, translator::Translator,
    writer::Writer, GROUP_SIZE,
};
use std::{fmt::Write, path::PathBuf};

#[derive(Debug)]
pub struct Handler {
    pub subtitle_entries: Vec<SubtitleEntry>,
    pub subtitle_extractor: SubtitleExtractor,
    pub text_splitter: TextSplitter,
    pub translator: Translator,
    pub subtitle_combiner: SubtitleCombiner,
    pub progress_bar: indicatif::ProgressBar,
}

impl Handler {
    pub fn from_path(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let subtitle_entries = crate::parse::parse_file(&path)?;
        Self::new(subtitle_entries)
    }

    pub fn new(subtitle_entries: Vec<SubtitleEntry>) -> Result<Self, Box<dyn std::error::Error>> {
        let subtitle_extractor = SubtitleExtractor::extractor(&subtitle_entries)?;
        let text_splitter = TextSplitter::split_text(&subtitle_extractor.text_info)?;
        let progress_bar =
            indicatif::ProgressBar::new((subtitle_entries.len() / GROUP_SIZE).try_into().unwrap());
        progress_bar.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}]  {pos}/{len} ({eta})",
            )
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
                write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
            })
            .progress_chars("#>-"),
        );
        progress_bar.set_position(0);
        Ok(Self {
            subtitle_entries,
            subtitle_extractor,
            text_splitter,
            translator: Translator::new(),
            subtitle_combiner: SubtitleCombiner::new(),
            progress_bar,
        })
    }

    pub async fn handle_translator(
        &mut self,
        source_language: String,
        target_language: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut final_srt_content = String::with_capacity(self.subtitle_entries.len());
        let chunk_count = self.subtitle_entries.len() / GROUP_SIZE;

        for index in 0..chunk_count {
            final_srt_content.push_str(
                &self
                    .process_chunk(index, &source_language, &target_language)
                    .await?,
            );
            self.progress_bar.inc(1);
        }
        self.progress_bar.finish_with_message("done");

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
) -> Result<(), Box<dyn std::error::Error>> {
    let mut handler = Handler::from_path(path.clone())?;
    let final_srt_content = handler
        .handle_translator(source_language, target_language.clone())
        .await?;

    let output_path = generate_output_path(&path, &target_language);
    Writer::write_file(final_srt_content, output_path)?;
    Ok(())
}
fn generate_output_path(input_path: &PathBuf, target_language: &str) -> PathBuf {
    let file_name = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    input_path.with_file_name(format!("{}_{}.srt", file_name, target_language))
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_handle() {
        let path = std::path::PathBuf::from("test2.srt");

        crate::handler::handle_openai_translate(path, "en".to_string(), "zh_CN".to_string())
            .await
            .unwrap();
    }
}
