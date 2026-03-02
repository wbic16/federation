//! Base256 Phonetic Encoding CLI
//!
//! Usage:
//!   b256 encode <hex>       Encode hex bytes to syllables
//!   b256 decode <syllables> Decode syllables to hex bytes
//!   b256 table              Print full encoding table
//!   b256 help               Show this help

use base256_phonetic::{decode_string, encode_bytes, generate_table};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "encode" | "e" => {
            if args.len() < 3 {
                eprintln!("Usage: b256 encode <hex>");
                eprintln!("Example: b256 encode DEADBEEF");
                std::process::exit(1);
            }
            encode_hex(&args[2]);
        }
        "decode" | "d" => {
            if args.len() < 3 {
                eprintln!("Usage: b256 decode <syllables>");
                eprintln!("Example: b256 decode \"sof nod pof tom\"");
                std::process::exit(1);
            }
            let syllables = args[2..].join(" ");
            decode_syllables(&syllables);
        }
        "table" | "t" => {
            print_table();
        }
        "help" | "-h" | "--help" => {
            print_help();
        }
        _ => {
            // Try to auto-detect: if it looks like hex, encode; otherwise decode
            let input = &args[1];
            if looks_like_hex(input) {
                encode_hex(input);
            } else {
                let syllables = args[1..].join(" ");
                decode_syllables(&syllables);
            }
        }
    }
}

fn looks_like_hex(s: &str) -> bool {
    let clean = s.trim_start_matches("0x").trim_start_matches("0X");
    !clean.is_empty() && clean.chars().all(|c| c.is_ascii_hexdigit())
}

fn encode_hex(hex: &str) {
    let clean = hex
        .trim_start_matches("0x")
        .trim_start_matches("0X")
        .replace(' ', "");

    if clean.len() % 2 != 0 {
        eprintln!("Error: hex string must have even length");
        std::process::exit(1);
    }

    let bytes: Result<Vec<u8>, _> = (0..clean.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&clean[i..i + 2], 16))
        .collect();

    match bytes {
        Ok(data) => {
            let encoded = encode_bytes(&data);
            println!("{}", encoded);
        }
        Err(e) => {
            eprintln!("Error parsing hex: {}", e);
            std::process::exit(1);
        }
    }
}

fn decode_syllables(input: &str) {
    match decode_string(input) {
        Ok(bytes) => {
            let hex: String = bytes.iter().map(|b| format!("{:02X}", b)).collect();
            println!("0x{}", hex);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn print_table() {
    println!("Base256 Phonetic Encoding Table");
    println!("================================");
    println!();
    println!("     0    1    2    3    4    5    6    7    8    9    A    B    C    D    E    F");
    println!("   +----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+");

    let table = generate_table();
    let onsets = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'r', 's', 't', 'w'];

    for (row, onset) in onsets.iter().enumerate() {
        print!(" {} |", onset.to_ascii_uppercase());
        for col in 0..16 {
            let idx = row * 16 + col;
            print!(" {}|", table[idx].1);
        }
        println!();
    }

    println!("   +----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+");
}

fn print_help() {
    println!("Base256 Phonetic Encoding (b256)");
    println!("================================");
    println!();
    println!("Encode bytes as pronounceable syllables.");
    println!();
    println!("USAGE:");
    println!("  b256 encode <hex>        Encode hex to syllables");
    println!("  b256 decode <syllables>  Decode syllables to hex");
    println!("  b256 table               Print full encoding table");
    println!("  b256 <auto>              Auto-detect and convert");
    println!();
    println!("EXAMPLES:");
    println!("  b256 encode DEADBEEF");
    println!("    → sof nod pof tom");
    println!();
    println!("  b256 decode \"sof nod pof tom\"");
    println!("    → 0xDEADBEEF");
    println!();
    println!("  b256 DEADBEEF            (auto-encode)");
    println!("  b256 sof nod pof tom     (auto-decode)");
    println!();
    println!("ENCODING:");
    println!("  Each byte maps to a CVC syllable:");
    println!("    onset  (high nibble) : b c d f g h j k l m n p r s t w");
    println!("    vowel  (bits 2-3)    : a e i o");
    println!("    coda   (bits 0-1)    : c d f m");
    println!();
    println!("VERSION: 0.1.0 (R23W23)");
}
