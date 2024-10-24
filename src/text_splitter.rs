use crate::GROUP_SIZE;
/// 文本分割器，用于将文本按组分割
#[derive(Debug)]
pub struct TextSplitter {
    pub split_result: Vec<String>,
}

/// 分隔符常量
const DELIMITER: &str = "<T>";

impl TextSplitter {
    /// 创建一个新的文本分割器实例
    pub fn new() -> Self {
        Self {
            split_result: Vec::new(),
        }
    }
    /// 将文本按组大小分割
    ///
    /// # Arguments
    /// * `text_info` - 要分割的文本数组
    ///
    /// # Returns
    /// * `Ok(TextSplitter)` - 成功时返回分割结果
    /// * `Err(String)` - 失败时返回错误信息
    ///
    /// # Errors
    /// - 当输入文本为空时返回错误
    /// - 当分割后的组为空时返回错误
    pub fn split_text(text_info: &[String]) -> Result<Self, String> {
        if text_info.is_empty() {
            return Err("输入文本不能为空".to_string());
        }

        let mut text_splitter = TextSplitter::new();

        for chunk in text_info.chunks(GROUP_SIZE) {
            let group_text = chunk.join(DELIMITER);
            text_splitter.split_result.push(group_text);
        }

        if text_splitter.split_result.is_empty() {
            return Err("分割后的组不能为空".to_string());
        }

        Ok(text_splitter)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_text_basic() {
        let path = std::path::PathBuf::from("test.srt");
        let subtitle_entries = crate::parse::parse_file(&path).expect("Failed to parse file");
        let subtitle_extractor =
            crate::subtitle_extractor::SubtitleExtractor::extractor(&subtitle_entries)
                .expect("Failed to extract subtitle");

        let text_splitter = TextSplitter::split_text(&subtitle_extractor.text_info).unwrap();

        assert_eq!(
            text_splitter.split_result.len(),
            subtitle_entries.len() / GROUP_SIZE
        );
    }
}
