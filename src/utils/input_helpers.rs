// src/utils/input_helpers.rs

use std::io::{self, Write};

pub fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn read_block_height() -> Option<u64> {
    print!("Enter block height: ");
    io::stdout().flush().unwrap();
    let input = read_input();
    match input.trim().parse::<u64>() {
        Ok(height) => Some(height),
        Err(_) => {
            println!("Invalid block height");
            None
        }
    }
}

pub fn read_txid() -> Option<String> {
    print!("Enter transaction ID (txid -> 2cf4f1ab0df25610231a0f4fb7e1e18720a6c555f102b75bb115e99b1ad41241): ");
    io::stdout().flush().unwrap();
    let input = read_input().trim().to_string();
    if !input.is_empty() {
        Some(input)
    } else {
        println!("Invalid transaction ID");
        None
    }
}
