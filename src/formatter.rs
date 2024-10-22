#[derive(Debug)]
pub struct Formatter {
    pub tagged_text: String,
    pub chunk_to_translate: String,
}
impl Formatter {
    pub fn new() -> Self {
        Self {
            tagged_text: String::new(),
            chunk_to_translate: String::new(),
        }
    }
    pub fn format(index: usize, source_text_chunks: &Vec<String>) -> Self {
        let mut formatter = Formatter::new();
        let before = source_text_chunks[0..index].join("");
        let current = format!(
            "<TRANSLATE_THIS>{}</TRANSLATE_THIS>",
            source_text_chunks[index]
        );
        let after = source_text_chunks[index + 1..].join("");
        let tagged_text = format!("{}{}{}", before, current, after);
        formatter.tagged_text = tagged_text;
        formatter.chunk_to_translate = source_text_chunks[index].clone();
        formatter
    }
}

mod test {
    #[test]
    fn test_format() {
        let mock = crate::mock::Mock::new();
        let formatter = crate::formatter::Formatter::format(0, &mock.text_splitter.split_result);
        // eprintln!("{:?}", formatter);
        eprintln!("tagged_text:{}", formatter.tagged_text);
        eprintln!("chunk_to_translate:{}", formatter.chunk_to_translate);
        assert_eq!(formatter.chunk_to_translate.len(), 1263);
    }
}
