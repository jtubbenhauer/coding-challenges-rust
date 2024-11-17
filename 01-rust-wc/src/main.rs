use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'c', long)]
    bytes: bool,

    #[arg(short = 'l', long)]
    lines: bool,

    #[arg(short = 'w', long)]
    words: bool,

    #[arg(required = true)]
    filename: String,
}

struct FileStats {
    bytes: Option<usize>,
    lines: Option<usize>,
    words: Option<usize>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut output = String::from("");
    let stats = get_stats(&args.filename, args.bytes, args.lines, args.words)?;

    if let Some(lines) = stats.lines {
        output.push_str(&lines.to_string());
    }

    if let Some(bytes) = stats.bytes {
        if !output.is_empty() {
            output.push(' ');
        }
        output.push_str(&bytes.to_string());
    }

    if let Some(words) = stats.words {
        if !output.is_empty() {
            output.push(' ');
        }
        output.push_str(&words.to_string());
    }

    println!("{output} {}", &args.filename);
    Ok(())
}

fn get_stats(
    filename: &String,
    get_bytes: bool,
    get_lines: bool,
    get_words: bool,
) -> io::Result<FileStats> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut bytes: usize = 0;
    let mut lines: usize = 0;
    let mut words: usize = 0;

    for line in reader.lines() {
        let line = line?;
        if get_lines {
            lines += 1;
        }
        if get_bytes {
            bytes += line.len() + 1;
        }
        if get_words {
            words += line.split(' ').filter(|word| !word.is_empty()).count();
        }
    }

    Ok(FileStats {
        bytes: if get_bytes { Some(bytes) } else { None },
        lines: if get_lines { Some(lines) } else { None },
        words: if get_words { Some(words) } else { None },
    })
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use tempfile::NamedTempFile;

    use super::*;

    fn create_temp_file_with_content(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", content).unwrap();
        file
    }

    #[test]
    fn test_get_stats_lines() {
        let content = "Line 1\nLine 2\nLine 3\n";
        let temp_file = create_temp_file_with_content(content);
        let filename = temp_file.path().to_str().unwrap().to_string();

        let stats = get_stats(&filename, false, true, false).unwrap();
        assert_eq!(stats.lines, Some(3));
        assert_eq!(stats.bytes, None);
    }

    #[test]
    fn test_get_stats_bytes() {
        let content = "Line 1\nLine 2\nLine 3\n";
        let temp_file = create_temp_file_with_content(content);
        let filename = temp_file.path().to_str().unwrap().to_string();

        let stats = get_stats(&filename, true, false, false).unwrap();
        assert_eq!(stats.bytes, Some(content.len()));
        assert_eq!(stats.lines, None);
    }

    #[test]
    fn test_get_stats_lines_and_bytes() {
        let content = "Line 1\nLine 2\nLine 3\n";
        let temp_file = create_temp_file_with_content(content);
        let filename = temp_file.path().to_str().unwrap().to_string();

        let stats = get_stats(&filename, true, true, false).unwrap();
        assert_eq!(stats.lines, Some(3));
        assert_eq!(stats.bytes, Some(content.len()));
    }

    #[test]
    fn test_get_stats_empty_file() {
        let content = "";
        let temp_file = create_temp_file_with_content(content);
        let filename = temp_file.path().to_str().unwrap().to_string();

        let stats = get_stats(&filename, true, true, false).unwrap();
        assert_eq!(stats.lines, Some(0));
        assert_eq!(stats.bytes, Some(0));
    }

    #[test]
    fn test_get_stats_words() {
        let content = "One Two\nThree Four\n\nFive Six\n";
        let temp_file = create_temp_file_with_content(&content);
        let filename = temp_file.path().to_str().unwrap().to_string();

        let stats = get_stats(&filename, false, false, true).unwrap();
        assert_eq!(stats.lines, None);
        assert_eq!(stats.bytes, None);
        assert_eq!(stats.words, Some(6));
    }
}
