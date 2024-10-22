#[derive(Debug)]
pub struct TextSplitter {
    pub split_result: Vec<String>,
}
impl TextSplitter {
    pub fn new() -> Self {
        Self {
            split_result: Vec::new(),
        }
    }
    pub fn split_text(text_info: &Vec<String>) -> Self {
        let group_size = 40;
        let delimiter = "<T>";
        let mut text_splitter = TextSplitter::new();

        for i in (0..text_info.len()).step_by(group_size) {
            let end_index = (i + group_size).min(text_info.len()); // 确保不会超出范围
            let group = &text_info[i..end_index];
            let group_text = group.join(delimiter);
            text_splitter.split_result.push(group_text);
        }
        text_splitter
    }
}

mod test {
    #[test]
    fn test_split_text() {
        let path = std::path::PathBuf::from("test.srt");
        let subtitle_entries = crate::parse::parse_file(&path);
        let subtitle_extractor =
            crate::subtitle_extractor::SubtitleExtractor::extractor(subtitle_entries);
        let text_splitter =
            crate::text_splitter::TextSplitter::split_text(&subtitle_extractor.text_info);
        eprintln!("{:?}", text_splitter.split_result[0]);
        assert_eq!(text_splitter.split_result.len(), 2)
    }
}
