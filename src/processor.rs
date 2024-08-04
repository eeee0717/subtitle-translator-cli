use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::{
    get_all_files, pb_init, read_file_trim_bom, sort_and_extract_translations, SrtFile,
    SubtitleFile,
};
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
) -> Result<(), Box<dyn Error>> {
    // when file_path == "*.srt", should get all files in the current directory
    let mut files = Vec::new();
    if file_path.contains("*") {
        files = get_all_files(file_path.as_str()).unwrap();
    } else {
        files.push(std::path::PathBuf::from(file_path));
    }
    for file in files {
        process_single_file(
            file.to_str().unwrap(),
            input_language.clone(),
            output_language.clone(),
        )
        .unwrap();
    }
    Ok(())
}

fn process_single_file(
    file_path: &str,
    input_language: String,
    output_language: String,
) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(&file_path).expect("Something went wrong reading the file");
    let text = read_file_trim_bom(&text);
    let file_name = file_path.split('.').next().unwrap_or("");
    let file_extension = file_path.split('.').last().unwrap_or("");

    let file_struct = match file_extension {
        "srt" => Box::new(SrtFile {}),
        _ => return Err("Unsupported file type".into()),
    };

    let (number_info, time_info, text_info) = file_struct.extract_information(text).unwrap();
    // println!("{:?}", number_info.len());
    // println!("{:?}", time_info.len());
    // println!("{:?}", text_info.len());
    let split_text = file_struct.split_text(text_info).unwrap();
    // println!("{:?}", split_text.len());

<<<<<<< HEAD
    let format_text = file_struct.format_text(split_text, 1).unwrap();
    println!("{:?}", format_text.0);

=======
    let (tagged_text, chunk_to_translate) = file_struct.format_text(split_text.clone(), 0).unwrap();
    // println!(
    //     "tagged_text:{:?}\r\n
    //     split_text:{:?}",
    //     tagged_text, chunk_to_translate
    // );

    let system_prompt = create_system_prompt(source_language.clone(), target_language.clone());
    let task_prompt = create_task_prompt(
        source_language,
        target_language,
        tagged_text,
        chunk_to_translate.clone(),
    );
    let response = openai_translate(system_prompt, task_prompt).await.unwrap();
    let result = file_struct.split_translated_text(response).unwrap();

    println!("第四轮翻译结果{:?}", result);

    let srt_content = file_struct.merge_contents(
        chunk_to_translate.clone(),
        result.clone(),
        time_info,
        0,
        number_info,
    );
    println!("srt_content:{:?}", srt_content);

    let (is_end, i) = file_struct
        .check_translation_completion(split_text, chunk_to_translate)
        .unwrap();

    println!("is_end:{:?}, i:{:?}", is_end, i);
>>>>>>> 3f41bee (feat: check translation completed)
    // let translated_combined_text = run_translation_tasks(
    //     split_contents,
    //     input_language.clone(),
    //     output_language.clone(),
    // );

    // let merged_contents = file_struct
    //     .merge_contents(&contents, translated_combined_text)
    //     .unwrap();

    // let mut file = File::create(format!(
    //     "{}_{}.{}",
    //     file_name,
    //     output_language.clone(),
    //     file_extension
    // ))?;

    // file.write_all(merged_contents.as_bytes())?;
    Ok(())
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
    let progress = Arc::new(Mutex::new(0));
    let total = contents.len(); // 总任务数
    let pb = pb_init(total as u64);

    contents
        .into_par_iter()
        .enumerate()
        .for_each(|(index, content)| {
            let translated_text = crate::translate(
                content.clone(),
                input_language.clone(),
                output_language.clone(),
            );

            {
                let mut translated_combined_text = translated_combined_text
                    .lock()
                    .expect("Failed to acquire lock");
                translated_combined_text.push((index, translated_text));
            }

            {
                let mut progress = progress.lock().expect("Failed to acquire lock");
                *progress += 1;
                pb.set_position(*progress);
                if *progress % 5 == 0 {
                    thread::sleep(time::Duration::from_millis(200));
                }
            }
        });
    pb.finish_with_message("finished");
}
