extern crate smallvec;
extern crate textwrap;
extern crate unicode_width;

use smallvec::*;
use std::io::{Result, Write};
use textwrap::fill;
use unicode_width::UnicodeWidthStr;

// Constants! :D
const ENDSL: &[u8] = b"| ";
const ENDSR: &[u8] = b" |\n";
#[cfg(not(feature = "clippy"))]
const FERRIS: &[u8] = br#"
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
"#;

#[cfg(feature = "clippy")]
const CLIPPY: &[u8] = br#"
        \
         \
            __
           /  \
           |  |
           @  @
           |  |
           || |/
           || ||
           |\_/|
           \___/
"#;
const NEWLINE: u8 = b'\n';
const DASH: u8 = b'-';
const UNDERSCORE: u8 = b'_';
const SPACE: u8 = b' ';

// A decent number for SmallVec's Buffer Size, not too large
// but also big enough for most inputs
const BUFSIZE: usize = 2048;

/// Print out Ferris saying something.
///
/// `input` is a string slice that you want to be written out to somewhere
///
/// `max_width` is the maximum width of a line of text before it is wrapped
///
/// `writer` is anywhere that can be written to using the Writer trait like
/// STDOUT or STDERR
///
/// # Example
///
/// The following bit of code will write the byte string to STDOUT
///
/// ```rust
/// use ferris_says::*;
/// use std::io::{ stdout, BufWriter };
///
/// let stdout = stdout();
/// let out = "Hello fellow Rustaceans!";
/// let width = 24;
///
/// let writer = BufWriter::new(stdout.lock());
/// say(out, width, writer).unwrap();
/// ```
///
/// This will print out:
///
/// ```plain
///  __________________________
/// < Hello fellow Rustaceans! >
///  --------------------------
///         \
///          \
///             _~^~^~_
///         \) /  o o  \ (/
///           '_   -   _'
///           / '-----' \
/// ```
pub fn say<W>(input: &str, max_width: usize, mut writer: W) -> Result<()>
where
    W: Write,
{
    // Final output is stored here
    let mut write_buffer = SmallVec::<[u8; BUFSIZE]>::new();

    // Let textwrap work its magic
    let wrapped = fill(input, max_width);

    let lines: Vec<&str> = wrapped.lines().collect();

    let line_count = lines.len();
    let actual_width = longest_line(&lines);

    // top box border
    write_buffer.push(SPACE);
    for _ in 0..(actual_width + 2) {
        write_buffer.push(UNDERSCORE);
    }
    write_buffer.push(NEWLINE);

    // inner message
    for (i, line) in lines.into_iter().enumerate() {
        if line_count == 1 {
            write_buffer.extend_from_slice(b"< ");
        } else if i == 0 {
            write_buffer.extend_from_slice(b"/ ");
        } else if i == line_count - 1 {
            write_buffer.extend_from_slice(b"\\ ");
        } else {
            write_buffer.extend_from_slice(ENDSL);
        }

        let line_len = UnicodeWidthStr::width(line);
        write_buffer.extend_from_slice(line.as_bytes());
        for _ in line_len..actual_width {
            write_buffer.push(SPACE);
        }

        if line_count == 1 {
            write_buffer.extend_from_slice(b" >\n");
        } else if i == 0 {
            write_buffer.extend_from_slice(b" \\\n");
        } else if i == line_count - 1 {
            write_buffer.extend_from_slice(b" /\n");
        } else {
            write_buffer.extend_from_slice(ENDSR);
        }
    }

    // bottom box border
    write_buffer.push(SPACE);
    for _ in 0..(actual_width + 2) {
        write_buffer.push(DASH);
    }

    // mascot
    #[cfg(feature = "clippy")]
    write_buffer.extend_from_slice(CLIPPY);
    #[cfg(not(feature = "clippy"))]
    write_buffer.extend_from_slice(FERRIS);

    writer.write_all(&write_buffer)
}

fn longest_line(lines: &[&str]) -> usize {
    lines
        .iter()
        .map(|line| UnicodeWidthStr::width(*line))
        .max()
        .unwrap_or(0)
}
