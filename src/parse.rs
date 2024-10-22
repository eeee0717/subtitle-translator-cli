use std::path::Path;

use subparse::{get_subtitle_format, parse_str, SubtitleEntry};

/// This function reads the content of a file to a `String`.
pub fn read_file(path: &Path) -> String {
    use std::io::Read;
    let mut file = std::fs::File::open(path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s
}
/// This function parses a subtitle file and returns a vector of `SubtitleEntry`.
pub fn parse_file(path: &Path) -> Vec<SubtitleEntry> {
    let file_content: String = read_file(&path); // your own load routine
    let format =
        get_subtitle_format(path.extension(), file_content.as_bytes()).expect("unknown format");
    let subtitle_file = parse_str(format, &file_content, 25.0).expect("parser error");
    let subtitle_entries: Vec<SubtitleEntry> = subtitle_file
        .get_subtitle_entries()
        .expect("unexpected error");
    subtitle_entries
}

mod test_parse {

    #[test]
    fn test_parse_file() {
        let mock = crate::mock::Mock::new();
        let subtitle_entries = crate::parse::parse_file(&mock.path);
        eprintln!("{:?}", subtitle_entries);
        assert_eq!(subtitle_entries.len(), 60);
    }
}
