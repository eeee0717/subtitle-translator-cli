use subparse::SubtitleEntry;

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
    pub fn extractor(entries: Vec<SubtitleEntry>) -> Self {
        let mut subtitle_extractor = Self::new();
        for (index, entry) in entries.into_iter().enumerate() {
            let time_info = format!("{} --> {}", entry.timespan.start, entry.timespan.end);
            subtitle_extractor.time_info.push(time_info);
            subtitle_extractor
                .text_info
                .push(entry.line.expect("No line found"));
            subtitle_extractor.number_info.push((index + 1).to_string());
        }
        subtitle_extractor
    }
}
mod test {
    #[test]
    fn test_extractor() {
        let path = std::path::PathBuf::from("test.srt");
        let subtitle_entries = crate::parse::parse_file(&path);
        let subtitle_extractor =
            crate::subtitle_extractor::SubtitleExtractor::extractor(subtitle_entries);
        eprintln!("{:?}", subtitle_extractor);
        assert_eq!(subtitle_extractor.number_info.len(), 2)
    }
}
