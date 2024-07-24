use bitcoincore_rpc::{Client, RpcApi};
use crate::errors::MyError;
use crate::utils::decode_hex::decode_hex_string;

/// Funktion zum Abrufen von Block-Informationen
pub fn get_block_info(client: &Client, block_height: u64) -> Result<(), MyError> {
    let block_hash = client.get_block_hash(block_height)
        .map_err(|err| MyError::Rpc(format!("Failed to get block hash: {}", err)))?;
    let block_info = client.get_block_info(&block_hash)
        .map_err(|err| MyError::Rpc(format!("Failed to get block info: {}", err)))?;

    println!("\nBlock Information:");
    println!("  Hash: {}", block_info.hash);
    println!("  Confirmations: {}", block_info.confirmations);
    println!("  Height: {}", block_info.height);
    println!("  Version: {}", block_info.version);
    println!("  Version Hex: {:?}", block_info.version_hex); // Using {:?} for Vec<u8>
    println!("  Merkleroot: {}", block_info.merkleroot);
    println!("  Time: {}", block_info.time);
    println!("  Median Time: {:?}", block_info.mediantime); // Using {:?} for Option<usize>
    println!("  Nonce: {}", block_info.nonce);
    println!("  Bits: {}", block_info.bits);
    println!("  Difficulty: {}", block_info.difficulty);
    println!("  Chainwork: {:?}", block_info.chainwork); // Using {:?} for Vec<u8>
    println!("  Previous Block Hash: {:?}", block_info.previousblockhash); // Using {:?} for Option<BlockHash>
    if let Some(next_block_hash) = block_info.nextblockhash {
        println!("  Next Block Hash: {:?}", next_block_hash); // Using {:?} for Option<BlockHash>
    }

    // Berechne die Zeit, die benötigt wurde, um den Block zu minen
    if let Some(previous_block_hash) = block_info.previousblockhash {
        let previous_block_info = client.get_block_info(&previous_block_hash)
            .map_err(|err| MyError::Rpc(format!("Failed to get previous block info: {}", err)))?;
        let time_diff = block_info.time - previous_block_info.time;

        // Zeit in geeigneter Einheit formatieren
        let (days, remainder) = (time_diff / 86400, time_diff % 86400);
        let (hours, remainder) = (remainder / 3600, remainder % 3600);
        let (minutes, seconds) = (remainder / 60, remainder % 60);

        let mut time_str = String::new();
        if days > 0 {
            time_str.push_str(&format!("{} days ", days));
        }
        if hours > 0 {
            time_str.push_str(&format!("{} hours ", hours));
        }
        if minutes > 0 {
            time_str.push_str(&format!("{} minutes ", minutes));
        }
        if seconds > 0 || time_str.is_empty() {
            time_str.push_str(&format!("{} seconds", seconds));
        }

        println!("  Time taken to mine the block: {}", time_str);
    } else {
        println!("  No information about the previous block available.");
    }

    println!("  Number of transactions in this block: {}\n", block_info.tx.len());

    Ok(())
}

/// Funktion zum Abrufen und Anzeigen der Nachricht im script_sig der ersten Transaktion eines Blocks
pub fn get_block_messages(client: &Client, block_height: u64) -> Result<(), MyError> {
    let block_hash = client.get_block_hash(block_height)
        .map_err(|err| MyError::Rpc(format!("Failed to get block hash: {}", err)))?;
    let block = client.get_block(&block_hash)
        .map_err(|err| MyError::Rpc(format!("Failed to get block: {}", err)))?;

    println!("\nBlock Messages:");
    if let Some(first_tx) = block.txdata.first() {
        for input in &first_tx.input {
            let script_sig_str = format!("{:?}", input.script_sig);
            if let Some(last_space) = script_sig_str.rfind(' ') {
                let mut hex_str = script_sig_str[last_space + 1..].trim().to_string();
                // Bereinigen des Hex-Strings, um nur gültige Hex-Zeichen zu enthalten
                hex_str.retain(|c| c.is_digit(16));
                if let Ok(message) = decode_hex_string(&hex_str) {
                    println!("  Message in script_sig: {}", message);
                } else {
                    println!("  Failed to decode hex string.");
                }
            } else {
                println!("  No hex string found in script_sig.");
            }
        }
    } else {
        println!("  No transactions found in this block.");
    }

    Ok(())
}
