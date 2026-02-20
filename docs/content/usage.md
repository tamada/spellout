---
title: ":runner: Usage"
date: 2026-02-19
---

## Help Message

The help message of `spellout` provides detailed information on how to use the CLI tool:

```
Usage: spellout [OPTIONS] [ARGS]...

Arguments:
  [ARGS]...  The words to encode using the specified phonetic code.
             Gives '-' read from stdin. No arguments also reads from stdin.

Options:
  -c, --code <CODE>   Specify the phonetic code for encoding/decoding the input text.
                      Default is NATO. Use `--list` option to see all available codes.
  -l, --list          Prints the available phonetic codes.
  -p, --print         Prints the phonetic codes for the given type.
      --only-code     Prints the only phonetic code for the given words.
  -d, --decode        Decodes given phonetic codes into string.
      --input <FILE>  Specify the path to a custom phonetic code file.
  -h, --help          Print help
  -V, --version       Print version
```

## Examples

### Basic Conversion

```bash
$ spellout "Hello World"
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

### Using a Different Phonetic Code

Convert a Japanese phrase using the `japanese` alphabet.

```bash
$ spellout -c japanese "こんにちは"
こ    子どものコ
ん    おしまいのン
に    日本の二
ち    ちどりのチ
は    葉書のハ
```

### Listing Available Codes

```bash
$ spellout -l
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

### Printing a Full Alphabet

```bash
$ spellout -c uk --print
A    Able
B    Baker
C    Charlie
D    Dog
E    Easy
F    Fox
G    George
H    How
I    Item
J    Jig
K    King
L    Love
M    Mike
N    Nan
O    Oboe
P    Peter
Q    Queen
R    Roger
S    Samuel
T    Tare
U    Uncle
V    Victor
W    William
X    X-ray
Y    Yoke
Z    Zebra
```

### Using a Custom File

Define your own codes in a file (e.g., `my_codes.txt`):

```
A,Apple
B,Ball
C,Cat
```

And use it with the `--input` option:

```bash
$ spellout --input my_codes.txt "CAB"
C    Cat
A    Apple
B    Ball
```

### Decoding Phonetic Codes

```bash
$ spellout --only-code "Hello World" | tee codes.txt
Hotel 
Echo 
Lima 
Lima 
Oscar 

Whiskey 
Oscar 
Romeo 
Lima 
Delta 
$ cat codes.txt | spellout --decode
HELLO WORLD
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
