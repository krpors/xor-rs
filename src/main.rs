use std::env;
use std::error::Error;
use std::process;

use base64::{decode, encode};

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
fn str_encode(string_for_encoding: &String) -> String {
    let encoded_string = xor(string_for_encoding);

    encode(encoded_string)
}

/// Decodes the string by xoring every character and base64-decoding it.
fn str_decode(string_for_decoding: &String) -> Result<String, Box<dyn Error>> {
    let b64_decoded = decode(string_for_decoding)?;
    let b64_decoded = String::from_utf8(b64_decoded)?;

    Ok(xor(&b64_decoded))
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
        let s = str_encode(string_for_coding);
        println!("{}: {}", string_for_coding, s);
    } else if oper == "-d" {
        match str_decode(string_for_coding) {
            Ok(s) => println!("{}: {}", string_for_coding, s),
            Err(e) => {
                eprintln!("Could not decode: {}", e);
                process::exit(1);
            }
        }
    } else {
        eprintln!("Invalid operation {}", oper);
        process::exit(1);
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn xor_bytes_properly() {
        let xored = super::xor(&"hello".to_string());
        let b = xored.as_bytes();

        assert_eq!(b.len(), 5);
        assert_eq!(b[0], 55);
        assert_eq!(b[1], 58);
        assert_eq!(b[2], 51);
        assert_eq!(b[3], 51);
        assert_eq!(b[4], 48);
    }

    #[test]
    fn encoding_string() {
        let encoded = str_encode(&"hello world".to_string());
        assert_eq!(encoded, "NzozMzB/KDAtMzs=");
    }

    #[test]
    fn decoding_string() {
        let decoded = str_decode(&"NzozMzB/KDAtMzs=".to_string()).unwrap();
        assert_eq!(decoded, "hello world");
    }

    #[test]
    fn decode_failure_results_in_error() {
        let decoded = str_decode(&"improper base64 string here".to_string());
        assert!(decoded.is_err());
    }
}
