// tests/error_tests.rs

use rfb_2_2024_4::errors::MyError;

#[test]
fn test_client_creation_error() {
    let error = MyError::ClientCreation("Test error".to_string());
    assert_eq!(format!("{}", error), "Client creation error: Test error");
}

#[test]
fn test_rpc_error() {
    let error = MyError::Rpc("Test error".to_string());
    assert_eq!(format!("{}", error), "RPC error: Test error");
}

#[test]
fn test_decode_error() {
    let error = MyError::Decode("Test error".to_string());
    assert_eq!(format!("{}", error), "Decode error: Test error");
}
