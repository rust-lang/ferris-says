extern crate ferris_says;

use ferris_says::say;

// Default width when running the binary
const DEFAULT_WIDTH: usize = 40;

#[test]
fn hello_fellow_rustaceans_width_24() -> Result<(), ()> {
    //Hello fellow Rustaceans!
    #[cfg(not(feature = "clippy"))]
    let expected = r#"
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
    let expected = r#"
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

    let input = "Hello fellow Rustaceans!";
    let width = 24;

    let mut vec = Vec::new();

    say(input, width, &mut vec).unwrap();

    let actual = std::str::from_utf8(&vec).unwrap();

    assert_eq!(&expected[1..], actual);
    Ok(())
}

#[test]
fn hello_fellow_rustaceans_width_12() -> Result<(), ()> {
    //Hello fellow Rustaceans!
    #[cfg(not(feature = "clippy"))]
    let expected = r#"
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
    let expected = r#"
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

    let input = "Hello fellow Rustaceans!";
    let width = 12;

    let mut vec = Vec::new();

    say(input, width, &mut vec).unwrap();

    let actual = std::str::from_utf8(&vec).unwrap();

    assert_eq!(&expected[1..], actual);
    Ok(())
}

#[test]
fn hello_fellow_rustaceans_width_6() -> Result<(), ()> {
    //Hello fellow Rustaceans!
    #[cfg(not(feature = "clippy"))]
    let expected = r#"
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
    let expected = r#"
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

    let input = "Hello fellow Rustaceans!";
    let width = 6;

    let mut vec = Vec::new();

    say(input, width, &mut vec).unwrap();

    let actual = std::str::from_utf8(&vec).unwrap();

    assert_eq!(&expected[1..], actual);
    Ok(())
}

#[test]
fn hello_fellow_rustaceans_width_3() -> Result<(), ()> {
    //Hello fellow Rustaceans!
    #[cfg(not(feature = "clippy"))]
    let expected = r#"
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
    let expected = r#"
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

    let input = "Hello fellow Rustaceans!";
    let width = 3;

    let mut vec = Vec::new();

    say(input, width, &mut vec).unwrap();

    let actual = std::str::from_utf8(&vec).unwrap();

    assert_eq!(&expected[1..], actual);
    Ok(())
}

#[test]
fn multibyte_string() -> Result<(), ()> {
    #[cfg(not(feature = "clippy"))]
    let expected = r#"
 ____________
< 突然の死👻 >
 ------------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
"#;
    #[cfg(feature = "clippy")]
    let expected = r#"
 ____________
< 突然の死👻 >
 ------------
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

    let input = "突然の死👻";
    let width = DEFAULT_WIDTH;

    let mut vec = Vec::new();

    say(input, width, &mut vec).unwrap();

    let actual = std::str::from_utf8(&vec).unwrap();

    assert_eq!(&expected[1..], actual);
    Ok(())
}
