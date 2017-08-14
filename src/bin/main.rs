#![recursion_limit = "1024"]

extern crate ferris_says;
extern crate clap;
#[macro_use] extern crate error_chain;

use ferris_says::*;
use clap::{ Arg, App };
use std::io::{ stderr, stdin, stdout, BufWriter, BufReader, Read, Write };
use std::process::exit;
use std::fs::File;

error_chain!{}

// Constants used for err messages
const ARGS:   &str = "Invalid argument passed to fsays caused an error";
const INPUT:  &str = "Failed to read input to the program";
const STDOUT: &str = "Failed to write stdout";
const STDERR: &str = "Failed to write stderr";

fn main() {

    if let Err(ref e) = run() {
        let stderr = &mut stderr();

        writeln!(stderr, "error: {}", e).expect(STDERR);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(STDERR);
        }

        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(STDERR);
        }

        exit(1);
    }

}

fn run() -> Result<()> {
    let args = App::new("Ferris Says")
                    .version("0.1")
                    .author("Michael Gattozzi <mgattozzi@gmail.com>")
                    .about("Prints out input text with Ferris the Rustacean")
                    .arg(Arg::with_name("FILES")
                        .long("files")
                        .short("f")
                        .help("Sets the input files to use")
                        .required(false)
                        .takes_value(true)
                        .multiple(true))
                    .arg(Arg::with_name("WIDTH")
                        .long("width")
                        .short("w")
                        .help("Sets the width of the text box")
                        .takes_value(true)
                        .required(false))
                    .get_matches();

    let width = args.value_of("WIDTH")
                    .unwrap_or("50")
                    .parse()
                    .chain_err(|| ARGS)?;

    let stdin = stdin();
    let stdout = stdout();

    if let Some(files) = args.values_of("FILES") {
        // Read in files and say them with Ferris
        let reader = files
            .into_iter()
            .map(|i| {
                let reader = BufReader::new(File::open(i).chain_err(|| INPUT)?);
                Ok(reader
                    .bytes()
                    .fold(Ok(Vec::new()), |a: Result<Vec<u8>>, b| {
                        if let Ok(mut a) = a {
                            a.push(b.chain_err(|| INPUT)?);
                            Ok(a)
                        } else {
                            a
                        }
                    })?)
            })
            .collect::<Vec<Result<Vec<u8>>>>();

        let mut writer = BufWriter::new(stdout.lock());
        for i in reader {
            say(&i?, width, &mut writer)
                .chain_err(|| STDOUT)?;
        }

        Ok(())

    } else {

        let reader = BufReader::new(stdin.lock())
            .bytes()
            .fold(Ok(Vec::new()), |a: Result<Vec<u8>>, b| {
                if let Ok(mut a) = a {
                    a.push(b.chain_err(|| INPUT)?);
                    Ok(a)
                } else {
                    a
                }
            })?;
        let mut writer = BufWriter::new(stdout.lock());
        say(&reader, width, &mut writer)
            .chain_err(|| STDOUT)

    }

}
