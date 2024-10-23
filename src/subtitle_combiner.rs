#[derive(Debug)]
pub struct SubtitleCombiner {
    pub srt_content: String,
    pub current_index: usize,
}

impl SubtitleCombiner {
    pub fn new() -> Self {
        Self {
            srt_content: String::new(),
            current_index: 0,
        }
    }
    pub fn combine(
        combined_text: String,
        translated_text: String,
        time_info: Vec<String>,
        current_index: usize,
        number_info: Vec<String>,
    ) -> Self {
        let text_lines = combined_text.split("<T>").collect::<Vec<&str>>();
        let result_lines = translated_text.split("<T>").collect::<Vec<&str>>();
        let mut combined_lines: Vec<String> = vec![];
        for (index, result_line) in result_lines.clone().into_iter().enumerate() {
            combined_lines.push(number_info[current_index + index].clone());
            combined_lines.push(time_info[current_index + index].clone());
            combined_lines.push(result_line.to_string());
            combined_lines.push(text_lines[index].to_string());
            combined_lines.push("".to_string());
        }
        // replace <nl> with newline
        let srt_content = combined_lines.join("\n").replace("<nl>", "\n");
        return Self {
            srt_content,
            current_index: current_index + result_lines.len(),
        };
    }
}
mod test {

    #[test]
    fn test_combine() {
        let mock = crate::mock::Mock::new();
        let subtitle_combiner = crate::subtitle_combiner::SubtitleCombiner::combine(
            mock.formatter.chunk_to_translate,
            mock.translator.translated_result,
            mock.subtitle_extractor.time_info,
            0,
            mock.subtitle_extractor.number_info,
        );
        // eprintln!("{:?}", subtitle_combiner);
        eprintln!("current_index:{}", subtitle_combiner.current_index);
        assert_eq!(subtitle_combiner.current_index, crate::GROUP_SIZE);
    }
}
