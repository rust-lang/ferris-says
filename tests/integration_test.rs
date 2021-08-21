extern crate ferris_says;

use ferris_says::{say, think, perform, SpeechModes, Eyes, FerrisConfig};

// Default width when running the binary
const DEFAULT_WIDTH: usize = 40;

const SPEECH_BUBBLE: &str = r#"        \
         \"#;
const THOUGHT_BUBBLE: &str = r#"        o
         o"#;

#[cfg(not(feature = "clippy"))]
const REGULAR_FERRIS: &str = r#"
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
"#;
#[cfg(not(feature = "clippy"))]
const HAPPY_FERRIS: &str = r#"
            _~^~^~_
        \) /  ^ ^  \ (/
          '_   -   _'
          / '-----' \
"#;

#[cfg (feature = "clippy")]
const REGULAR_CLIPPY: &str = r#"
            __
           /  \
           |  |
           o  o
           |  |
           || |/
           || ||
           |\_/|
           \___/
"#;
#[cfg (feature = "clippy")]
const HAPPY_CLIPPY: &str = r#"
            __
           /  \
           |  |
           ^  ^
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
    let (expected_say, expected_think) = create_regular_ferris(speech.clone());

    let input = b"Hello fellow Rustaceans!";
    let width = 24;

    let say = FerrisConfig {
        mode: SpeechModes::Say,
        eyes: Eyes::RegularEyes
    };
    let think = FerrisConfig {
        mode: SpeechModes::Think,
        eyes: Eyes::RegularEyes
    };

    compare_strings_perform(input, width, &expected_say.as_bytes(), &say);
    compare_strings_perform(input, width, &expected_think.as_bytes(), &think);

    let (happy_say, happy_think) = create_happy_ferris(speech.clone());

    compare_strings_say_think(
        input, width, &happy_say.as_bytes(), &SpeechModes::Say, &Eyes::HappyEyes
    );
    compare_strings_say_think(
        input, width, &happy_think.as_bytes(), &SpeechModes::Think, &Eyes::HappyEyes
    );
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
    let (expected_say, expected_think) = create_regular_ferris(speech.clone());

    let input = b"Hello fellow Rustaceans!";
    let width = 12;

    let say = FerrisConfig {
        mode: SpeechModes::Say,
        eyes: Eyes::RegularEyes
    };
    let think = FerrisConfig {
        mode: SpeechModes::Think,
        eyes: Eyes::RegularEyes
    };

    compare_strings_perform(input, width, &expected_say.as_bytes(), &say);
    compare_strings_perform(input, width, &expected_think.as_bytes(), &think);

    let (happy_say, happy_think) = create_happy_ferris(speech.clone());

    compare_strings_say_think(
        input, width, &happy_say.as_bytes(), &SpeechModes::Say, &Eyes::HappyEyes
    );
    compare_strings_say_think(
        input, width, &happy_think.as_bytes(), &SpeechModes::Think, &Eyes::HappyEyes
    );
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
    let (expected_say, expected_think) = create_regular_ferris(speech.clone());

    let input = b"Hello fellow Rustaceans!";
    let width = 6;

    let say = FerrisConfig {
        mode: SpeechModes::Say,
        eyes: Eyes::RegularEyes
    };
    let think = FerrisConfig {
        mode: SpeechModes::Think,
        eyes: Eyes::RegularEyes
    };

    compare_strings_perform(input, width, &expected_say.as_bytes(), &say);
    compare_strings_perform(input, width, &expected_think.as_bytes(), &think);

    let (happy_say, happy_think) = create_happy_ferris(speech.clone());

    compare_strings_say_think(
        input, width, &happy_say.as_bytes(), &SpeechModes::Say, &Eyes::HappyEyes
    );
    compare_strings_say_think(
        input, width, &happy_think.as_bytes(), &SpeechModes::Think, &Eyes::HappyEyes
    );
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
    let (expected_say, expected_think) = create_regular_ferris(speech.clone());

    let input = b"Hello fellow Rustaceans!";
    let width = 3;

    let say = FerrisConfig {
        mode: SpeechModes::Say,
        eyes: Eyes::RegularEyes
    };
    let think = FerrisConfig {
        mode: SpeechModes::Think,
        eyes: Eyes::RegularEyes
    };

    compare_strings_perform(input, width, &expected_say.as_bytes(), &say);
    compare_strings_perform(input, width, &expected_think.as_bytes(), &think);

    let (happy_say, happy_think) = create_happy_ferris(speech.clone());

    compare_strings_say_think(
        input, width, &happy_say.as_bytes(), &SpeechModes::Say, &Eyes::HappyEyes
    );
    compare_strings_say_think(
        input, width, &happy_think.as_bytes(), &SpeechModes::Think, &Eyes::HappyEyes
    );
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
    let (expected_say, expected_think) = create_regular_ferris(speech.clone());

    let input = "突然の死👻";
    let width = DEFAULT_WIDTH;

    let say = FerrisConfig {
        mode: SpeechModes::Say,
        eyes: Eyes::RegularEyes
    };
    let think = FerrisConfig {
        mode: SpeechModes::Think,
        eyes: Eyes::RegularEyes
    };

    compare_strings_perform(input.as_bytes(), width, &expected_say.as_bytes(), &say);
    compare_strings_perform(input.as_bytes(), width, &expected_think.as_bytes(), &think);

    let (happy_say, happy_think) = create_happy_ferris(speech);

    compare_strings_say_think(
        input.as_bytes(), width, &happy_say.as_bytes(), &SpeechModes::Say, &Eyes::HappyEyes
    );
    compare_strings_say_think(
        input.as_bytes(), width, &happy_think.as_bytes(), &SpeechModes::Think, &Eyes::HappyEyes
    );
    Ok(())
}

fn create_regular_ferris(speech: String) -> (String, String) {
    #[cfg(not(feature = "clippy"))]
    let expected_say = speech.clone() + SPEECH_BUBBLE + REGULAR_FERRIS;
    #[cfg(not(feature = "clippy"))]
    let expected_think = speech + THOUGHT_BUBBLE + REGULAR_FERRIS;

    #[cfg(feature = "clippy")]
    let expected_say = speech.clone() + SPEECH_BUBBLE + REGULAR_CLIPPY;
    #[cfg(feature = "clippy")]
    let expected_think = speech + THOUGHT_BUBBLE + REGULAR_CLIPPY;
    (expected_say, expected_think)
}

fn create_happy_ferris(speech: String) -> (String, String) {
    #[cfg(not(feature = "clippy"))]
    let expected_say = speech.clone() + SPEECH_BUBBLE + HAPPY_FERRIS;
    #[cfg(not(feature = "clippy"))]
    let expected_think = speech + THOUGHT_BUBBLE + HAPPY_FERRIS;

    #[cfg(feature = "clippy")]
    let expected_say = speech.clone() + SPEECH_BUBBLE + HAPPY_CLIPPY;
    #[cfg(feature = "clippy")]
    let expected_think = speech + THOUGHT_BUBBLE + HAPPY_CLIPPY;
    (expected_say, expected_think)
}

fn compare_strings_perform(
    input: &[u8], width: usize, expected: &[u8], cfg: &FerrisConfig
) {
    let mut vec = Vec::new();
    perform(input, width, &mut vec, cfg).unwrap();
    let actual = std::str::from_utf8(&vec).unwrap();
    assert_eq!(std::str::from_utf8(&expected).unwrap(), actual);
}

fn compare_strings_say_think(
    input: &[u8], width: usize, expected: &[u8], mode: &SpeechModes, eyes: &Eyes)
{
    let mut vec = Vec::new();
    match mode {
        SpeechModes::Say => {
            say(input, width, &mut vec, eyes).unwrap();
        }
        SpeechModes::Think => {
            think(input, width, &mut vec, eyes).unwrap();
        }
    };
    let actual = std::str::from_utf8(&vec).unwrap();
    assert_eq!(std::str::from_utf8(&expected).unwrap(), actual);
}
