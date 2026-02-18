# CLI command of phonetic-code

This is the CLI command of the `phonetic-code` library.
It provides a command-line interface to get the phonetic code for a given character or word.

## :runner: Usage

```bash
phonetic-code [OPTIONS] <WORDS...>
OPTIONS
    -c, --code <TYPE>        Specify the phonetic code for encoding the input text.
        --input <FILE>       Specify the the path to a custom phonetic code file.
    -p, --print <TYPE>       print the all of phonetic code for the given type.
        --only-code          print the only phonetic code for the given words.
WORDS
    The words to convert to phonetic codes.
```

## Example: basic use

```bash
$ phonetic-code "Hello World"
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

## Example: phonetic code only

```bash
$ phonetic-code -c "Hello World"
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
```
