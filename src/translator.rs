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
    pub fn format_translated_result(&mut self) -> Self {
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
            self.translated_result = result[result.len() - 1].trim().to_string();
        }
        Self {
            translated_result: self.translated_result.clone(),
        }
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
        translator.translated_result = String::from("【思考】本轮是对源文本的第一轮直译，目标是忠实地将内容逐句翻译成中文，保证信息的完整性。\n\n【翻译】\n在哪里见鬼<nl>你的所谓魔法师？<T>他会来的，兰斯洛特。<T>他发誓过。<T>他们正在准备第二波攻击。<T>冲锋！<T>这……这就是终结的模样。<T>你的所谓魔法师，<nl>梅林，帮不了我们。<T>我们人数以百对一。<T>我们该如何摆脱这个？<T>他承诺会有武器，<nl>一种强大的武器。<T>亚瑟，你是我的国王。<T>我愿为你献出生命，<T>但这个梅林根本不是巫师。<T>他是个无用的酒鬼！<T>哦，天啊，我喝醉了！<T>再来一口。<T><i>魔法确实存在。</i><T><i>它很久以前被发现，</i><T><i>就在一艘坠毁的外星飞船里。</i><T>你好？\n\n【思考】第二轮意译是在第一轮直译的基础上，使语言更通顺流畅，同时保留原有的信息。\n\n【翻译】\n你的所谓魔法师到底在哪里？<T>他会来的，兰斯洛特。<T>他发过誓。<T>他们在准备第二波攻击。<T>冲！<T>这……这就是末日的样子。<T>你的所谓魔法师，<nl>梅林，无法帮助我们。<T>我们的敌人多我们100倍！<T>我们怎么才能脱身？<T>他答应过会给我们一个武器，<nl>一件强大的武器。<T>亚瑟，你是我的国王。<T>我愿意为你献出生命，<T>但这个梅林根本不是个巫师。<T>他只是个无用的酒徒！<T>哦，天啊，我醉得不轻！<T>再来一口。<T><i>魔法确实存在。</i><T><i>很久以前被发现于，</i><T><i>一艘坠毁的外星飞船。</i><T>你好？\n\n【思考】本轮反思主要是检查意译的准确性、流畅性、风格和术语使用。\n\n*准确性*：翻译基本准确，没有明显遗漏或错误，但可以进一步简化某些表达。  \n*流畅性*：整体表达较流畅，但个别句子可以更口语化。  \n*风格*：基本符合源文本的风格，但在一些情感的传达上可以稍作增强。  \n*术语*：术语使用较为一致，注意“巫师”和“魔法师”的调用。\n\n【建议】\n- 确保“魔法师”和“巫师”的用词统一性。\n- 在某些表达上更自然口语化，例如“再来一口”可以轻松一点。\n- 在表达情感反应时，让角色的挫折感更明显。\n\n【思考】本轮提升根据建议对译文进行修改，提高语言的流畅度和情感的传达。\n\n``` \n你的所谓魔法师到底在哪里？<T>他马上就会来，兰斯洛特。<T>他发过誓。<T>他们在准备第二波攻击。<T>冲！<T>这……这就是末日的样子。<T>你的所谓魔法师，<nl>梅林，根本帮不了我们。<T>我们的人数多达敌方的100倍！<T>我们该怎么脱身？<T>他承诺会给我们一个武器，<nl>一件强大的武器。<T>亚瑟，你是我的国王。<T>我愿为你献出生命，<T>但是这个梅林根本不是个巫师。<T>他只不过是个废物酒鬼！<T>哦，天啊，我醉得不轻！<T>再来一口。<T><i>魔法确实存在。</i><T><i>很久以前就在，</i><T><i>一艘坠毁的外星飞船里被发现。</i><T>你好？\n```");
        let translator = translator.format_translated_result();
        // eprintln!("{:?}", translator.translated_result);
        for (index, item) in translator.translated_result.split("<T>").enumerate() {
            eprintln!("{}:{}", index, item);
        }
        assert!(translator.translated_result.split("<T>").count() == crate::GROUP_SIZE);
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
        eprintln!("{:?}", translator);
        assert!(translator.translated_result.len() > 0);
    }
}
