extern crate ferris_says;

use ferris_says::say;

// Default width when running the binary
const DEFAULT_WIDTH: usize = 40;

#[test]
fn hello_fellow_rustaceans_width_24() -> Result<(), ()> {
    //Hello fellow Rustaceans!
    #[cfg(not(feature = "clippy"))]
    let expected = br#"
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

    let mut vec = Vec::new();

    say(input, width, &mut vec).unwrap();

    let actual = std::str::from_utf8(&vec).unwrap();

    assert_eq!(std::str::from_utf8(&expected[1..]).unwrap(), actual);
    Ok(())
}

#[test]
fn hello_fellow_rustaceans_width_12() -> Result<(), ()> {
    //Hello fellow Rustaceans!
    #[cfg(not(feature = "clippy"))]
    let expected = br#"
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

    let mut vec = Vec::new();

    say(input, width, &mut vec).unwrap();

    let actual = std::str::from_utf8(&vec).unwrap();

    assert_eq!(std::str::from_utf8(&expected[1..]).unwrap(), actual);
    Ok(())
}

#[test]
fn hello_fellow_rustaceans_width_6() -> Result<(), ()> {
    //Hello fellow Rustaceans!
    #[cfg(not(feature = "clippy"))]
    let expected = br#"
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

    let mut vec = Vec::new();

    say(input, width, &mut vec).unwrap();

    let actual = std::str::from_utf8(&vec).unwrap();

    assert_eq!(std::str::from_utf8(&expected[1..]).unwrap(), actual);
    Ok(())
}

#[test]
fn hello_fellow_rustaceans_width_3() -> Result<(), ()> {
    //Hello fellow Rustaceans!
    #[cfg(not(feature = "clippy"))]
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

    let mut vec = Vec::new();

    say(input, width, &mut vec).unwrap();

    let actual = std::str::from_utf8(&vec).unwrap();

    assert_eq!(std::str::from_utf8(&expected[1..]).unwrap(), actual);
    Ok(())
}

#[test]
fn multibyte_string() -> Result<(), ()> {
    #[cfg(not(feature = "clippy"))]
    let expected = concat!(
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

    let mut vec = Vec::new();

    say(input.as_bytes(), width, &mut vec).unwrap();

    let actual = std::str::from_utf8(&vec).unwrap();

    assert_eq!(std::str::from_utf8(&expected.as_bytes()).unwrap(), actual);
    Ok(())
}
