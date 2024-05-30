use base64ct::{Base64, Encoding};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::prelude::*;
use std::io::Cursor;

pub fn compress_string(input: &str) -> String {
    // Convert string to bytes
    let string_bytes = input.as_bytes();

    // Compress bytes
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(string_bytes)
        .expect("Failed to write bytes");
    let compressed_bytes = encoder.finish().expect("Failed to compress bytes");

    // Encode compressed bytes using base64ct
    let encoded_compressed_string = Base64::encode_string(&compressed_bytes);
    return encoded_compressed_string;
}

pub fn uncompress_string(input: &str) -> String {
    // Decode the Base64 encoded input string
    let decoded_bytes = Base64::decode_vec(input).expect("Failed to decode Base64");

    // Create a Cursor to read the decoded bytes
    let cursor = Cursor::new(decoded_bytes);

    // Create a ZlibDecoder to decompress the data
    let mut decoder = ZlibDecoder::new(cursor);

    // Create a buffer to hold the decompressed data
    let mut decompressed_bytes = Vec::new();

    // Read the decompressed data into the buffer
    decoder
        .read_to_end(&mut decompressed_bytes)
        .expect("Failed to read decompressed data");

    // Convert the decompressed bytes to a UTF-8 string
    let result_string =
        String::from_utf8(decompressed_bytes).expect("Failed to convert bytes to string");

    // Return the resulting string
    result_string
}
