# CLI command of spellout

This is the CLI command of the `spellout` library.
It provides a command-line interface to get the phonetic code for a given character or word.

## :anchor: Installation

You can install the CLI tool (`spellout`) using Homebrew :beer::

```sh
brew install tamada/tap/spellout
```

## :runner: Usage

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
      --input <FILE>  Specify the the path to a custom phonetic code file.
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
