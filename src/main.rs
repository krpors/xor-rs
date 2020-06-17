extern crate base64;

use base64::{decode, encode};

use std::env;
use std::process;

const MAGIC: u8 = 0x5f;

fn print_help() {
    let doc = "Usage: xor [-h] [-e STRING] [-d BASE64STRING]

Encodes or decodes Websphere obfuscated strings. Websphere obfuscates passwords
by doing an xor operation on every character in a string, then encoding it using
base64. This utility will encode and decode such strings.

Encoding:

    $ xor -e \"hello world\"
    hello world: NzozMzB/KDAtMzs=

Decoding:

    $ xor -d \"NzozMzB/KDAtMzs=\"
    NzozMzB/KDAtMzs=: hello world";

    eprintln!("{}", doc);
}

/// Runs an xor operation on every byte with the [`MAGIC`] value. Allocates
/// a new String and returns it.
fn xor(s: &String) -> String {
    let mut bleh = String::new();

    let bytes = s.bytes();
    for b in bytes {
        bleh.push((b ^ MAGIC) as char);
    }

    return bleh;
}

/// Encodes the string by xoring every character and base64-encoding it.
fn str_encode(string_for_encoding: &String) {
    let encoded_string = xor(string_for_encoding);
    let b64_encoded = encode(encoded_string);
    println!("{}: {}", string_for_encoding, b64_encoded);
}

/// Decodes the string by xoring every character and base64-decoding it.
fn str_decode(string_for_decoding: &String) {
    let b64_decoded = decode(string_for_decoding);
    let b64_decoded = match b64_decoded {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Incorrect base-64 encoded string supplied.");
            process::exit(1);
        }
    };

    let b64_decoded = String::from_utf8(b64_decoded).unwrap();

    let decoded_string = xor(&b64_decoded);

    println!("{}: {}", string_for_decoding, decoded_string);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        print_help();
        process::exit(1);
    }


    let oper = &args[1];

    if oper == "-h" {
        print_help();
        process::exit(0);
    }

    if args.len() != 3 {
        print_help();
        process::exit(1);
    }

    let string_for_coding = &args[2];

    if oper == "-e" {
        str_encode(string_for_coding);
    } else if oper == "-d" {
        str_decode(string_for_coding);
    } else {
        eprintln!("Invalid operation {}", oper);
        process::exit(1);
    }
}
