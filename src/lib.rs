extern crate smallvec;
extern crate textwrap;
extern crate unicode_width;

use smallvec::*;
use std::io::{Result, Write};
use std::iter::repeat;
use std::str;
use textwrap::fill;
use unicode_width::UnicodeWidthStr;

#[derive(Copy, Clone)]
pub enum SpeechModes {
    Think,
    Say,
}

#[derive(Copy, Clone)]
pub enum Eyes {
    RegularEyes,
    GreedyEyes,
    YouthfulEyes,
    ParanoidEyes,
    DeadEyes,
    TiredEyes,
    CryingEyes,
    HappyEyes
}

pub struct FerrisConfig {
    pub mode: SpeechModes,
    pub eyes: Eyes
}

// Constants! :D
const ENDSL: &[u8] = b"| ";
const ENDSR: &[u8] = b" |\n";
const REGULAR_EYES: &[u8] = b"o";
const GREEDY_EYES: &[u8] = b"$";
const YOUTHFUL_EYES: &[u8] = b".";
const PARANOID_EYES: &[u8] = b"@";
const DEAD_EYES: &[u8] = b"x";
const TIRED_EYES: &[u8] = b"-";
const CRYING_EYES: &[u8] = b"T";
const HAPPY_EYES: &[u8] = b"^";

#[cfg(not(feature = "clippy"))]
const FERRIS_TOP: &[u8] = br#"
            _~^~^~_
        \) /  "#;
#[cfg(not(feature = "clippy"))]
const FERRIS_BOTTOM: &[u8] = br#"  \ (/
          '_   -   _'
          / '-----' \
"#;

#[cfg(feature = "clippy")]
const CLIPPY_TOP: &[u8] = br#"
            __
           /  \
           |  |
           "#;
#[cfg(feature = "clippy")]
const CLIPPY_BOTTOM: &[u8] = br#"
           |  |
           || |/
           || ||
           |\_/|
           \___/
"#;

const SPEECH_BUBBLE: &[u8] = br#"
        \
         \"#;
const THOUGHT_BUBBLE: &[u8] = br#"
        o
         o"#;

const NEWLINE: u8 = b'\n';
const DASH: u8 = b'-';
const UNDERSCORE: u8 = b'_';

// A decent number for SmallVec's Buffer Size, not too large
// but also big enough for most inputs
const BUFSIZE: usize = 2048;

/// Print out Ferris saying something.
///
/// `input` is a slice of bytes that you want to be written out to somewhere
///
/// `max_width` is the maximum width of a line of text before it is wrapped
///
/// `writer` is anywhere that can be written to using the Writer trait like
/// STDOUT or STDERR
///
/// `eyes` Ferris has different moods
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
/// let out = b"Hello fellow Rustaceans!";
/// let width = 24;
///
/// let mut writer = BufWriter::new(stdout.lock());
/// say(out, width, &mut writer, &Eyes::RegularEyes).unwrap();
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

pub fn say<W>(input: &[u8], max_width: usize, writer: &mut W, eyes: &Eyes) -> Result<()>
where
    W: Write,
{
    let cfg = FerrisConfig {
        mode: SpeechModes::Say,
        eyes: *eyes
    };
    perform(input, max_width, writer, &cfg)
}

/// Print out Ferris thinking something
///
/// `input` is a slice of bytes that you want to be written out to somewhere
///
/// `max_width` is the maximum width of a line of text before it is wrapped
///
/// `writer` is anywhere that can be written to using the Writer trait like
/// STDOUT or STDERR
///
/// `eyes` Ferris has different moods
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
/// let out = b"Hello fellow Rustaceans!";
/// let width = 24;
///
/// let mut writer = BufWriter::new(stdout.lock());
/// think(out, width, &mut writer, &Eyes::RegularEyes).unwrap();
/// ```
///
/// This will print out:
///
/// ```plain
///  __________________________
/// < Hello fellow Rustaceans! >
///  --------------------------
///         o
///          o
///             _~^~^~_
///         \) /  o o  \ (/
///           '_   -   _'
///           / '-----' \
/// ```
pub fn think<W>(input: &[u8], max_width: usize, writer: &mut W, eyes: &Eyes) -> Result<()>
where
    W: Write,
{
    let cfg = FerrisConfig {
        mode: SpeechModes::Think,
        eyes: *eyes
    };
    perform(input, max_width, writer, &cfg)
}

/// Let Ferris say or think something
///
/// `input` is a slice of bytes that you want to be written out to somewhere
///
/// `max_width` is the maximum width of a line of text before it is wrapped
///
/// `writer` is anywhere that can be written to using the Writer trait like
/// STDOUT or STDERR
///
/// `cfg` Ferris can have different moods and also likes to think sometimes
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
/// let out = b"Hello fellow Rustaceans!";
/// let width = 24;
///
/// let mut writer = BufWriter::new(stdout.lock());
/// let ferris_cfg = FerrisConfig {
///     mode: SpeechModes::Think,
///     eyes: Eyes::HappyEyes
/// };
/// perform(out, width, &mut writer, &ferris_cfg).unwrap();
/// ```
///
/// This will print out:
///
/// ```plain
///  __________________________
/// < Hello fellow Rustaceans! >
///  --------------------------
///         o
///          o
///             _~^~^~_
///         \) /  ^ ^  \ (/
///           '_   -   _'
///           / '-----' \
/// ```
pub fn perform<W>(input: &[u8], max_width: usize, writer: &mut W, cfg: &FerrisConfig) -> Result<()>
where
    W: Write,
{
    // Final output is stored here
    let mut write_buffer = SmallVec::<[u8; BUFSIZE]>::new();

    // Let textwrap work its magic
    let wrapped = fill(
        str::from_utf8(input).map_err(|_| std::io::ErrorKind::InvalidData)?,
        max_width,
    );

    let lines: Vec<&str> = wrapped.lines().collect();

    let line_count = lines.len();
    let actual_width = longest_line(&lines);

    let mut top_bar_buffer: Vec<u8> = repeat(UNDERSCORE).take(actual_width + 2).collect();
    top_bar_buffer.insert(0, b' ');

    let mut bottom_bar_buffer: Vec<u8> = repeat(DASH).take(actual_width + 2).collect();
    bottom_bar_buffer.insert(0, b' ');

    write_buffer.extend_from_slice(&top_bar_buffer);
    write_buffer.push(NEWLINE);

    for (current_line, line) in lines.into_iter().enumerate() {
        if line_count == 1 {
            write_buffer.extend_from_slice(b"< ");
        } else if current_line == 0 {
            write_buffer.extend_from_slice(b"/ ");
        } else if current_line == line_count - 1 {
            write_buffer.extend_from_slice(b"\\ ");
        } else {
            write_buffer.extend_from_slice(ENDSL);
        }

        let line_len = UnicodeWidthStr::width(line);
        write_buffer.extend_from_slice(line.as_bytes());
        for _i in line_len..actual_width {
            write_buffer.extend_from_slice(b" ");
        }

        if line_count == 1 {
            write_buffer.extend_from_slice(b" >\n");
        } else if current_line == 0 {
            write_buffer.extend_from_slice(b" \\\n");
        } else if current_line == line_count - 1 {
            write_buffer.extend_from_slice(b" /\n");
        } else {
            write_buffer.extend_from_slice(ENDSR);
        }
    }

    write_buffer.extend_from_slice(&bottom_bar_buffer);
    let FerrisConfig { mode, eyes } = cfg;

    match mode {
        SpeechModes::Say =>  write_buffer.extend_from_slice(SPEECH_BUBBLE),
        SpeechModes::Think => write_buffer.extend_from_slice(THOUGHT_BUBBLE),
    }

    let eye = match eyes {
        Eyes::CryingEyes => CRYING_EYES,
        Eyes::DeadEyes => DEAD_EYES,
        Eyes::RegularEyes => REGULAR_EYES,
        Eyes::GreedyEyes => GREEDY_EYES,
        Eyes::ParanoidEyes => PARANOID_EYES,
        Eyes::YouthfulEyes => YOUTHFUL_EYES,
        Eyes::TiredEyes => TIRED_EYES,
        Eyes::HappyEyes => HAPPY_EYES,
    };

    #[cfg(feature = "clippy")]
    let eye_gap = b"  ";
    #[cfg(not(feature = "clippy"))]
    let eye_gap = b" ";

    #[cfg(feature = "clippy")]
    write_buffer.extend_from_slice(CLIPPY_TOP);
    #[cfg(not(feature = "clippy"))]
    write_buffer.extend_from_slice(FERRIS_TOP);

    write_buffer.extend_from_slice(eye);
    write_buffer.extend_from_slice(eye_gap);
    write_buffer.extend_from_slice(eye);

    #[cfg(feature = "clippy")]
    write_buffer.extend_from_slice(CLIPPY_BOTTOM);
    #[cfg(not(feature = "clippy"))]
    write_buffer.extend_from_slice(FERRIS_BOTTOM);
    writer.write_all(&write_buffer)?;

    Ok(())
}

fn longest_line(lines: &[&str]) -> usize {
    let mut max_width = 0;
    for line in lines {
        let line_width = UnicodeWidthStr::width(line.to_owned());
        if line_width > max_width {
            max_width = line_width;
        }
    }
    max_width
}
