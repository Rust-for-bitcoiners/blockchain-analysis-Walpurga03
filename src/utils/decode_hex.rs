use crate::errors::MyError;
use hex;

// Funktion zum Decodieren von Hex-Strings
pub fn decode_hex_string(hex_str: &str) -> Result<String, MyError> {
    let bytes = hex::decode(hex_str).map_err(|err| MyError::Decode(format!("Failed to decode hex string: {}", err)))?;
    String::from_utf8(bytes).map_err(|err| MyError::Decode(format!("Failed to convert bytes to string: {}", err)))
}