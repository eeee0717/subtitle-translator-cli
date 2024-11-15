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
        // eprintln!("input\n{:?}", input);
        let combined_text: Vec<&str> = input.combined_text.split("<T>").collect();
        let translated_text: Vec<&str> = input.translated_text.split("<T>").collect();

        if combined_text.len() != translated_text.len() {
            eprintln!(
                "Line {} to {} need manual translation",
                input.number_info[self.current_index],
                input.number_info[self.current_index + combined_text.len() - 1]
            );
        }
        let mut combined_lines = Vec::with_capacity(combined_text.len() * 5);

        for (index, (translated_line, combined_line)) in
            translated_text.iter().zip(combined_text.iter()).enumerate()
        {
            // eprintln!(
            //     "index: {}, translated_line: {}, combined_line: {}",
            //     index, translated_line, combined_line
            // );
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
                entry.push(combined_line.to_string());
            } else {
                entry.extend([
                    translated_line.trim().to_string(),
                    combined_line.to_string(),
                ]);
            }
            entry.push(String::new());
            combined_lines.extend(entry);
        }
        // eprintln!("combined_lines:\n {:?}", combined_lines);
        // add \n in the end
        self.srt_content = combined_lines.join("\n").replace("<nl>", "\n");
        self.srt_content.push('\n');
        // eprintln!("srt_content:\n {:?}", self.srt_content);
        self.current_index += translated_text.len();
        Ok(())
    }
}
