mod config;
pub use config::*;

mod translator;
pub use translator::*;

mod subtitle_file;
pub use subtitle_file::*;

mod processor;
pub use processor::*;

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;
    use std::{
        fs,
        sync::{Arc, Mutex},
        thread,
        time::Instant,
    };
    #[test]
    fn test_translate() {
        let config = Config {
            file_path: "TEST.txt".to_string(),
            file_name: "TEST.txt".to_string(),
            input_language: "auto".to_string(),
            output_language: "zh-CN".to_string(),
        };
        let contents = fs::read_to_string(&config.file_path).unwrap();

        let translated_text =
            translate(contents, config.input_language, config.output_language).unwrap();
        println!("{:?}", translated_text);
    }

    #[test]
    fn test_regex() {
        let contents = fs::read_to_string("test.srt").unwrap();
        let re = Regex::new(
            r"\d+\r\n\d{2}:\d{2}:\d{2},\d{3} --> \d{2}:\d{2}:\d{2},\d{3}\r\n.*(?:\n.*)?",
        )
        .unwrap();
        let segments: Vec<&str> = re.find_iter(&contents).map(|cap| cap.as_str()).collect();
        println!("{:?}", segments.len());
    }
    #[test]
    fn test_split() {
        let contents = fs::read_to_string("test.srt").unwrap();
        let segments = contents.split("\r\n").collect::<Vec<&str>>();
        assert_eq!("1", segments[0]);
    }
    #[test]
    fn test_single_thread() {
        let start = Instant::now();
        let split_contents = vec![
            "谁拥有香料".to_string(),
            "谁就拥有世界".to_string(),
            "帝国日记 10191年 评论三".to_string(),
            "帝国日记 10191年 评论三".to_string(),
            "帝国日记 10191年 评论三".to_string(),
            "帝国日记 10191年 评论三".to_string(),
        ];
        let mut translated_combined_text = Vec::new();
        for contents in split_contents {
            let translated_text = translate(contents, "zh-CN".to_string(), "en".to_string())
                .expect("Translation failed");
            translated_combined_text.push(translated_text);
        }
        println!("{:?}", translated_combined_text);
        println!("Time elapsed: {:?}", start.elapsed());
    }

    #[test]
    fn test_multi_thread() {
        let start = Instant::now();
        let split_contents = vec![
            "1 谁拥有香料".to_string(),
            "2 谁就拥有世界".to_string(),
            "3 谁就拥有世界".to_string(),
            "4 谁就拥有世界".to_string(),
            "5 谁就拥有世界".to_string(),
            "6 谁就拥有世界".to_string(),
        ];
        let translated_combined_text =
            Arc::new(Mutex::new(Vec::with_capacity(split_contents.len())));
        let mut handles = vec![];

        for (index, content) in split_contents.into_iter().enumerate() {
            // 克隆 Arc 给每个线程
            let translated_combined_text = Arc::clone(&translated_combined_text);
            let handle = thread::spawn(move || {
                let translated_text = translate(content, "zh-CN".to_string(), "en".to_string())
                    .expect("Translation failed");
                // 将翻译结果安全地推入向量
                let mut translated_combined_text = translated_combined_text
                    .lock()
                    .expect("Failed to acquire lock");
                translated_combined_text.push((index, translated_text));
            });

            handles.push(handle);
        }
        for handle in handles {
            handle.join().expect("Thread failed");
        }

        let mut translated_combined_text = Arc::try_unwrap(translated_combined_text)
            .expect("Arc unwrap failed")
            .into_inner()
            .expect("Failed to acquire lock");

        // 按照索引排序以恢复原始顺序
        translated_combined_text.sort_by_key(|k| k.0);

        // 提取翻译后的文本
        let translated_combined_text: Vec<String> = translated_combined_text
            .into_iter()
            .map(|(_, text)| text)
            .collect();

        println!("{:#?}", translated_combined_text);
        println!("Time elapsed: {:?}", start.elapsed());
    }

    #[test]
    fn test_process_file() {
        let config = Config {
            file_path: "test.srt".to_string(),
            file_name: "test.srt".to_string(),
            input_language: "auto".to_string(),
            output_language: "en".to_string(),
        };
        let start = Instant::now();
        let translated_text = process_file(
            config.file_path,
            config.input_language,
            config.output_language,
        )
        .unwrap();
        println!("{:?}", translated_text);
        println!("Time elapsed: {:?}", start.elapsed());
    }
}
