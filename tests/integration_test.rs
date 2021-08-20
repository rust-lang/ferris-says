extern crate ferris_says;

use ferris_says::{say, think, SpeechModes};

// Default width when running the binary
const DEFAULT_WIDTH: usize = 40;

#[test]
fn hello_fellow_rustaceans_width_24() -> Result<(), ()> {
    //Hello fellow Rustaceans!
    #[cfg(not(feature = "clippy"))]
    let expected_say = br#"
 __________________________
< Hello fellow Rustaceans! >
 --------------------------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
"#;
    #[cfg(not(feature = "clippy"))]
    let expected_think = br#"
 __________________________
< Hello fellow Rustaceans! >
 --------------------------
        o
         o
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
"#;
    #[cfg(feature = "clippy")]
    let expected = br#"
 __________________________
< Hello fellow Rustaceans! >
 --------------------------
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

    let input = b"Hello fellow Rustaceans!";
    let width = 24;

    compare_strings(input, width, &expected_say[1..], SpeechModes::SAY);
    compare_strings(input, width, &expected_think[1..], SpeechModes::THINK);
    Ok(())
}

#[test]
fn hello_fellow_rustaceans_width_12() -> Result<(), ()> {
    //Hello fellow Rustaceans!
    #[cfg(not(feature = "clippy"))]
    let expected_say = br#"
 ______________
/ Hello fellow \
\ Rustaceans!  /
 --------------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
"#;
    #[cfg(not(feature = "clippy"))]
    let expected_think = br#"
 ______________
/ Hello fellow \
\ Rustaceans!  /
 --------------
        o
         o
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
"#;
    #[cfg(feature = "clippy")]
    let expected = br#"
 ______________
/ Hello fellow \
\ Rustaceans!  /
 --------------
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

    let input = b"Hello fellow Rustaceans!";
    let width = 12;

    compare_strings(input, width, &expected_say[1..], SpeechModes::SAY);
    compare_strings(input, width, &expected_think[1..], SpeechModes::THINK);

    Ok(())
}

#[test]
fn hello_fellow_rustaceans_width_6() -> Result<(), ()> {
    //Hello fellow Rustaceans!
    #[cfg(not(feature = "clippy"))]
    let expected_say = br#"
 ________
/ Hello  \
| fellow |
| Rustac |
\ eans!  /
 --------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
"#;
    #[cfg(not(feature = "clippy"))]
    let expected_think = br#"
 ________
/ Hello  \
| fellow |
| Rustac |
\ eans!  /
 --------
        o
         o
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
"#;
    #[cfg(feature = "clippy")]
    let expected = br#"
 ________
/ Hello  \
| fellow |
| Rustac |
\ eans!  /
 --------
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

    let input = b"Hello fellow Rustaceans!";
    let width = 6;

    compare_strings(input, width, &expected_say[1..], SpeechModes::SAY);
    compare_strings(input, width, &expected_think[1..], SpeechModes::THINK);

    Ok(())
}

#[test]
fn hello_fellow_rustaceans_width_3() -> Result<(), ()> {
    //Hello fellow Rustaceans!
    #[cfg(not(feature = "clippy"))]
    let expected_say = br#"
 _____
/ Hel \
| lo  |
| fel |
| low |
| Rus |
| tac |
| ean |
\ s!  /
 -----
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
"#;
    #[cfg(not(feature = "clippy"))]
    let expected_think = br#"
 _____
/ Hel \
| lo  |
| fel |
| low |
| Rus |
| tac |
| ean |
\ s!  /
 -----
        o
         o
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
"#;
    #[cfg(feature = "clippy")]
    let expected = br#"
 _____
/ Hel \
| lo  |
| fel |
| low |
| Rus |
| tac |
| ean |
\ s!  /
 -----
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

    let input = b"Hello fellow Rustaceans!";
    let width = 3;

    compare_strings(input, width, &expected_say[1..], SpeechModes::SAY);
    compare_strings(input, width, &expected_think[1..], SpeechModes::THINK);

    Ok(())
}

#[test]
fn multibyte_string() -> Result<(), ()> {
    #[cfg(not(feature = "clippy"))]
    let expected_say = concat!(
        " ____________\n",
        "< 突然の死👻 >\n",
        " ------------\n",
        r#"        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
"#
    );
    #[cfg(not(feature = "clippy"))]
    let expected_think = concat!(
        " ____________\n",
        "< 突然の死👻 >\n",
        " ------------\n",
        r#"        o
         o
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
"#
    );
    #[cfg(feature = "clippy")]
    let expected = concat!(
        " ____________\n",
        "< 突然の死👻 >\n",
        " ------------\n",
        r#"        \
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
"#
    );

    let input = "突然の死👻";
    let width = DEFAULT_WIDTH;

    compare_strings(input.as_bytes(), width, &expected_say.as_bytes(), SpeechModes::SAY);
    compare_strings(input.as_bytes(), width, &expected_think.as_bytes(), SpeechModes::THINK);

    Ok(())
}

fn compare_strings(input: &[u8], width: usize, expected: &[u8], mode: SpeechModes) {
    let mut vec = Vec::new();
    match mode {
        SpeechModes::SAY => {
            say(input, width, &mut vec).unwrap();
        }
        SpeechModes::THINK => {
            think(input, width, &mut vec).unwrap();
        }
    }
    let actual = std::str::from_utf8(&vec).unwrap();
    assert_eq!(std::str::from_utf8(&expected).unwrap(), actual);
}