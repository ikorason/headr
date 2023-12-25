use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use anyhow::Result;
use clap::{Arg, Command};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: u64,
    bytes: Option<u64>,
}

pub fn get_args() -> Config {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("ikorason <irfanabliz914@gmail.com")
        .about("rust head")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .value_parser(clap::value_parser!(u64).range(1..))
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .conflicts_with("lines")
                .help("Number of bytes")
                .value_parser(clap::value_parser!(u64).range(1..)),
        )
        .get_matches();

    Config {
        files: matches
            .get_many("files")
            .expect("No files given")
            .cloned()
            .collect(),
        lines: matches.get_one("lines").cloned().unwrap(),
        bytes: matches.get_one("bytes").cloned(),
    }
}

pub fn run(config: Config) -> Result<()> {
    let num_files = config.files.len();

    for (file_num, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        &filename
                    );
                }

                if let Some(num_bytes) = config.bytes {
                    let mut buffer = vec![0; num_bytes as usize];
                    let bytes_read = file.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{line}");
                        line.clear();
                    }
                }
            }
        }
    }

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
