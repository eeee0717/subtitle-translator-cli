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
    ) -> (String, usize) {
        // eprintln!("combined_text:{}", combined_text);
        // eprintln!("translated_text:{}", translated_text);
        // eprintln!("time_info: {:?}", time_info);
        // eprintln!("current_index:{}", current_index);
        // eprintln!("number_info: {:?}", number_info);
        let text_lines = combined_text.split("<T>").collect::<Vec<&str>>();
        eprintln!("text_lines: {:?}", text_lines);
        eprintln!("text_lines_len: {:?}", text_lines.len());
        let result_lines = translated_text.split("<T>").collect::<Vec<&str>>();
        eprintln!("result_lines: {:?}", result_lines);
        eprintln!("result_lines_len: {:?}", result_lines.len());
        let mut combined_lines: Vec<String> = vec![];
        for (index, result_line) in result_lines.clone().into_iter().enumerate() {
            combined_lines.push(number_info[current_index + index].clone());
            combined_lines.push(time_info[current_index + index].clone());
            combined_lines.push(result_line.to_string());
            combined_lines.push(text_lines[index].to_string());
            combined_lines.push("".to_string());
        }
        let srt_content = combined_lines.join("\n");
        return (srt_content, current_index + result_lines.len());
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
            19,
            mock.subtitle_extractor.number_info,
        );
        eprintln!("{:?}", subtitle_combiner);
        assert_eq!(subtitle_combiner.1, 39);
    }
}
