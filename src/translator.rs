use crate::{openai::OpenAI, TEMPLATES};

#[derive(Debug)]
pub struct Translator {
    pub translated_result: String,
}
impl Translator {
    pub fn new() -> Self {
        Self {
            translated_result: String::new(),
        }
    }
    pub fn format_user_message(
        &self,
        source_language: &String,
        target_language: &String,
        tagged_text: String,
        chunk_to_translate: String,
    ) -> String {
        let mut context = tera::Context::new();
        context.insert("source_language", &source_language);
        context.insert("target_language", &target_language);
        context.insert("tagged_text", &tagged_text);
        context.insert("chunk_to_translate", &chunk_to_translate);

        let formatted_user_message = match TEMPLATES.render("user_message.txt", &context) {
            Ok(s) => s,
            Err(e) => {
                println!("Error: {}", e);
                let mut cause = std::error::Error::source(&e);
                while let Some(e) = cause {
                    println!("Reason: {}", e);
                    cause = e.source();
                }
                String::new()
            }
        };
        formatted_user_message
    }
    pub fn format_prompt(&self, source_language: &String, target_language: &String) -> String {
        let mut context = tera::Context::new();
        context.insert("source_language", source_language);
        context.insert("target_language", target_language);
        let formatted_prompt = match TEMPLATES.render("prompt.txt", &context) {
            Ok(s) => s,
            Err(e) => {
                println!("Error: {}", e);
                let mut cause = std::error::Error::source(&e);
                while let Some(e) = cause {
                    println!("Reason: {}", e);
                    cause = e.source();
                }
                String::new()
            }
        };
        formatted_prompt
    }
    pub fn format_translated_result() {
        todo!()
    }
    pub async fn translate(
        &self,
        source_language: &String,
        target_language: &String,
        tagged_text: String,
        chunk_to_translate: String,
    ) -> Self {
        let mut translator = Self::new();
        let formatted_user_message = translator.format_user_message(
            source_language,
            target_language,
            tagged_text,
            chunk_to_translate,
        );
        let formatted_prompt = translator.format_prompt(source_language, target_language);
        let openai = OpenAI::new();
        let translated_result = openai
            .chat(formatted_prompt, formatted_user_message)
            .await
            .expect("Error translating text");

        translator.translated_result = translated_result;
        translator
    }
}
mod test {
    #[test]
    fn test_format_user_message() {
        let mock = crate::mock::Mock::new();
        let formatter = crate::formatter::Formatter::format(0, &mock.text_splitter.split_result);
        let translator = crate::translator::Translator::new();
        let formatted_user_message = translator.format_user_message(
            &"en".to_string(),
            &"zh_CN".to_string(),
            formatter.tagged_text,
            formatter.chunk_to_translate,
        );
        eprintln!("{:?}", formatted_user_message);
        assert!(formatted_user_message.len() > 0);
    }
    #[test]
    fn test_format_prompt() {
        let translator = crate::translator::Translator::new();
        let formatted_prompt = translator.format_prompt(&"en".to_string(), &"zh_CN".to_string());
        eprintln!("{:?}", formatted_prompt);
        assert!(formatted_prompt.len() > 0);
    }
    #[tokio::test]
    async fn test_translate() {
        let mock = crate::mock::Mock::new();
        let formatter = crate::formatter::Formatter::format(0, &mock.text_splitter.split_result);
        let translator = crate::translator::Translator::new();
        let source_language = "en".to_string();
        let target_language = "zh_CN".to_string();
        let translator = translator
            .translate(
                &source_language,
                &target_language,
                formatter.tagged_text,
                formatter.chunk_to_translate,
            )
            .await;
        assert!(translator.translated_result.len() > 0);
        eprintln!("{:?}", translator);
    }
}
