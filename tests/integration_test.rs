extern crate ferris_says;

use ferris_says::{say, think, SpeechModes};

// Default width when running the binary
const DEFAULT_WIDTH: usize = 40;

const SPEECH_BUBBLE: &str = r#"        \
         \"#;
const THOUGHT_BUBBLE: &str = r#"        o
         o"#;

#[cfg(not(feature = "clippy"))]
const FERRIS: &str = r#"
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
"#;

#[cfg (feature = "clippy")]
const CLIPPY: &str = r#"
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

#[test]
fn hello_fellow_rustaceans_width_24() -> Result<(), ()> {
    //Hello fellow Rustaceans!
    let speech = String::from(concat!(
        " __________________________\n",
        "< Hello fellow Rustaceans! >\n",
        " --------------------------\n",
    ));
    #[cfg(not(feature = "clippy"))]
    let expected_say = speech.clone() + SPEECH_BUBBLE + FERRIS;
    #[cfg(not(feature = "clippy"))]
    let expected_think = speech + THOUGHT_BUBBLE + FERRIS;

    #[cfg(feature = "clippy")]
    let expected_say = speech.clone() + SPEECH_BUBBLE + CLIPPY;
    #[cfg(feature = "clippy")]
    let expected_think = speech + THOUGHT_BUBBLE + CLIPPY;

    let input = b"Hello fellow Rustaceans!";
    let width = 24;

    compare_strings(input, width, &expected_say.as_bytes(), SpeechModes::Say);
    compare_strings(input, width, &expected_think.as_bytes(), SpeechModes::Think);
    Ok(())
}

#[test]
fn hello_fellow_rustaceans_width_12() -> Result<(), ()> {
    //Hello fellow Rustaceans!
    let speech = String::from(concat!(
        " ______________\n",
        "/ Hello fellow \\\n",
        "\\ Rustaceans!  /\n",
        " --------------\n"
    ));
    #[cfg(not(feature = "clippy"))]
    let expected_say = speech.clone() + SPEECH_BUBBLE + FERRIS;
    #[cfg(not(feature = "clippy"))]
    let expected_think = speech + THOUGHT_BUBBLE + FERRIS;

    #[cfg(feature = "clippy")]
    let expected_say = speech.clone() + SPEECH_BUBBLE + CLIPPY;
    #[cfg(feature = "clippy")]
    let expected_think = speech + THOUGHT_BUBBLE + CLIPPY;

    let input = b"Hello fellow Rustaceans!";
    let width = 12;

    compare_strings(input, width, &expected_say.as_bytes(), SpeechModes::Say);
    compare_strings(input, width, &expected_think.as_bytes(), SpeechModes::Think);

    Ok(())
}

#[test]
fn hello_fellow_rustaceans_width_6() -> Result<(), ()> {
    let speech = String::from(concat!(
        " ________\n",
        "/ Hello  \\\n",
        "| fellow |\n",
        "| Rustac |\n",
        "\\ eans!  /\n",
        " --------\n"
    ));
    #[cfg(not(feature = "clippy"))]
    let expected_say = speech.clone() + SPEECH_BUBBLE + FERRIS;
    #[cfg(not(feature = "clippy"))]
    let expected_think = speech + THOUGHT_BUBBLE + FERRIS;

    #[cfg(feature = "clippy")]
    let expected_say = speech.clone() + SPEECH_BUBBLE + CLIPPY;
    #[cfg(feature = "clippy")]
    let expected_think = speech + THOUGHT_BUBBLE + CLIPPY;

    let input = b"Hello fellow Rustaceans!";
    let width = 6;

    compare_strings(input, width, &expected_say.as_bytes(), SpeechModes::Say);
    compare_strings(input, width, &expected_think.as_bytes(), SpeechModes::Think);

    Ok(())
}

#[test]
fn hello_fellow_rustaceans_width_3() -> Result<(), ()> {
    //Hello fellow Rustaceans!
    let speech = String::from(concat!(
        " _____\n",
        "/ Hel \\\n",
        "| lo  |\n",
        "| fel |\n",
        "| low |\n",
        "| Rus |\n",
        "| tac |\n",
        "| ean |\n",
        "\\ s!  /\n",
        " -----\n"
    ));
    #[cfg(not(feature = "clippy"))]
    let expected_say = speech.clone() + SPEECH_BUBBLE + FERRIS;
    #[cfg(not(feature = "clippy"))]
    let expected_think = speech + THOUGHT_BUBBLE + FERRIS;

    #[cfg(feature = "clippy")]
    let expected_say = speech.clone() + SPEECH_BUBBLE + CLIPPY;
    #[cfg(feature = "clippy")]
    let expected_think = speech + THOUGHT_BUBBLE + CLIPPY;

    let input = b"Hello fellow Rustaceans!";
    let width = 3;

    compare_strings(input, width, &expected_say.as_bytes(), SpeechModes::Say);
    compare_strings(input, width, &expected_think.as_bytes(), SpeechModes::Think);

    Ok(())
}

#[test]
fn multibyte_string() -> Result<(), ()> {
    //Hello fellow Rustaceans!
    let speech = String::from(concat!(
        " ____________\n",
        "< 突然の死👻 >\n",
        " ------------\n"
    ));
    #[cfg(not(feature = "clippy"))]
    let expected_say = speech.clone() + SPEECH_BUBBLE + FERRIS;
    #[cfg(not(feature = "clippy"))]
    let expected_think = speech + THOUGHT_BUBBLE + FERRIS;

    #[cfg(feature = "clippy")]
    let expected_say = speech.clone() + SPEECH_BUBBLE + CLIPPY;
    #[cfg(feature = "clippy")]
    let expected_think = speech + THOUGHT_BUBBLE + CLIPPY;

    let input = "突然の死👻";
    let width = DEFAULT_WIDTH;

    compare_strings(input.as_bytes(), width, &expected_say.as_bytes(), SpeechModes::Say);
    compare_strings(input.as_bytes(), width, &expected_think.as_bytes(), SpeechModes::Think);

    Ok(())
}

fn compare_strings(input: &[u8], width: usize, expected: &[u8], mode: SpeechModes) {
    let mut vec = Vec::new();
    match mode {
        SpeechModes::Say => {
            say(input, width, &mut vec).unwrap();
        }
        SpeechModes::Think => {
            think(input, width, &mut vec).unwrap();
        }
    }
    let actual = std::str::from_utf8(&vec).unwrap();
    assert_eq!(std::str::from_utf8(&expected).unwrap(), actual);
}