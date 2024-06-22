pub struct Config {
    pub file_path: String,
    pub file_name: String,
    pub input_language: String,
    pub output_language: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // Skip the first argument, which is the program name
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("No file path provided"),
        };
        let file_name = file_path.split('/').last().unwrap().to_string();
        let input_language = match args.next() {
            Some(arg) => arg,
            None => return Err("No input language provided"),
        };
        let output_language = match args.next() {
            Some(arg) => arg,
            None => return Err("No output language provided"),
        };
        Ok(Config {
            file_path,
            file_name,
            input_language,
            output_language,
        })
    }
}
