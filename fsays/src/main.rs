#![recursion_limit = "1024"]

use clap::{App, Arg};
use ferris_says::*;
use std::{
    error::Error,
    fs,
    io::{stderr, stdin, stdout, BufWriter, Read, Write},
    process::exit,
    str,
};

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

    let mut stdin = stdin();
    let stdout = stdout();

    let mut writer = BufWriter::new(stdout.lock());

    if let Some(files) = args.values_of("FILES") {
        // Read in files and say them with Ferris
        for f in files {
            let content = fs::read_to_string(f).map_err(|_| INPUT)?;
            say(&content, width, &mut writer).map_err(|_| STDOUT)?;
        }
        Ok(())
    } else if let Some(other_args) = args.values_of("TEXT") {
        let text = other_args.collect::<Vec<&str>>().join(" ");
        say(&text, width, &mut writer).map_err(|_| STDOUT)?;
        Ok(())
    } else {
        let mut input = String::new();
        stdin.read_to_string(&mut input).map_err(|_| INPUT)?;
        say(&input, width, &mut writer).map_err(|_| STDOUT)?;
        Ok(())
    }
}
