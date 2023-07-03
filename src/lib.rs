use sha3::{Digest, Sha3_256};
use std::convert::TryInto;

pub enum Arg {
    Address(String),
    Uint(u64),
    String(Vec<u8>),
}

// Helper function to calculate sha3_256 hash
fn sha3_256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    hasher.update(input);
    let result = hasher.finalize();
    let mut output = [0u8; 32];
    output.copy_from_slice(result.as_slice());
    output
}

// Helper function to encode integer to 32-byte big-endian
fn encode_int(value: u64) -> Vec<u8> {
    let mut buffer = vec![0u8; 32];
    let bytes = value.to_be_bytes();
    buffer.splice(24..32, bytes.iter().cloned());
    buffer
}

// Helper function to encode address to 20-byte big-endian with 12 bytes of padding
fn encode_address(address: &str) -> Vec<u8> {
    let mut buffer = vec![0u8; 32];
    let bytes = hex::decode(address).expect("Invalid address");
    buffer.splice(12..32, bytes.iter().cloned());
    buffer
}

// Helper function to encode string
fn encode_string(input: &[u8]) -> Vec<u8> {
    let len = input.len();
    let mut buffer = encode_int(len.try_into().unwrap());
    buffer.extend_from_slice(input);
    while buffer.len() % 32 != 0 {
        buffer.push(0);
    }
    buffer
}

pub fn encode_fn_sig_and_args(fn_selector: &str, args: Vec<Arg>) -> Vec<u8> {
    let mut data = vec![];

    // Get first 4 bytes of sha3_256 hash of function signature
    let hash = sha3_256(fn_selector.as_bytes());
    data.extend_from_slice(&hash[0..4]);

    // Encode arguments
    for arg in args {
        match arg {
            Arg::Address(a) => {
                data.extend_from_slice(&encode_address(&a));
            }
            Arg::Uint(i) => {
                data.extend_from_slice(&encode_int(i));
            }
            Arg::String(s) => {
                data.extend_from_slice(&encode_string(&s));
            }
        }
    }

    data
}
