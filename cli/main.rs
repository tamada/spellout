use std::path::PathBuf;

use clap::{Parser, ValueEnum, arg, command};
use phonetic_code::{Codes, CodesBuilder, PhoneticCode};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliOpts {
    #[arg(short, long, value_enum, default_value_t = PhoneticCode::Nato, help = "Specify the phonetic code for encoding the input text.")]
    code: PhoneticCode,

    #[arg(short, long, default_value_t = false, help = "Prints the available phonetic codes.")]
    list: bool,

    #[arg(short, long, default_value_t = false, help = "Prints the phonetic codes for the given type.")]
    print: bool,

    #[arg(long = "only-code", default_value_t = false, help = "Prints the only phonetic code for the given words.")]
    only: bool,

    #[arg(long, help = "Specify the the path to a custom phonetic code file.")]
    input: Option<PathBuf>,

    #[arg(help = "The words to encode using the specified phonetic code.")]
    args: Vec<String>,
}

fn print_all(codes: Codes, only: bool) {
    for code in codes.entries() {
        if only {
            println!("{}", code.letter());
        } else {
            println!("{}: {}", code.letter(), code.code());
        }
    }
}

fn encode_words(codes: Codes, only: bool, words: Vec<String>) {
    for c in words.join(" ").chars() {
        if let Some(code) = codes.code(c) {
            if only {
                println!("{}", code.code());
            } else {
                println!("{}    {} ", c, code.code());
            }
        } else {
            println!("{} ", c);
        }
    }
}

fn print_list() {
    for code in PhoneticCode::value_variants() {
        println!("{}", code);
    }
}

fn perform(opts: CliOpts) -> Result<(), Box<dyn std::error::Error>> {
    let codes = if let Some(input) = opts.input {
        CodesBuilder::build_from_file(input)?
    } else {
        CodesBuilder::build(opts.code)
    };
    if opts.list {
        print_list()
    } else if opts.print {
        print_all(codes, opts.only);
    } else {
        encode_words(codes, opts.only, opts.args);
    }
    Ok(())
}

fn main() {
    let opts = CliOpts::parse();
    if let Err(e) = perform(opts) {
        eprintln!("Error: {e}");
    }
}
