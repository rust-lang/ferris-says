use clap::{command, value_parser, Arg, ArgAction};
use ferris_says::*;
use std::error::Error;
use std::fs::File;
use std::io::{stderr, stdin, stdout, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::process::exit;
use std::str;

// Constants used for err messages
const INPUT: &str = "Failed to read input to the program";
const STDOUT: &str = "Failed to write stdout";
const STDERR: &str = "Failed to write stderr";

fn main() {
    if let Err(ref e) = run() {
        let stderr = &mut stderr();

        writeln!(stderr, "error: {}", e).expect(STDERR);

        exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = command!("Ferris Says")
        .about("Prints out input text with Ferris the Rustacean")
        .arg(
            Arg::new("FILES")
                .long("files")
                .short('f')
                .help("Set the input files to use")
                .action(ArgAction::Append)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("WIDTH")
                .long("width")
                .short('w')
                .help("Set the width of the text box")
                .default_value("40")
                .value_parser(value_parser!(usize)),
        )
        .arg(Arg::new("TEXT").action(ArgAction::Append))
        .get_matches();

    let width = *args.get_one::<usize>("WIDTH").unwrap();

    let stdin = stdin();
    let stdout = stdout();

    let mut writer = BufWriter::new(stdout.lock());

    if let Some(files) = args.get_many::<PathBuf>("FILES") {
        // Read in files and say them with Ferris
        let reader = files
            .map(|i| {
                let reader = BufReader::new(File::open(i).map_err(|_| INPUT)?);
                Ok(reader.bytes().fold(
                    Ok(Vec::new()),
                    |a: Result<Vec<u8>, Box<dyn Error>>, b| {
                        if let Ok(mut a) = a {
                            a.push(b.map_err(|_| INPUT)?);
                            Ok(a)
                        } else {
                            a
                        }
                    },
                )?)
            })
            .collect::<Vec<Result<Vec<u8>, Box<dyn Error>>>>();
        for i in reader {
            say(str::from_utf8(&i?)?, width, &mut writer).map_err(|_| STDOUT)?;
        }

        Ok(())
    } else if let Some(other_args) = args.get_many::<String>("TEXT") {
        let s = other_args
            .map(String::as_str)
            .collect::<Vec<&str>>()
            .join(" ");
        say(&s, width, &mut writer).map_err(|_| STDOUT)?;

        Ok(())
    } else {
        let reader = BufReader::new(stdin.lock()).bytes().fold(
            Ok(Vec::new()),
            |a: Result<Vec<u8>, Box<dyn Error>>, b| {
                if let Ok(mut a) = a {
                    a.push(b.map_err(|_| INPUT)?);
                    Ok(a)
                } else {
                    a
                }
            },
        )?;
        say(str::from_utf8(&reader)?, width, &mut writer).map_err(|_| STDOUT)?;

        Ok(())
    }
}
