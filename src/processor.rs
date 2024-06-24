use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

// processor.rs
use crate::{SrtFile, SubtitleFile};
use std::error::Error;
use std::fs;
use std::sync::{Arc, Mutex};

fn read_file_trim_bom(contents: &str) -> String {
    let bom = "\u{FEFF}";
    if contents.starts_with(bom) {
        contents[bom.len()..].to_string()
    } else {
        contents.to_string()
    }
}

pub fn process_file(
    file_path: String,
    input_language: String,
    output_language: String,
) -> Result<Vec<String>, Box<dyn Error>> {
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
    Ok(translated_combined_text)
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
