use thiserror::Error;

#[derive(Error, Debug)]
pub enum SubtitleError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

#[derive(Debug)]
pub struct CombineInput {
    pub combined_text: String,
    pub translated_text: String,
    pub time_info: Vec<String>,
    pub number_info: Vec<String>,
}

#[derive(Debug)]
pub struct SubtitleCombiner {
    srt_content: String,
    current_index: usize,
}

impl SubtitleCombiner {
    pub fn new() -> Self {
        Self {
            srt_content: String::new(),
            current_index: 0,
        }
    }

    pub fn current_index(mut self, value: usize) -> Self {
        self.current_index = value;
        self
    }

    /// 获取当前处理的索引
    pub fn get_current_index(&self) -> usize {
        self.current_index
    }

    /// 获取合并后的字幕内容
    pub fn get_content(&self) -> &str {
        &self.srt_content
    }

    /// 合并字幕内容
    ///
    /// combined_text: 原文
    /// translated_text: 翻译后的文本
    pub fn combine(&mut self, input: CombineInput) -> Result<(), SubtitleError> {
        println!("input\n{:?}", input);
        let combined_text: Vec<&str> = input.combined_text.split("<T>").collect();
        let translated_text: Vec<&str> = input.translated_text.split("<T>").collect();

        let mut combined_lines = Vec::with_capacity(combined_text.len() * 5);

        for (index, (translated_line, combined_line)) in
            translated_text.iter().zip(combined_text.iter()).enumerate()
        {
            println!(
                "index: {}, translated_line: {}, combined_line: {}",
                index, translated_line, combined_line
            );
            let current_pos = self.current_index + index;

            if current_pos >= input.number_info.len() || current_pos >= input.time_info.len() {
                return Err(SubtitleError::InvalidInput(
                    "Index out of bounds for number_info or time_info".to_string(),
                ));
            }

            let mut entry = vec![
                input.number_info[current_pos].clone(),
                input.time_info[current_pos].clone(),
            ];
            // 如果文本行和结果行数量不匹配，则不合并翻译后的文本
            if combined_text.len() != translated_text.len() {
                eprintln!(
                    "Combined text and translated text length mismatch, {}, {}",
                    translated_text.len(),
                    combined_text.len()
                );
                entry.push(combined_line.to_string());
            } else {
                entry.extend([translated_line.to_string(), combined_line.to_string()]);
            }
            entry.push(String::new());
            combined_lines.extend(entry);
        }

        self.srt_content = combined_lines.join("\n").replace("<nl>", "\n");
        self.current_index += translated_text.len();
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_combine() {
        let path = std::path::PathBuf::from("test.srt");
        let subtitle_entries = crate::parse::parse_file(&path).expect("Failed to parse file");
        let subtitle_extractor =
            crate::subtitle_extractor::SubtitleExtractor::extractor(&subtitle_entries)
                .expect("Failed to extract subtitle");
        let text_splitter =
            crate::text_splitter::TextSplitter::split_text(&subtitle_extractor.text_info)
                .expect("Failed to split text");
        let formatter = crate::formatter::Formatter::format(1, &text_splitter.split_result);
        let mut translator = crate::translator::Translator::new();
        translator.translated_result = String::from("【思考】本轮是对源文本的第一轮直译，目标是忠实地将内容逐句翻译成中文，保证信息的完整性。\n\n【翻译】\n在哪里见鬼<nl>你的所谓魔法师？<T>他会来的，兰斯洛特。<T>他发誓过。<T>他们正在准备第二波攻击。<T>冲锋！<T>这……这就是终结的模样。<T>你的所谓魔法师，<nl>梅林，帮不了我们。<T>我们人数以百对一。<T>我们该如何摆脱这个？<T>他承诺会有武器，<nl>一种强大的武器。<T>亚瑟，你是我的国王。<T>我愿为你献出生命，<T>但这个梅林根本不是巫师。<T>他是个无用的酒鬼！<T>哦，天啊，我喝醉了！<T>再来一口。<T><i>魔法确实存在。</i><T><i>它很久以前被发现，</i><T><i>就在一艘坠毁的外星飞船里。</i><T>你好？\n\n【思考】第二轮意译是在第一轮直译的基础上，使语言更通顺流畅，同时保留原有的信息。\n\n【翻译】\n你的所谓魔法师到底在哪里？<T>他会来的，兰斯洛特。<T>他发过誓。<T>他们在准备第二波攻击。<T>冲！<T>这……这就是末日的样子。<T>你的所谓魔法师，<nl>梅林，无法帮助我们。<T>我们的敌人多我们100倍！<T>我们怎么才能脱身？<T>他答应过会给我们一个武器，<nl>一件强大的武器。<T>亚瑟，你是我的国王。<T>我愿意为你献出生命，<T>但这个梅林根本不是个巫师。<T>他只是个无用的酒徒！<T>哦，天啊，我醉得不轻！<T>再来一口。<T><i>魔法确实存在。</i><T><i>很久以前被发现于，</i><T><i>一艘坠毁的外星飞船。</i><T>你好？\n\n【思考】本轮反思主要是检查意译的准确性、流畅性、风格和术语使用。\n\n*准确性*：翻译基本准确，没有明显遗漏或错误，但可以进一步简化某些表达。  \n*流畅性*：整体表达较流畅，但个别句子可以更口语化。  \n*风格*：基本符合源文本的风格，但在一些情感的传达上可以稍作增强。  \n*术语*：术语使用较为一致，注意“巫师”和“魔法师”的调用。\n\n【建议】\n- 确保“魔法师”和“巫师”的用词统一性。\n- 在某些表达上更自然口语化，例如“再来一口”可以轻松一点。\n- 在表达情感反应时，让角色的挫折感更明显。\n\n【思考】本轮提升根据建议对译文进行修改，提高语言的流畅度和情感的传达。\n\n``` \n你的所谓魔法师到底在哪里？<T>他马上就会来，兰斯洛特。<T>他发过誓。<T>他们在准备第二波攻击。<T>冲！<T>这……这就是末日的样子。<T>你的所谓魔法师，<nl>梅林，根本帮不了我们。<T>我们的人数多达敌方的100倍！<T>我们该怎么脱身？<T>他承诺会给我们一个武器，<nl>一件强大的武器。<T>亚瑟，你是我的国王。<T>我愿为你献出生命，<T>但是这个梅林根本不是个巫师。<T>他只不过是个废物酒鬼！<T>哦，天啊，我醉得不轻！<T>再来一口。<T><i>魔法确实存在。</i><T><i>很久以前就在，</i><T><i>一艘坠毁的外星飞船里被发现。</i><T>你好？\n```");
        let formatted_reslult = translator.format_translated_result();
        let mut combiner = SubtitleCombiner::new();
        let input = CombineInput {
            combined_text: formatter.chunk_to_translate,
            translated_text: formatted_reslult,
            time_info: subtitle_extractor.time_info,
            number_info: subtitle_extractor.number_info,
        };

        combiner.combine(input).expect("Combine should succeed");

        assert_eq!(combiner.get_current_index(), crate::GROUP_SIZE);
    }
}
