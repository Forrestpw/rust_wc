use std::fs::metadata;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::process::exit;
use clap::{command, Arg, ArgAction};

fn main() {
    let matches = command!()
        .arg(Arg::new("bytes")
            .short('b')
            .long("bytes")
            .action(ArgAction::SetTrue)
            .help("Provides the number of bytes in a file")
        )
        .arg(Arg::new("lines")
            .short('l')
            .long("lines")
            .action(ArgAction::SetTrue)
            .help("Provides the number of lines in a file")
        )
        .arg(Arg::new("words")
            .short('w')
            .long("words")
            .action(ArgAction::SetTrue)
            .help("Provides the number of words in a file")
        )
        .arg(Arg::new("chars")
            .short('c')
            .long("chars")
            .action(ArgAction::SetTrue)
            .help("Provides the number of chars in a file")
        )
        .arg(Arg::new("file")
            .required(true)
            .help("The file path that the operation will be performed on.")
        )
        .get_matches();

    let file_path = matches.get_one::<String>("file").unwrap();

    for flag in vec!["bytes", "lines", "words", "chars"] {
        if matches.get_flag(flag) {
            let operation_result = match flag {
                "bytes" => calculate_file_size(file_path),
                "lines" => calculate_file_line_count(file_path),
                "words" => calculate_file_word_count(file_path),
                "chars" => calculate_file_char_count(file_path),
                _ => {
                    println!("Error unknown operation");
                    exit(0);
                }
            };

            match operation_result {
                Ok(size) => println!("file {file_path} has {size} {flag}"),
                Err(e) => eprintln!("Error calculating file {flag}: {e}")
            }
        }
    }
}

fn calculate_file_size(file_path: &str) -> Result<usize, std::io::Error> {
    let metadata = metadata(file_path)?;
    Ok(metadata.len() as usize)
}

fn calculate_file_line_count(file_path: &str) -> Result<usize, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let line_count = reader.lines().count();
    Ok(line_count)
}

fn calculate_file_word_count(file_path: &str) -> Result<usize, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let word_count = contents.split_whitespace().count();

    Ok(word_count)
}

fn calculate_file_char_count(file_path: &str) -> Result<usize, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let word_count = contents.chars().count();

    Ok(word_count)
}
