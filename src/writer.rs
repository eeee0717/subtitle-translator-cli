use std::{fs::File, io::Write, path::PathBuf};

pub struct Writer {}
impl Writer {
    pub fn write_file(content: String, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;
        eprintln!("File written successfully");
        Ok(())
    }
}
