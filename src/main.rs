use clap::Parser;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process;
use base64::{Engine as _, engine::general_purpose};
use regex::{Regex, RegexSet};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: PathBuf,

    #[arg(short, long, default_value_t = 1)]
    level: u8,

    #[arg(long)]
    compile: bool,
}

const TAPE_SIZE: usize = 30_000;

fn main() -> io::Result<()> {
    let args = Args::parse();

    if args.compile {
        panic!("--compile feature is not yet implemented!");
    }

    let source_code = fs::read_to_string(args.path)?;
    
    match args.level {
        1 => run_level1(&source_code),
        2 => run_level2(&source_code),
        3 => run_level3(&source_code),
        4 => run_level4(&source_code),
        _ => eprintln!("Error: Invalid level '{}'. Please choose a level from 1 to 4.", args.level),
    }

    Ok(())
}

fn run_level1(code: &str) {
    let instructions: Vec<char> = code.chars().collect();
    let mut tape = vec![0u8; TAPE_SIZE];
    let mut pointer: usize = 0;
    let mut instruction_pointer: usize = 0;

    while instruction_pointer < instructions.len() {
        match instructions[instruction_pointer] {
            '~' => pointer = if pointer == TAPE_SIZE - 1 { 0 } else { pointer + 1 },
            '!' => pointer = if pointer == 0 { TAPE_SIZE - 1 } else { pointer - 1 },
            '*' => tape[pointer] = tape[pointer].wrapping_add(1),
            '^' => tape[pointer] = tape[pointer].wrapping_sub(1),
            '@' => { print!("{}", tape[pointer] as char); io::stdout().flush().unwrap(); },
            '#' => {
                let mut buffer = [0; 1];
                if io::stdin().read_exact(&mut buffer).is_ok() { tape[pointer] = buffer[0]; } else { tape[pointer] = 0; }
            },
            '{' => {
                if tape[pointer] == 0 {
                    let mut loop_nesting = 1;
                    while loop_nesting > 0 {
                        instruction_pointer += 1;
                        if instruction_pointer >= instructions.len() { panic!("Unmatched '{{'"); }
                        match instructions[instruction_pointer] { '{' => loop_nesting += 1, '}' => loop_nesting -= 1, _ => {} }
                    }
                }
            },
            '}' => {
                if tape[pointer] != 0 {
                    let mut loop_nesting = 1;
                    while loop_nesting > 0 {
                        if instruction_pointer == 0 { panic!("Unmatched '}}'"); }
                        instruction_pointer -= 1;
                        match instructions[instruction_pointer] { '}' => loop_nesting += 1, '{' => loop_nesting -= 1, _ => {} }
                    }
                }
            },
            _ => {}
        }
        instruction_pointer += 1;
    }
}

fn run_level2(code: &str) {
    let instructions: Vec<char> = code.chars().collect();
    let mut tape = vec![0u8; TAPE_SIZE];
    let mut pointer: usize = 0;
    let mut instruction_pointer: usize = 0;

    while instruction_pointer < instructions.len() {
        match instructions[instruction_pointer] {
            '<' => pointer = if pointer == TAPE_SIZE - 1 { 0 } else { pointer + 1 },
            '>' => pointer = if pointer == 0 { TAPE_SIZE - 1 } else { pointer - 1 },
            '-' => tape[pointer] = tape[pointer].wrapping_add(1),
            '+' => tape[pointer] = tape[pointer].wrapping_sub(1),
            '.' => { print!("{}", tape[pointer] as char); io::stdout().flush().unwrap(); },
            ',' => {
                let mut buffer = [0; 1];
                if io::stdin().read_exact(&mut buffer).is_ok() { tape[pointer] = buffer[0]; } else { tape[pointer] = 0; }
            },
            ']' => {
                if tape[pointer] == 0 {
                    let mut loop_nesting = 1;
                    while loop_nesting > 0 {
                        instruction_pointer += 1;
                        if instruction_pointer >= instructions.len() { eprintln!("SAW ERROR: Teeth misaligned: ']' not closed."); process::exit(1); }
                        match instructions[instruction_pointer] { ']' => loop_nesting += 1, '[' => loop_nesting -= 1, _ => {} }
                    }
                }
            },
            '[' => {
                if tape[pointer] != 0 {
                    let mut loop_nesting = 1;
                    while loop_nesting > 0 {
                        if instruction_pointer == 0 { eprintln!("SAW ERROR: Blade jammed on unmatched '['."); process::exit(1); }
                        instruction_pointer -= 1;
                        match instructions[instruction_pointer] { '[' => loop_nesting += 1, ']' => loop_nesting -= 1, _ => {} }
                    }
                }
            },
            _ => {}
        }
        instruction_pointer += 1;
    }
}

fn run_level3(code: &str) {
    let source_string: String = code.chars().filter(|c| !c.is_whitespace()).collect();

    let instructions = match general_purpose::STANDARD.decode(&source_string) {
        Ok(decoded_bytes) => decoded_bytes,
        Err(_) => {
            eprintln!("CHAINSAW ERROR: Bad fuel mix (Invalid Base64 string)");
            process::exit(1);
        }
    };

    let mut tape = vec![0u8; TAPE_SIZE];
    let mut pointer: usize = 0;
    let mut instruction_pointer: usize = 0;

    while instruction_pointer < instructions.len() {
        match instructions[instruction_pointer] {
            b'a' => pointer = if pointer == TAPE_SIZE - 1 { 0 } else { pointer + 1 },
            b'b' => pointer = if pointer == 0 { TAPE_SIZE - 1 } else { pointer - 1 },
            b'c' => tape[pointer] = tape[pointer].wrapping_add(1),
            b'd' => tape[pointer] = tape[pointer].wrapping_sub(1),
            b'e' => { print!("{}", tape[pointer] as char); io::stdout().flush().unwrap(); },
            b'f' => {
                let mut buffer = [0; 1];
                if io::stdin().read_exact(&mut buffer).is_ok() { tape[pointer] = buffer[0]; } else { tape[pointer] = 0; }
            },
            b'g' => {
                if tape[pointer] == 0 {
                    let mut loop_nesting = 1;
                    while loop_nesting > 0 {
                        instruction_pointer += 1;
                        if instruction_pointer >= instructions.len() { eprintln!("CHAINSAW ERROR: Chain flew off (unmatched 'g')"); process::exit(1); }
                        match instructions[instruction_pointer] { b'g' => loop_nesting += 1, b'h' => loop_nesting -= 1, _ => {} }
                    }
                }
            },
            b'h' => {
                if tape[pointer] != 0 {
                    let mut loop_nesting = 1;
                    while loop_nesting > 0 {
                        if instruction_pointer == 0 { eprintln!("CHAINSAW ERROR: Kickback fault (unmatched 'h')"); process::exit(1); }
                        instruction_pointer -= 1;
                        match instructions[instruction_pointer] { b'h' => loop_nesting += 1, b'g' => loop_nesting -= 1, _ => {} }
                    }
                }
            },
            other => { eprintln!("CHAINSAW ERROR: Stalled on unrecognized command byte '{}'", other); process::exit(1); }
        }
        instruction_pointer += 1;
    }
}

fn run_level4(code: &str) {
    let mut tape = vec![0u8; TAPE_SIZE];
    let mut pointer: usize = 0;
    let mut source = code.chars().filter(|c| !c.is_whitespace()).collect::<String>();

    let patterns = &[
        r"(?i)^AAGH",
        r"(?i)^A{5,}GH",
        r"(?i)^[aA]+GH",
        r"(?i)^A{5,}H",
        r"(?i)^A{1,4}H",
        r"(?i)^U+GH",
    ];

    let set = RegexSet::new(patterns).unwrap();
    let scream_rules: Vec<Regex> = patterns.iter().map(|p| Regex::new(p).unwrap()).collect();

    while !source.is_empty() {
        let matches: Vec<_> = set.matches(&source).into_iter().collect();

        if let Some(match_index) = matches.first() {
            let matched_regex = &scream_rules[*match_index];
            let matched_text = matched_regex.find(&source).unwrap().as_str();
            
            match *match_index {
                0 => {
                    print!("{}", tape[pointer] as char);
                    io::stdout().flush().unwrap();
                },
                1 => pointer = if pointer == TAPE_SIZE - 1 { 0 } else { pointer + 1 },
                2 => tape[pointer] = tape[pointer].wrapping_sub(1),
                3 => tape[pointer] = tape[pointer].wrapping_add(1),
                4 => pointer = if pointer == 0 { TAPE_SIZE - 1 } else { pointer - 1 },
                5 => {
                    let mut buffer = [0; 1];
                    if io::stdin().read_exact(&mut buffer).is_ok() { tape[pointer] = buffer[0]; } else { tape[pointer] = 0; }
                },
                _ => unreachable!(),
            }
            
            source = source[matched_text.len()..].to_string();
        } else {
            eprintln!("\nFATAL SYSTEM ANGUISH: Unintelligible screaming detected near '{}...'", &source[..std::cmp::min(10, source.len())]);
            process::exit(1);
        }
    }
}