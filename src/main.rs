mod parser;

use flate2::bufread::GzDecoder;
use parser::Parser;
use std::{fs::read, io::Read, thread};

fn main() {
    let map_bytes = read_gzip_to_bytes("retrograde.cmr");

    match map_bytes {
        Some(bytes) => {
            let mut parser = Parser::new(bytes);

            parser.parse_map()
        }
        None => println!("error"),
    }
}

fn read_gzip_to_bytes(path: &str) -> Option<Vec<u8>> {
    if let Ok(compressed_bytes) = read(path) {
        let mut gz = GzDecoder::new(&compressed_bytes[..]);
        let mut bytes = Vec::new();

        match gz.read_to_end(&mut bytes) {
            Ok(_) => Some(bytes),
            Err(_) => None,
        }
    } else {
        None
    }
}