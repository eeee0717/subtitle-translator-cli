use std::error::Error;

use regex::Regex;

pub trait SubtitleFile {
    fn extract_information(
        &self,
        text: &String,
    ) -> Result<(Vec<String>, Vec<String>, Vec<String>), Box<dyn Error>>;
    fn split_text(&self, text_info: Vec<String>) -> Result<Vec<String>, Box<dyn Error>>;
    fn format_text(
        &self,
        source_text_chunks: Vec<String>,
        i: usize,
    ) -> Result<(String, String), Box<dyn Error>>;
    fn merge_contents(
        &self,
        contents: &String,
        translated_contents: Vec<String>,
    ) -> Result<String, Box<dyn Error>>;
}

pub struct SrtFile {}

impl SubtitleFile for SrtFile {
    fn extract_information(
        &self,
        text: &String,
    ) -> Result<(Vec<String>, Vec<String>, Vec<String>), Box<dyn Error>> {
        // split "\r\n" or "\n" to get lines
        let re: Regex = Regex::new(r"\r\n|\n").unwrap();
        let lines: Vec<&str> = re.split(text).collect::<Vec<&str>>();
        let time_pattern =
            Regex::new(r"\d{2}:\d{2}:\d{2},\d{3} --> \d{2}:\d{2}:\d{2},\d{3}").unwrap();
        let number_pattern = Regex::new(r"^\d+$").unwrap();
        let mut number_info: Vec<String> = vec![];
        let mut time_info: Vec<String> = vec![];
        let mut text_info: Vec<String> = vec![];
        let mut current_text: Vec<String> = vec![];

        for line in lines {
            let line = line.trim();
            if number_pattern.is_match(line) {
                number_info.push(line.to_string());
            } else if time_pattern.is_match(line) {
                time_info.push(line.to_string());
                if current_text.len() > 0 {
                    text_info.push(current_text.join("\n"));
                    current_text = vec![];
                }
            } else if line == String::from("") {
                continue;
            } else {
                current_text.push(line.to_string());
            }
        }
        if current_text.len() > 0 {
            text_info.push(current_text.join("\n"));
        }

        Ok((number_info, time_info, text_info))
    }

    fn split_text(&self, text_info: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
        let group_size = 20;
        let delimiter = "<T>";
        let mut result: Vec<String> = vec![];
        for i in (0..text_info.len()).step_by(group_size) {
            let end = std::cmp::min(i + group_size, text_info.len());
            result.push(text_info[i..end].join(delimiter));
        }

        Ok(result)
    }

    fn format_text(
        &self,
        source_text_chunks: Vec<String>,
        i: usize,
    ) -> Result<(String, String), Box<dyn Error>> {
        let before = source_text_chunks[0..i].join("");
        let current = format!("<TRANSLATE_THIS>{}</TRANSLATE_THIS>", source_text_chunks[i]);
        let after = source_text_chunks[i + 1..].join("");
        let tagged_text = format!("{}{}{}", before, current, after);

        Ok((tagged_text, source_text_chunks[i].clone()))
    }
    fn merge_contents(
        &self,
        contents: &String,
        translated_contents: Vec<String>,
    ) -> Result<String, Box<dyn Error>> {
        let mut merged_contents = String::new();
        let mut translated_contents = translated_contents.into_iter();
        for line in contents.lines() {
            if is_number(line) || is_timeline(line) || is_empty_line(line) {
                merged_contents.push_str(format!("{}\n", line).as_str());
                continue;
            }
            // add original content
            merged_contents.push_str(format!("{}\n", line).as_str());
            // add translated content
            match translated_contents.next() {
                Some(translated_line) => {
                    merged_contents.push_str(format!("{}\n", translated_line).as_str());
                }
                None => {
                    merged_contents.push_str(format!("{}\n", line).as_str());
                }
            }
        }
        Ok(merged_contents)
    }
}

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
