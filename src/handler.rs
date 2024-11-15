use futures::StreamExt;
use indicatif::{ProgressState, ProgressStyle};
use subparse::SubtitleEntry;

use crate::{
    formatter::Formatter, subtitle_combiner::SubtitleCombiner,
    subtitle_extractor::SubtitleExtractor, text_splitter::TextSplitter, translator::Translator,
    writer::Writer, GROUP_SIZE,
};
use std::{fmt::Write, future::Future, path::PathBuf};

pub struct Handler {
    subtitle_entries: Vec<SubtitleEntry>,
    subtitle_extractor: SubtitleExtractor,
    text_splitter: TextSplitter,
    subtitle_combiner: SubtitleCombiner,
    progress_bar: indicatif::ProgressBar,
}

impl Handler {
    pub fn from_path(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let subtitle_entries = crate::parse::parse_file(&path)?;
        Self::new(subtitle_entries)
    }

    pub fn new(subtitle_entries: Vec<SubtitleEntry>) -> Result<Self, Box<dyn std::error::Error>> {
        let subtitle_extractor = SubtitleExtractor::extractor(&subtitle_entries)?;
        let text_splitter = TextSplitter::split_text(&subtitle_extractor.text_info)?;
        let progress_bar = indicatif::ProgressBar::new(
            ((subtitle_entries.len() + GROUP_SIZE - 1) / GROUP_SIZE)
                .try_into()
                .unwrap(),
        );
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
            subtitle_combiner: SubtitleCombiner::new(),
            progress_bar,
        })
    }

    pub async fn handle_translator(
        &mut self,
        source_language: String,
        target_language: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // if the length of subtitle_entries is not divisible by GROUP_SIZE, then round up
        let chunk_count = (self.subtitle_entries.len() + GROUP_SIZE - 1) / GROUP_SIZE;

        let tasks = self.create_translation_tasks(chunk_count, &source_language, &target_language);
        let results = self
            .execute_translation_tasks(tasks, chunk_count)
            .await
            .expect("Failed to execute translation tasks");
        let final_srt_content = self.combine_translation_results(results)?;

        self.progress_bar.finish_with_message("done");
        Ok(final_srt_content)
    }

    fn create_translation_tasks(
        &self,
        chunk_count: usize,
        source_language: &str,
        target_language: &str,
    ) -> Vec<impl Future<Output = Result<(usize, String, String), String>>> {
        (0..chunk_count)
            .map(|index| {
                let formatter = Formatter::format(index, &self.text_splitter.split_result);
                let source_lang = source_language.to_string();
                let target_lang = target_language.to_string();
                let mut translator = Translator::new();

                async move {
                    let result = translator
                        .translate(
                            &source_lang,
                            &target_lang,
                            formatter.tagged_text,
                            formatter.chunk_to_translate.clone(),
                        )
                        .await;

                    match result {
                        Ok(_) => Ok((
                            index,
                            translator.format_translated_result(),
                            formatter.chunk_to_translate,
                        )),
                        Err(e) => Err(e.to_string()),
                    }
                }
            })
            .collect()
    }

    /// use multiple tasks to translate the text
    async fn execute_translation_tasks(
        &mut self,
        tasks: Vec<impl Future<Output = Result<(usize, String, String), String>>>,
        chunk_count: usize,
    ) -> Result<Vec<(usize, String, String)>, Box<dyn std::error::Error>> {
        let mut results = Vec::with_capacity(chunk_count);
        let stream = futures::stream::iter(tasks).buffer_unordered(10);
        tokio::pin!(stream);

        while let Some(result) = stream.next().await {
            let (index, translated_text, chunk_to_translate) = result?;
            results.push((index, translated_text, chunk_to_translate));
            self.progress_bar.inc(1);
        }

        results.sort_by_key(|(index, _, _)| *index);
        Ok(results)
    }

    fn combine_translation_results(
        &mut self,
        results: Vec<(usize, String, String)>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut final_srt_content = String::with_capacity(self.subtitle_entries.len());

        for (_, translated_text, chunk_to_translate) in results {
            let input = crate::subtitle_combiner::CombineInput {
                combined_text: chunk_to_translate,
                translated_text,
                time_info: self.subtitle_extractor.time_info.clone(),
                number_info: self.subtitle_extractor.number_info.clone(),
            };

            self.subtitle_combiner.combine(input)?;
            final_srt_content.push_str(&self.subtitle_combiner.get_content());
        }

        Ok(final_srt_content)
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
        let path = std::path::PathBuf::from("example/WOLFS.en.srt");

        crate::handler::handle_openai_translate(path, "en".to_string(), "zh_CN".to_string())
            .await
            .unwrap();
        eprintln!("done");
    }
}
