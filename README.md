# phonetic-code

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

`phonetic-code` is a Rust library and command-line tool for converting text into various phonetic codes, also known as spelling alphabets. It supports a wide range of predefined alphabets like the NATO phonetic alphabet, as well as custom alphabets defined by the user.

## Features

- **Dual Functionality**: Usable as both a command-line tool (`spellout`) and a Rust library.
- **Wide Range of Alphabets**: Comes with many predefined phonetic alphabets, including international and country-specific ones.
- **Customizable**: Create your own phonetic alphabets by providing a base alphabet with substitutions or by loading definitions from a file.
- **Unicode Normalization**: Optional feature to normalize input strings for consistent conversion.
- **Simple & Fast**: Lightweight and designed for quick conversions.

## CLI Installation & Usage

### Installation

You can install the CLI tool using Cargo:

```sh
brew install tamada/tap/spellout
```

### Basic Usage

To convert a string to the default NATO phonetic alphabet:

```sh
spellout "Hello World"
```

Output:
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

Output:
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

Output:
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

Output:
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

## Library Usage

To use `phonetic-code` in your Rust project, add it to your `Cargo.toml`:

```toml
[dependencies]
phonetic-code = "0.1.0" # Check for the latest version
```

### Example: Convert a word using a predefined alphabet

```rust
use phonetic_code::{CodesBuilder, PhoneticCode};

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

`phonetic-code` supports the following built-in phonetic alphabets:

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

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
