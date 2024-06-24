use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::{SrtFile, SubtitleFile};
use core::time;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn process_file(
    file_path: String,
    input_language: String,
    output_language: String,
) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(&file_path).expect("Something went wrong reading the file");
    let contents = read_file_trim_bom(&contents);
    let file_extension = file_path.split('.').last().unwrap_or("");

    let file_struct = match file_extension {
        "srt" => Box::new(SrtFile {}),
        _ => return Err("Unsupported file type".into()),
    };

    let split_contents = file_struct.split_contents(&contents).unwrap();
    let translated_combined_text =
        run_translation_tasks(split_contents, input_language, output_language);
    let merged_contents = file_struct.merge_contents(&contents, translated_combined_text);
    let mut file = File::create("output.srt")?;
    file.write_all(merged_contents.as_bytes())?;
    Ok(merged_contents)
}

pub fn read_file_trim_bom(contents: &str) -> String {
    let bom = "\u{FEFF}";
    if contents.starts_with(bom) {
        contents[bom.len()..].to_string()
    } else {
        contents.to_string()
    }
}

fn run_translation_tasks(
    contents: Vec<String>,
    input_language: String,
    output_language: String,
) -> Vec<String> {
    let translated_combined_text = Arc::new(Mutex::new(Vec::with_capacity(contents.len())));

    spawn_translation_threads(
        contents,
        input_language,
        output_language,
        translated_combined_text.clone(),
    );

    let mut translated_combined_text = Arc::try_unwrap(translated_combined_text)
        .expect("Arc unwrap failed")
        .into_inner()
        .expect("Failed to acquire lock");

    sort_and_extract_translations(&mut translated_combined_text)
}

fn spawn_translation_threads(
    contents: Vec<String>,
    input_language: String,
    output_language: String,
    translated_combined_text: Arc<Mutex<Vec<(usize, String)>>>,
) {
    contents
        .into_par_iter()
        .enumerate()
        .for_each(|(index, content)| {
            let translated_text =
                crate::translate(content, input_language.clone(), output_language.clone())
                    .expect("Translation failed");
            let mut translated_combined_text = translated_combined_text
                .lock()
                .expect("Failed to acquire lock");
            translated_combined_text.push((index, translated_text));
            thread::sleep(time::Duration::from_millis(200));
        });
}

fn sort_and_extract_translations(
    translated_combined_text: &mut Vec<(usize, String)>,
) -> Vec<String> {
    translated_combined_text.sort_by_key(|k| k.0);

    translated_combined_text
        .into_iter()
        .map(|(_, text)| text.to_owned())
        .collect()
}
