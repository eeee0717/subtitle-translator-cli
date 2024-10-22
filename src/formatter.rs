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
    fn test_format() {}
}
