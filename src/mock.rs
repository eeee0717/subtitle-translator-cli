use std::path::PathBuf;

use subparse::SubtitleEntry;

use crate::{
    parse::parse_file, subtitle_extractor::SubtitleExtractor, text_splitter::TextSplitter,
};

pub struct Mock {
    pub path: PathBuf,
    pub subtitle_entries: Vec<SubtitleEntry>,
    pub subtitle_extractor: SubtitleExtractor,
    pub text_splitter: TextSplitter,
}

impl Mock {
    pub fn new() -> Self {
        let path = PathBuf::from("test.srt");
        let subtitle_entries = parse_file(&path);
        let subtitle_extractor = SubtitleExtractor::extractor(&subtitle_entries);
        let text_splitter = TextSplitter::split_text(&subtitle_extractor.text_info);
        Self {
            path,
            subtitle_entries,
            subtitle_extractor,
            text_splitter,
        }
    }
}
