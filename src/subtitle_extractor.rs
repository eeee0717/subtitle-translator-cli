use subparse::SubtitleEntry;

/// 字幕提取器，用于存储和处理字幕信息
#[derive(Debug)]
pub struct SubtitleExtractor {
    pub time_info: Vec<String>,
    pub text_info: Vec<String>,
    pub number_info: Vec<String>,
}

impl SubtitleExtractor {
    pub fn new() -> Self {
        Self {
            time_info: Vec::new(),
            text_info: Vec::new(),
            number_info: Vec::new(),
        }
    }
    /// 格式化时间信息
    fn format_time_info(entry: &SubtitleEntry) -> String {
        format!("{} --> {}", entry.timespan.start, entry.timespan.end)
    }

    /// 处理文本信息，替换换行符
    fn process_text_info(entry: &SubtitleEntry) -> Option<String> {
        entry.line.clone().map(|text| text.replace("\n", "<nl>"))
    }

    /// 从字幕条目提取信息
    pub fn extractor(entries: &Vec<SubtitleEntry>) -> Result<Self, String> {
        let mut subtitle_extractor = Self::new();

        for (index, entry) in entries.iter().enumerate() {
            let time_info = Self::format_time_info(entry);

            let text_info = Self::process_text_info(entry)
                .ok_or_else(|| format!("No line found at index {}", index))?;

            subtitle_extractor.time_info.push(time_info);
            subtitle_extractor.text_info.push(text_info);
            subtitle_extractor.number_info.push((index + 1).to_string());
        }

        Ok(subtitle_extractor)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::parse;

    #[test]
    fn test_extractor_success() {
        let path = std::path::PathBuf::from("test.srt");
        let subtitle_entries = parse::parse_file(&path).expect("Failed to parse file");
        let result = SubtitleExtractor::extractor(&subtitle_entries);

        assert!(result.is_ok());
        let extractor = result.unwrap();
        assert_eq!(extractor.number_info.len(), 60);
        assert_eq!(extractor.time_info.len(), 60);
        assert_eq!(extractor.text_info.len(), 60);
    }

    #[test]
    fn test_format_time_info() {
        let path = std::path::PathBuf::from("test.srt");
        let subtitle_entries = parse::parse_file(&path).expect("Failed to parse file");
        let entry = &subtitle_entries[0];
        let time_info = SubtitleExtractor::format_time_info(entry);
        assert_eq!(time_info, "0:01:34.095 --> 0:01:36.180");
    }

    #[test]
    fn test_process_text_info() {
        let path = std::path::PathBuf::from("test.srt");
        let subtitle_entries = parse::parse_file(&path).expect("Failed to parse file");
        let entry = &subtitle_entries[0];
        let text_info = SubtitleExtractor::process_text_info(entry);
        assert_eq!(
            text_info,
            Some("Where in hell is<nl>your so-called magician?".to_string())
        );
    }
}
