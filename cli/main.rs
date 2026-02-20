use std::{io::BufRead, path::PathBuf};

use clap::{Parser, ValueEnum};
use spellout::{Codes, CodesBuilder, Error, PhoneticCode};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliOpts {
    #[arg(
        short, long, value_enum, hide_default_value = true, hide_possible_values = true, 
        default_value_t = PhoneticCode::Nato,
        value_parser = parse_code,
        help = "Specify the phonetic code for encoding/decoding the input text.
Default is NATO. Use `--list` option to see all available codes."
    )]
    code: PhoneticCode,

    #[arg(short, long, default_value_t = false, help = "Prints the available phonetic codes. ")]
    list: bool,

    #[arg(short, long, default_value_t = false, help = "Prints the phonetic codes for the given type.")]
    print: bool,

    #[arg(long = "only-code", default_value_t = false, help = "Prints the only phonetic code for the given words.")]
    only: bool,

    #[arg(short, long, default_value_t = false, help = "Decodes given phonetic codes into string.")]
    decode: bool,

    #[arg(long, value_name = "FILE", help = "Specify the path to a custom phonetic code file.")]
    input: Option<PathBuf>,

    #[cfg(debug_assertions)]
    #[arg(long, hide = true, default_value_t = false, help = "Generates completion files.")]
    gencomp: bool,

    #[arg(help = "The words to encode using the specified phonetic code.
Gives '-' read from stdin. No arguments also reads from stdin.")]
    args: Vec<String>,
}

fn print_all(codes: &Codes, only: bool) {
    for code in codes.entries() {
        if only {
            println!("{}", code.letter());
        } else {
            println!("{}    {}", code.letter(), code.code());
        }
    }
}

fn encode_string(codes: &Codes, only: bool, input: String) {
    for (letter, code) in codes.encode(input.as_ref()) {
        if !only {
            print!("{letter}    ");
        }
        if let Some(c) = code {
            println!("{}", c.code());
        } else {
            println!();
        }
    }
}

fn handle_stdin(codes: &Codes, only: bool) {
    for line in collect_inputs_from_stdin() {
        encode_string(codes, only, line);
    }
}

fn encode_words(codes: &Codes, only: bool, words: Vec<String>) {
    if words.is_empty() {
        handle_stdin(codes, only);
    } else {
        for (i, word) in words.into_iter().enumerate() {
            if i > 0 {
                println!();
            }
            if word == "-" {
                handle_stdin(codes, only);
            } else {
                encode_string(codes, only, word);
            }
        }
    }
}

fn print_list() {
    for pc in PhoneticCode::available_names() {
        println!("{pc}");
    }
}

fn collect_inputs_from_stdin() -> Vec<String> {
    let mut inputs = Vec::new();
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => inputs.push(line),
            Err(e) => eprintln!("Failed to read a line from stdin: {e}"),
        }
    }
    inputs
}

fn decode_all(codes: &Codes, arg: Vec<String>) {
    let mut inputs = Vec::new();
    if arg.is_empty() {
        inputs = collect_inputs_from_stdin();
    } else {
        for a in arg {
            if a == "-" {
                inputs.extend(collect_inputs_from_stdin());
            } else {
                inputs.push(a);
            }
        }
    }
    println!("{}", codes.decode(inputs));
}

fn parse_code(s: &str) -> Result<PhoneticCode, String> {
    if let Ok(pc) = PhoneticCode::from_str(s, true) {
        Ok(pc)
    } else if spellout::is_available_name(s) {
        Ok(PhoneticCode::Asset(s.to_string()))
    } else {
        Err(format!("Invalid phonetic code: {s}"))
    }
}

#[cfg(debug_assertions)]
mod gencomp;

fn perform(opts: CliOpts) -> Result<(), Error> {
    let codes = if let Some(input) = opts.input {
        CodesBuilder::build_from_file(input)?
    } else {
        CodesBuilder::build(opts.code)
    };
    #[cfg(debug_assertions)]
    if cfg!(debug_assertions) && opts.gencomp {
        gencomp::generate(std::path::Path::new("assets/completions"));
    }
    if opts.list {
        print_list()
    } else if opts.print {
        print_all(&codes, opts.only);
    } else if opts.decode {
        decode_all(&codes, opts.args);
    } else {
        encode_words(&codes, opts.only, opts.args);
    }
    Ok(())
}

fn main() {
    let opts = CliOpts::parse();
    if let Err(e) = perform(opts) {
        eprintln!("Error: {e:?}");
    }
}
