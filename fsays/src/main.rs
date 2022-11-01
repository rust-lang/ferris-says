#![recursion_limit = "1024"]

use clap::{App, Arg};
use ferris_says::*;
use std::error::Error;
use std::fs::File;
use std::io::{stderr, stdin, stdout, BufReader, BufWriter, Read, Write};
use std::process::exit;
use std::str;

// Constants used for err messages
const ARGS: &str = "Invalid argument passed to fsays caused an error";
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
    let args = App::new("Ferris Says")
        .version("0.1")
        .author("Michael Gattozzi <mgattozzi@gmail.com>")
        .about("Prints out input text with Ferris the Rustacean")
        .arg(
            Arg::with_name("FILES")
                .long("files")
                .short("f")
                .help("Sets the input files to use")
                .required(false)
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("WIDTH")
                .long("width")
                .short("w")
                .help("Sets the width of the text box")
                .takes_value(true)
                .default_value("40")
                .required(false),
        )
        .arg(
            Arg::with_name("TEXT")
                .required(false)
                .multiple(true)
                .hidden(true),
        )
        .get_matches();

    let width = args.value_of("WIDTH").unwrap().parse().map_err(|_| ARGS)?;

    let stdin = stdin();
    let stdout = stdout();

    let mut writer = BufWriter::new(stdout.lock());

    if let Some(files) = args.values_of("FILES") {
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
    } else if let Some(other_args) = args.values_of("TEXT") {
        let s = other_args.collect::<Vec<&str>>().join(" ");
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
