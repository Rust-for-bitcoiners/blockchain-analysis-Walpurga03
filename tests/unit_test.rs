// tests/unit_test.rs

use rfb_2_2024_4::utils::decode_hex::decode_hex_string;
use rfb_2_2024_4::errors::MyError;

#[test]
fn test_decode_hex_string() {
    let hex_str = "68656c6c6f";
    let result = decode_hex_string(hex_str);
    if let Ok(decoded_str) = result {
        assert_eq!(decoded_str, "hello".to_string());
    } else {
        panic!("Decoding failed for valid hex string");
    }

    let invalid_hex_str = "invalid";
    let result = decode_hex_string(invalid_hex_str);
    assert!(matches!(result, Err(MyError::Decode(_))));
}
