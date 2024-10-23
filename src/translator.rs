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
    pub fn format_translated_result(&self) -> Self {
        let mut translator = Self::new();
        let result: Vec<&str> = self
            .translated_result
            .split("```")
            .filter_map(|item| {
                let trimmed = item.trim(); // 去除前后空白
                if !trimmed.is_empty() {
                    Some(trimmed) // 如果非空则返回 Some
                } else {
                    None // 否则返回 None
                }
            })
            .collect();
        if !result[result.len() - 1].is_empty() {
            translator.translated_result = result[result.len() - 1].trim().to_string();
        }
        translator
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
        // eprintln!("{:?}", formatted_user_message);
        translator
    }
}
mod test {
    #[test]
    fn test_format_user_message() {
        let mock = crate::mock::Mock::new();
        let formatter = crate::formatter::Formatter::format(1, &mock.text_splitter.split_result);
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
    #[test]
    fn test_format_translated_result() {
        let mut translator = crate::translator::Translator::new();
        translator.translated_result = String::from("【思考】在这一轮中，我将对文本进行逐句的直译，以忠实传达原文的意思。\n\n【翻译】\n你好！<T>是我，<T>灵魂的召唤者，<T>黑暗艺术的大师！<T>有人在吗？<T>是我，梅林。<T>巫师？<T>还记得我吗？<T>不！我保守了你的秘密！<T>我确实保守了！<T>就像我承诺的那样！<T>我没有告诉任何人你的存在。没有人！<T>但是你必须理解，我们英国人<T>正在进行一场绝望的斗争。<T>一种末日般的事情。我是认真的。<T>这正发生在我们说话的同时。<T>真是糟糕！<T>大人物们简直就像互相碰撞，鲜血淋漓。<T>我不想要求你……但是我们需要你的帮助。<T>没有咒语。没有魔法。<T>- 我们撤退。活着再战。<T>- 不！<T>没有牺牲……<T>就不会有胜利。<T>这太疯狂了！ \n\n【思考】这一轮的重点是将原文意图准确传达，同时保持流畅性和准确性。以下是对初步翻译的反思和建议。\n\n【建议】\n*准确性*;“保守了！”视为强调，保持不变。\n*流畅性*;“就像我承诺的那样！”可以更口语化，建议用“就像我说的那样！”。\n*风格*;对于“巫师？”，使用更常见的中文“你是巫师吗？”更贴切。\n*术语*;“灵魂的召唤者”可能更口语化地翻译成“灵魂召唤者”。\n\n【思考】根据第三轮的建议，我将重新修改翻译，以确保其准确流畅且符合中文表达的习惯。\n\n``` \n你好！<T>我就是，<T>灵魂召唤者，<T>黑暗艺术的大师！<T>有人在吗？<T>是我，梅林。<T>你是巫师吗？<T>还记得我吗？<T>不！我保守了你的秘密！<T>我真的保持了！<T>就像我说的那样！<T>我没有告诉任何人你的存在。没有人！<T>但是你得理解，我们英国人<T>正在进行一场绝望的斗争。<T>就是那种末日的事情。我是认真的。<T>这事正在我们说话的同时发生。<T>真糟糕！<T>那些大人物们简直就像互相冲突，血淋淋的。<T>我不想开口……但是我们需要你的帮助。<T>没有咒语。没有魔法。<T>- 我们撤退。活着再战。<T>- 不！<T>没有牺牲……<T>就不会有胜利。<T>这太疯狂了！\n```");
        let translator = translator.format_translated_result();
        // eprintln!("{:?}", translator.translated_result);
        for (index, item) in translator.translated_result.split("<T>").enumerate() {
            eprintln!("{}:{}", index, item);
        }
        assert!(translator.translated_result.len() > 0);
    }
    #[tokio::test]
    async fn test_translate() {
        let mock = crate::mock::Mock::new();
        let formatter = crate::formatter::Formatter::format(1, &mock.text_splitter.split_result);
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
