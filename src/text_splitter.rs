#[derive(Debug)]
pub struct TextSplitter {
    pub split_result: Vec<String>,
}
const GROUP_SIZE: usize = 20;
impl TextSplitter {
    pub fn new() -> Self {
        Self {
            split_result: Vec::new(),
        }
    }
    pub fn split_text(text_info: &Vec<String>) -> Self {
        let delimiter = "<T>";
        let mut text_splitter = TextSplitter::new();

        for i in (0..text_info.len()).step_by(GROUP_SIZE) {
            let end_index = (i + GROUP_SIZE).min(text_info.len()); // 确保不会超出范围
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
        let mock = crate::mock::Mock::new();
        let text_splitter =
            crate::text_splitter::TextSplitter::split_text(&mock.subtitle_extractor.text_info);
        eprintln!("{:?}", text_splitter.split_result[0]);
        assert_eq!(text_splitter.split_result.len(), 3)
    }
}
