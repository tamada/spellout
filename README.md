# spellout

[![build](https://github.com/tamada/spellout/actions/workflows/build.yaml/badge.svg)](https://github.com/tamada/spellout/actions/workflows/build.yaml)
[![Coverage Status](https://coveralls.io/repos/github/tamada/spellout/badge.svg?branch=main)](https://coveralls.io/github/tamada/spellout?branch=main)

[![Version](https://img.shields.io/badge/Version-v0.1.0-information)](https://github.com/tamada/spellout/releases/tag/v0.1.0)
[![LICENSE](https://img.shields.io/badge/license-MIT-information)](LICENSE)

[![Docker](https://img.shields.io/badge/Docker-ghcr.io/tamada/spellout:0.1.0-blue?logo=docker)](https://github.com/tamada/spellout/pkgs/container/spellout/)
[![Homebrew](https://img.shields.io/badge/Homebrew-tamada/tap/spellout-blue?logo=homebrew)](https://github.com/tamada/homebrew-tap)

`spellout` is a Rust library and command-line tool for converting text into various phonetic codes, also known as spelling alphabets. It supports a wide range of predefined alphabets like the NATO phonetic alphabet, as well as custom alphabets defined by the user.

## Features

- **Dual Functionality**: Usable as both a command-line tool (`spellout`) and a Rust library.
- **Wide Range of Alphabets**: Comes with many predefined phonetic alphabets, including international and country-specific ones.
- **Customizable**: Create your own phonetic alphabets by providing a base alphabet with substitutions or by loading definitions from a file.
- **Unicode Normalization**: Optional feature to normalize input strings for consistent conversion.
- **Simple & Fast**: Lightweight and designed for quick conversions.

## CLI Installation & Usage

### :anchor: Installation

You can install the CLI tool using Cargo:

```sh
brew install tamada/tap/spellout
```

### Basic Usage

To convert a string to the default NATO phonetic alphabet:

```sh
spellout "Hello World"
```

**Output:**
```
H    Hotel 
e    Echo 
l    Lima 
l    Lima 
o    Oscar 
     
W    Whiskey 
o    Oscar 
r    Romeo 
l    Lima 
d    Delta 
```

### Specify a Different Alphabet

Use the `-c` or `--code` option to specify a different phonetic alphabet.

```sh
spellout -c uk "Radio"
```

**Output:**
```
R    Robert 
a    Able 
d    Dog 
i    Item 
o    Orange 
```

### Use a Custom Alphabet File

You can define your own alphabet in a file and use it with the `--input` option. The file should contain `character,codeword` pairs. See `testdata/custom_codes.txt` for an example.

```sh
spellout --input testdata/custom_codes.txt "Test"
```

**Output:**
```
T    Target
e    Echo-Six
s    Sector
t    Target
```

### List Available Alphabets

To see a list of all supported predefined alphabets, use the `-l` or `--list` flag:
```sh
spellout -l
```

**Output:**
```
chp
english
eu
france
indonesia
international
italia
nato
netherlands
philippines
sweden
uk
usaairpots
japanese
```

### :whale: Docker Available

You can also run `spellout` using Docker:

```sh
docker run --rm -it ghcr.io/tamada/spellout:latest "Hello World"
H    Hotel
e    Echo
l    Lima
l    Lima
o    Oscar

W    Whiskey
o    Oscar
r    Romeo
l    Lima
d    Delta
```

#### Available tags

- `latest` (the latest version of `no-features-glibc`)
- `$VERSION-no_features_glibc`
- `$VERSION-no_features-musl`
- `$VERSION-unicode_normalization_glibc`
- `$VERSION-unicode_normalization_musl`

`$VERSION` is the version of `spellout` (e.g., `0.1.0`).

## Library Usage

To use `spellout` in your Rust project, add it to your `Cargo.toml`:

```toml
[dependencies]
spellout = "0.1.0" # Check for the latest version
```

### Example: Convert a word using a predefined alphabet

```rust
use spellout::{CodesBuilder, PhoneticCode};

fn main() {
    // Build the UK phonetic alphabet
    let codes = CodesBuilder::build(PhoneticCode::Uk);

    // Convert a word and print the results
    let word = "Hello";
    for (char, code) in codes.convert(word) {
        if let Some(c) = code {
            println!("{}    {}", char, c.code());
        } else {
            println!("{} ", char);
        }
    }
}
```

## Supported Alphabets

`spellout` supports the following built-in phonetic alphabets:

- `chp`
- `english`
- `eu`
- `france`
- `indonesia` (Based on NATO)
- `international`
- `italia`
- `japanese`
- `nato` (Default)
- `netherlands`
- `philippines` (Based on NATO)
- `sweden`
- `uk`
- `usaairpots` (Based on NATO)

## License

[![LICENSE](https://img.shields.io/badge/license-MIT-information)](LICENSE)

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
