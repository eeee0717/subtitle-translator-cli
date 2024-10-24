use std::io;
use std::path::Path;
use subparse::{get_subtitle_format, parse_str, SubtitleEntry};

/// 从文件中读取内容到String
///
/// # Arguments
/// * `path` - 要读取的文件路径
///
/// # Returns
/// * `Result<String, io::Error>` - 成功返回文件内容，失败返回IO错误
///
/// # Errors
/// 当文件不存在或无法读取时会返回错误
pub fn read_file(path: &Path) -> Result<String, io::Error> {
    use std::io::Read;
    let mut file = std::fs::File::open(path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

/// 解析字幕文件并返回字幕条目列表
///
/// # Arguments
/// * `path` - 字幕文件路径
///
/// # Returns
/// * `Result<Vec<SubtitleEntry>, Box<dyn Error>>` - 成功返回字幕条目列表，失败返回错误
///
/// # Errors
/// - 文件读取错误
/// - 未知的字幕格式
/// - 解析错误
pub fn parse_file(path: &Path) -> Result<Vec<SubtitleEntry>, Box<dyn std::error::Error>> {
    let file_content = read_file(path)?;

    let format = match get_subtitle_format(path.extension(), file_content.as_bytes()) {
        Some(f) => f,
        None => return Err("Unknown subtitle format".into()),
    };

    let subtitle_file = match parse_str(format, &file_content, 25.0) {
        Ok(f) => f,
        Err(e) => return Err(format!("Failed to parse subtitle: {:?}", e).into()),
    };

    let subtitle_entries = match subtitle_file.get_subtitle_entries() {
        Ok(entries) => entries,
        Err(e) => return Err(format!("Failed to get subtitle entries: {:?}", e).into()),
    };

    Ok(subtitle_entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_file_success() {
        let valid_path = PathBuf::from("test.srt");

        let result = parse_file(&valid_path);
        assert!(result.is_ok());

        let entries = result.unwrap();
        assert_eq!(entries.len(), 60);
    }

    #[test]
    fn test_parse_file_invalid_path() {
        let invalid_path = PathBuf::from("nonexistent.srt");

        let result = parse_file(&invalid_path);

        assert!(result.is_err());
    }
}
