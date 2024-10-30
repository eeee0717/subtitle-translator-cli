use std::{fs::File, io::Write, path::PathBuf};

pub struct Writer {}
impl Writer {
    pub fn write_file(content: String, path: PathBuf) {
        let mut file = File::create(path).expect("Failed to create file");
        match file.write(content.as_bytes()) {
            Ok(_) => println!("File written successfully"),
            Err(e) => println!("Error writing file: {}", e),
        }
    }
}
