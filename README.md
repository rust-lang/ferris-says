# Ferris Says

A library for printing out text with Ferris as the mascot!

## Build requirements

You only need a stable version of the Rust compiler. Due to the use of the `?`
operator only versions 1.15 and up of `rustc` are supported.

## How to use the library

Put the following in your `Cargo.toml`:

```toml
[dependencies]
ferris_says = "1.0"
```

Then import the crate with:

```rust
extern crate ferris_says;
```

### Example

The following bit of code will write the byte string to STDOUT

```rust
extern crate ferris_says;

use ferris_says::say;
use std::io::{ stdout, BufWriter };

fn main() {
    let stdout = stdout();
    let out = b"Hello fellow Rustaceans!";
    let width = 24;

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
}
```

This will print out this when run:

```plain
----------------------------
| Hello fellow Rustaceans! |
----------------------------
              \
               \
                 _~^~^~_
             \) /  o o  \ (/
               '_   -   _'
               / '-----' \
```

## How to use the binary

The binary version is called `fsays`. It reads input from `stdin` and prints it
out to the console.

```bash
echo 'Hello fellow Rustaceans!' | fsays --width 24
```

This will print out this when run:

```plain
----------------------------
| Hello fellow Rustaceans! |
----------------------------
              \
               \
                 _~^~^~_
             \) /  o o  \ (/
               '_   -   _'
               / '-----' \
```

You can also use multiple files as input by using the `-f`/`--files` flag!

## Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md) for more information.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
