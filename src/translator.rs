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
    #[test]
    fn test_format_translated_result() {
        let mut translator = crate::translator::Translator::new();
        translator.translated_result = String::from("【思考】第一轮直译：将英文直译为中文，确保信息完整，不遗漏任何内容，尽量保持原意。\n\n【翻译】\n你所谓的魔法师到底在哪里？<T>他会来的，兰斯洛特。<T>他发誓过。<T>他们在准备第二波攻击。<T>冲啊！<T>这……这就是末日的样子。<T>你所谓的魔法师，梅林，帮助不了我们。<T>我们的数量被压倒，十比一。<T>我们该如何脱身？<T>他承诺会给我们一件武器，<T>一件强大的武器。<T>亚瑟，你是我的国王。<T>我愿为你献出我的生命，<T>但这个梅林根本不是巫师。<T>他是个无用的酒鬼！<T>哦，天哪，我醉了！<T>再来一口。<T><i>魔法确实存在。</i><T><i>它早在很久以前，</i><T><i>在一艘坠毁的外星飞船里被发现。</i><T>有人在吗？<T>你好！<T>是我，<T>灵魂的召唤者，<T>黑暗魔法的主人！<T>有人在吗？<T>是我，梅林。<T>巫师？<T>还记得我吗？<T>不！我守住了你的秘密！<T>我做到了！我按承诺做了！<T>我没有告诉任何人你的存在。没有人！<T>但是你得明白，我们英国人<T>正在进行一场绝望的斗争。<T>那是一种末世的感觉。我是认真的。<T>这一切正在我们说话的时候，<T>就在下面发生。<T>真可怕！<T>强势人物们相互碰撞，满身是伤。<T>我不想开口……但我们需要你的帮助。<T>没有咒语，也没有魔法。<T>- 我们撤退，活着再战。<T>- 不！<T>没有牺牲……<T>就没有胜利。<T>这太疯狂了！\n\n【思考】第二轮意译：在直译的基础上，将字幕用更通俗易懂的中文进行翻译，保留<T>标识。\n\n【翻译】\n你这个所谓的魔法师到底在哪里？<T>他会来的，兰斯洛特。<T>他发过誓。<T>他们正在准备第二波进攻。<T>冲啊！<T>这……这就是末日的模样。<T>你那个所谓的魔法师，梅林，根本无法帮助我们。<T>我们的人数是敌人的十分之一。<T>我们该怎么才能脱身？<T>他答应给我们一件武器，<T>一件力量巨大的武器。<T>亚瑟，你是我的国王。<T>我愿意为你付出我的生命，<T>但这个梅林根本不是巫师。<T>他是个毫无价值的酒鬼！<T>哦，天哪，我真醉了！<T>再来一口。<T><i>魔法确实存在。</i><T><i>早在很久以前，</i><T><i>就在一艘坠毁的外星飞船里被发现。</i><T>有人在吗？<T>你好！<T>是我，<T>灵魂的召唤者，<T>黑暗魔法的掌控者！<T>有人在吗？<T>是我，梅林。<T>巫师？<T>你还记得我吗？<T>不！我守住了你的秘密！<T>我做到了！我绝对遵守承诺！<T>我没有告诉任何人你的存在。没有人！<T>但是你得明白，我们英国人<T>正在进行一场绝望的斗争。<T>这是末日的感觉。我是认真的。<T>这一切就在我们说话的同时，<T>正在下面发生。<T>真是太可怕了！<T>强势的人物们互相冲突，满身是伤。<T>我不想请求你……但我们真的需要你的帮助。<T>没有咒语，也没有魔法。<T>- 我们撤退，活着再战。<T>- 不！<T>没有牺牲……<T>就没有胜利。<T>这太疯狂了！\n\n【思考】第三轮反思：从准确性、流畅性、风格和术语四个角度分析翻译内容，提出可改进之处。\n\n*准确性*：整体把握和表达很准确，未出现偏差，但在某些地方可以更简洁一些。\n*流畅性*：短句表现合理，但长句的表达还可以更为口语化和简易化，使内容更加自然。\n*风格*：整体风格符合原文，但可以增加一些口语化的表达，来匹配更轻松的说话氛围。\n*术语*：术语使用一致性良好，但“黑暗魔法的掌控者”可以换为“黑暗魔法大师”更为简洁。\n\n【建议】\n1. *准确性*：调整“毫无价值的酒鬼！”为“没用的酒鬼！”，更加口语化。\n2. *流畅性*：将“我愿意为你付出我的生命”改为“我愿为你献身”，表达更加简洁流畅。\n3. *风格*：增加一些口语化表达，诸如将“我不想请求你”改为“我不想麻烦你”。\n4. *术语*：将“黑暗魔法的掌控者！”改为“黑暗魔法大师！”，简化语句。\n\n【思考】第四轮提升：根据反馈进行更进一步的完善，形成最终译文。\n\n```\n你这个所谓的魔法师到底在哪里？<T>他会来的，兰斯洛特。<T>他发过誓。<T>他们正在准备第二波进攻。<T>冲啊！<T>这……这就是末日的模样。<T>你那个所谓的魔法师，梅林，根本无法帮助我们。<T>我们的人数是敌人的十分之一。<T>我们该怎么才能脱身？<T>他答应给我们一件武器，<T>一件力量巨大的武器。<T>亚瑟，你是我的国王。<T>我愿为你献身，<T>但这个梅林根本不是巫师。<T>他是个没用的酒鬼！<T>哦，天哪，我真醉了！<T>再来一口。<T><i>魔法确实存在。</i><T><i>早在很久以前，</i><T><i>就在一艘坠毁的外星飞船里被发现。</i><T>有人在吗？<T>你好！<T>是我，<T>灵魂的召唤者，<T>黑暗魔法大师！<T>有人在吗？<T>是我，梅林。<T>巫师？<T>你还记得我吗？<T>不！我守住了你的秘密！<T>我做到了！我绝对遵守承诺！<T>我没有告诉任何人你的存在。没有人！<T>但是你得明白，我们英国人<T>正在进行一场绝望的斗争。<T>这是末日的感觉。我是认真的。<T>这一切正在我们说话的时候，<T>在下面发生。<T>实在太可怕了！<T>强势的人物们互相冲突，满身是伤。<T>我不想麻烦你……但我们真的需要你的帮助。<T>没有咒语，也没有魔法。<T>- 我们撤退，活着再战。<T>- 不！<T>没有牺牲……<T>就没有胜利。<T>这太疯狂了！\n```");
        let translator = translator.format_translated_result();
        eprintln!("{:?}", translator.translated_result);
        assert!(translator.translated_result.len() > 0);
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
