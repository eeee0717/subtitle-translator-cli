use crate::{openai::OpenAI, TEMPLATES};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TranslatorError {
    #[error("Template rendering failed: {0}")]
    TemplateError(#[from] tera::Error),
    #[error("Translation failed: {0}")]
    TranslationError(String),
}
type Result<T> = std::result::Result<T, TranslatorError>;

#[derive(Debug, Clone)]
pub struct Translator {
    pub translated_result: String,
}
impl Translator {
    pub fn new() -> Self {
        Self {
            translated_result: String::new(),
        }
    }

    /// 处理模板错误并记录详细信息
    fn handle_template_error(e: tera::Error) -> TranslatorError {
        println!("Error: {}", e);
        let mut cause = std::error::Error::source(&e);
        while let Some(e) = cause {
            println!("Reason: {}", e);
            cause = e.source();
        }
        TranslatorError::TemplateError(e)
    }

    /// 格式化用户消息模板
    pub fn format_user_message(
        &self,
        source_language: &str,
        target_language: &str,
        tagged_text: String,
        chunk_to_translate: String,
    ) -> Result<String> {
        let mut context = tera::Context::new();
        context.insert("source_language", source_language);
        context.insert("target_language", target_language);
        context.insert("tagged_text", &tagged_text);
        context.insert("chunk_to_translate", &chunk_to_translate);

        TEMPLATES
            .render("user_message.txt", &context)
            .map_err(Self::handle_template_error)
    }

    /// 格式化提示模板
    pub fn format_prompt(&self, source_language: &str, target_language: &str) -> Result<String> {
        let mut context = tera::Context::new();
        context.insert("source_language", source_language);
        context.insert("target_language", target_language);

        TEMPLATES
            .render("prompt.txt", &context)
            .map_err(Self::handle_template_error)
    }

    /// 格式化翻译结果
    pub fn format_translated_result(&mut self) -> String {
        let result = self
            .translated_result
            .split("```")
            .filter(|s| !s.trim().is_empty())
            .last()
            .map(|s| s.trim().to_string())
            .unwrap_or_default();

        result
    }

    /// 执行翻译
    pub async fn translate(
        &mut self,
        source_language: &str,
        target_language: &str,
        tagged_text: String,
        chunk_to_translate: String,
    ) -> Result<()> {
        let formatted_user_message = self.format_user_message(
            source_language,
            target_language,
            tagged_text,
            chunk_to_translate,
        )?;

        let formatted_prompt = self.format_prompt(source_language, target_language)?;

        let openai = OpenAI::new();
        let translated_result = openai
            .chat(formatted_prompt, formatted_user_message)
            .await
            .map_err(|e| TranslatorError::TranslationError(e.to_string()))?;

        self.translated_result = translated_result;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formatter::Formatter;
    use futures::future;
    const MIN_SUCCESS_RATE: f64 = 20.0;

    fn setup() -> Formatter {
        let path = std::path::PathBuf::from("test.srt");
        let subtitle_entries = crate::parse::parse_file(&path).expect("Failed to parse file");
        let subtitle_extractor =
            crate::subtitle_extractor::SubtitleExtractor::extractor(&subtitle_entries)
                .expect("Failed to extract subtitle");
        let text_splitter =
            crate::text_splitter::TextSplitter::split_text(&subtitle_extractor.text_info)
                .expect("Failed to split text");
        let formatter = crate::formatter::Formatter::format(0, &text_splitter.split_result);
        formatter
    }
    #[test]
    fn test_format_user_message() {
        let formatter = setup();
        let translator = Translator::new();

        let formatted_user_message = translator.format_user_message(
            "en",
            "zh_CN",
            formatter.tagged_text,
            formatter.chunk_to_translate,
        );

        eprintln!("{:?}", formatted_user_message);
        assert!(formatted_user_message.is_ok());
        assert!(!formatted_user_message.unwrap().is_empty());
    }

    #[test]
    fn test_format_prompt() {
        let translator = Translator::new();

        let formatted_prompt = translator.format_prompt("en", "zh_CN");

        assert!(formatted_prompt.is_ok());
        assert!(!formatted_prompt.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_translate() {
        let formatter = setup();
        let mut translator = Translator::new();

        translator
            .translate(
                "en",
                "zh_CN",
                formatter.tagged_text,
                formatter.chunk_to_translate,
            )
            .await
            .expect("Failed to translate");

        eprintln!("{:?}", translator.translated_result);
        assert!(!translator.translated_result.is_empty());
    }

    pub(crate) async fn test_format_translated_result() -> bool {
        let formatter = setup();
        let mut translator = Translator::new();

        translator
            .translate(
                "en",
                "zh_CN",
                formatter.tagged_text,
                formatter.chunk_to_translate,
            )
            .await
            .expect("Failed to translate");

        let formatted_result = translator.format_translated_result();
        formatted_result.split("<T>").count() == crate::GROUP_SIZE
    }

    #[tokio::test]
    async fn test_format_translated_result_multiple_times() {
        const TOTAL_RUNS: usize = 10;
        let handles: Vec<_> = (0..TOTAL_RUNS)
            .map(|_| test_format_translated_result())
            .collect();

        let results = future::join_all(handles).await;
        let success_count = results.iter().filter(|&&success| success).count();
        let success_rate = (success_count as f64 / TOTAL_RUNS as f64) * 100.0;

        eprintln!("\nSuccess rate: {:.2}%\n", success_rate);
        assert!(
            success_rate >= MIN_SUCCESS_RATE,
            "Success rate {:.2}% is below the acceptable threshold of {:.2}%",
            success_rate,
            MIN_SUCCESS_RATE
        );
    }
}
