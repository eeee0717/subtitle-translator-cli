use std::path::PathBuf;

use glob::glob;
use indicatif::{ProgressBar, ProgressStyle};

/// Get all subtitle files in a directory
///
/// Parameters:
/// - file_path: String
///
pub fn get_all_files(file_path: &str) -> Result<Vec<PathBuf>, &str> {
    let mut files = Vec::new();

    for entry in glob(file_path).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => files.push(path),
            Err(e) => println!("{:?}", e),
        }
    }

    Ok(files)
}

pub fn read_file_trim_bom(contents: &str) -> String {
    let bom = "\u{FEFF}";
    if contents.starts_with(bom) {
        contents[bom.len()..].to_string()
    } else {
        contents.to_string()
    }
}

pub fn sort_and_extract_translations(
    translated_combined_text: &mut Vec<(usize, String)>,
) -> Vec<String> {
    translated_combined_text.sort_by_key(|k| k.0);

    translated_combined_text
        .into_iter()
        .map(|(_, text)| text.to_owned())
        .collect()
}

pub fn pb_init(total: u64) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}%",
        )
        .unwrap()
        .progress_chars("#>-"),
    );
    pb
}
