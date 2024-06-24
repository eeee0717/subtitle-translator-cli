use std::error::Error;

pub trait SubtitleFile {
    fn split_contents(&self, contents: &String) -> Result<Vec<String>, Box<dyn Error>>;
    fn merge_contents(&self, contents: &String, translated_contents: Vec<String>) -> String;
}

pub struct SrtFile {}

impl SubtitleFile for SrtFile {
    fn split_contents(&self, contents: &String) -> Result<Vec<String>, Box<dyn Error>> {
        let segments = contents.split("\r\n").collect::<Vec<&str>>();
        let extracted_contents = extract_contents(&segments);

        let extracted_strings: Vec<String> = extracted_contents
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        Ok(extracted_strings)
    }

    fn merge_contents(&self, contents: &String, translated_contents: Vec<String>) -> String {
        let mut merged_contents = String::new();
        let mut translated_contents = translated_contents.into_iter();
        for line in contents.lines() {
            if is_number(line) || is_timeline(line) || is_empty_line(line) {
                merged_contents.push_str(format!("{}\n", line).as_str());
                continue;
            }

            if let Some(translated_line) = translated_contents.next() {
                merged_contents.push_str(format!("{}\n", translated_line).as_str());
            } else {
                println!(
                    "Warning: Not enough translated content for all lines in original contents\r\n"
                );
                break;
            }
        }
        merged_contents
    }
}

// fn insert_translated_contents(contents:&String, )->String{

// }

fn extract_contents(segments: &[&str]) -> Vec<String> {
    let mut extracted_contents = Vec::new();
    for &segment in segments {
        if is_number(segment) || is_timeline(segment) || is_empty_line(segment) {
            continue;
        }
        extracted_contents.push(segment.to_string());
    }
    extracted_contents
}

fn is_number(s: &str) -> bool {
    s.chars().all(char::is_numeric)
}

fn is_timeline(s: &str) -> bool {
    s.contains("-->")
}

fn is_empty_line(s: &str) -> bool {
    s.is_empty()
}
