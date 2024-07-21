use rfb_2_2024_4::blockchain::blockchain_info::get_blockchain_info;
use rfb_2_2024_4::blockchain::block_info::{get_block_info, get_block_messages};
use rfb_2_2024_4::op_return::get_op_return_data;
use dotenv::dotenv;

#[test]
fn test_get_blockchain_info() {
    dotenv().ok();
    let result = get_blockchain_info();
    assert!(result.is_ok());
}

#[test]
fn test_get_block_info() {
    dotenv().ok();
    let block_height = 170;
    let result = get_block_info(block_height);
    assert!(result.is_ok());
}

#[test]
fn test_get_block_messages() {
    dotenv().ok();
    let block_height = 57043;
    let result = get_block_messages(block_height);
    assert!(result.is_ok());
}

#[test]
fn test_get_op_return_data() {
    dotenv().ok();
    let txid = "a1075db55d416d3ca199f55b6084e2115b9345e16c5cf302fc80e9d5fbf5d48d";
    let result = get_op_return_data(txid);
    
    match result {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("Error: {:?}", e);
            assert!(false, "Test failed with error: {:?}", e);
        }
    }
}
