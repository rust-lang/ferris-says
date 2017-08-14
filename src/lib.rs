extern crate smallvec;

use smallvec::*;
use std::io::{ Write, Result };
use std::iter::repeat;

// Constants! :D
const ENDSL:   &[u8] = b"| ";
const ENDSR:   &[u8] = b" |\n";
const FERRIS:  &[u8] = b"
              \\
               \\
                  _~^~^~_
              \\) /  o o  \\ (/
                '_   -   _'
                / '-----' \\
";
const NEWLINE: u8 = '\n' as u8;
const SPACE:   u8  = ' ' as u8;
const DASH:    u8  = '-' as u8;

// A decent number for SmallVec's Buffer Size, not too large
// but also big enough for most inputs
const BUFSIZE: usize = 2048;

// We need a value to add to the width which includes
// the length of ENDSL and ENDSR for the proper size
// calculation of the bar at the top and bottom of the
// box
const OFFSET:  usize = 4;


/// Have Ferris print out him saying something.
///
/// `input` is a slice of bytes that you want to be written out to somewhere
///
/// `width` is how wide you want the box that Ferris says something into to be
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
/// let out = b"Hello fellow Rustaceans!";
/// let width = 24;
///
/// let mut writer = BufWriter::new(stdout.lock());
/// say(out, width, &mut writer).unwrap();
/// ```
///
/// This will print out:
///
/// ```plain
/// ----------------------------
/// | Hello fellow Rustaceans! |
/// ----------------------------
///               \
///                \
///                   _~^~^~_
///               \) /  o o  \ (/
///                 '_   -   _'
///                 / '-----' \
/// ```

pub fn say<W>(input: &[u8], width: usize, writer: &mut W) -> Result<()>
    where W: Write
{
    // Final output is stored here
    let mut write_buffer = SmallVec::<[u8; BUFSIZE]>::new();

    // The top and bottom bar for the text box is calculated once here
    let bar_buffer: Vec<u8> = repeat(DASH).take(width + OFFSET).collect();

    write_buffer.extend_from_slice(&bar_buffer);
    write_buffer.push(NEWLINE);
    for i in input.split(|x| *x == '\n' as u8) {
        for j in i.chunks(width) {
            write_buffer.extend_from_slice(ENDSL);
            write_buffer.extend_from_slice(j);

            for _ in 0 .. width - j.len() {
                write_buffer.push(SPACE);
            }

            write_buffer.extend_from_slice(ENDSR);
        }

    }
    write_buffer.extend_from_slice(&bar_buffer);
    write_buffer.extend_from_slice(FERRIS);
    writer.write_all(&write_buffer)?;

    Ok(())
}
