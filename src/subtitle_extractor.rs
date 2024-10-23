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
    pub fn extractor(entries: &Vec<SubtitleEntry>) -> Self {
        let mut subtitle_extractor = Self::new();
        for (index, entry) in entries.into_iter().enumerate() {
            let time_info = format!("{} --> {}", entry.timespan.start, entry.timespan.end);
            // remove \n in the line
            let text_info = entry
                .line
                .clone()
                .expect("No line found")
                .replace("\n", "<nl>");
            subtitle_extractor.time_info.push(time_info);
            subtitle_extractor.text_info.push(text_info);
            subtitle_extractor.number_info.push((index + 1).to_string());
        }
        subtitle_extractor
    }
}
mod test {
    #[test]
    fn test_extractor() {
        let mock = crate::mock::Mock::new();
        let subtitle_extractor =
            crate::subtitle_extractor::SubtitleExtractor::extractor(&mock.subtitle_entries);
        eprintln!("time_info:{:?}", subtitle_extractor.time_info);
        // eprintln!("text_info:{:?}", subtitle_extractor.text_info);
        eprintln!("number_info:{:?}", subtitle_extractor.number_info);
        assert_eq!(subtitle_extractor.number_info.len(), 60)
    }
}
