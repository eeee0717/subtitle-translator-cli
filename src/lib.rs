mod args;
pub use args::*;

mod config;
pub use config::*;

mod translator;
pub use translator::*;

mod subtitle_file;
pub use subtitle_file::*;

mod processor;
pub use processor::*;

mod utils;
pub use utils::*;

mod openai;
pub use openai::*;
#[cfg(test)]
mod tests {

    use super::*;
    use std::{fs, time::Instant};
    #[test]
    fn test_translate() {
        let config = Config {
            file_path: "TEST.txt".to_string(),
            file_name: "TEST.txt".to_string(),
            input_language: "auto".to_string(),
            output_language: "zh-CN".to_string(),
        };
        let contents = fs::read_to_string(&config.file_path).unwrap();

        let translated_text = translate(contents, config.input_language, config.output_language);
        println!("{:?}", translated_text);
    }

    #[test]
    fn test_process_files() {
        let config = Config {
            file_path: "test.srt".to_string(),
            file_name: "//".to_string(),
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
