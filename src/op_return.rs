use bitcoincore_rpc::{RpcApi};
use bitcoin::blockdata::script::{Instruction, Script};
use bitcoin::blockdata::opcodes::all;
use bitcoin::blockdata::transaction::Transaction;
use bitcoin::consensus::encode::deserialize;
use crate::errors::MyError;
use crate::rpc::rpc_client::CLIENT;
use hex;

/// Funktion zum Überprüfen und Extrahieren von OP_RETURN-Daten aus einem Script
fn extract_op_return_data(script: &Script) -> Option<Vec<u8>> {
    let instructions = script.instructions_minimal();
    let mut iter = instructions.into_iter();
    if let Some(Ok(Instruction::Op(all::OP_RETURN))) = iter.next() {
        if let Some(Ok(Instruction::PushBytes(data))) = iter.next() {
            return Some(data.to_vec());
        }
    }
    None
}
/// Funktion zum Abrufen und Anzeigen der OP_RETURN-Daten einer Transaktion
pub fn get_op_return_data(txid: &str) -> Result<(), MyError> {
    let client = CLIENT.lock().unwrap();
    let txid = txid.parse().map_err(|err| MyError::Decode(format!("Failed to parse txid: {}", err)))?;
    let raw_tx = client.get_raw_transaction_hex(&txid, None)
        .map_err(|err| MyError::Rpc(format!("Failed to get raw transaction: {}", err)))?;
    let tx_hex = hex::decode(raw_tx).map_err(|err| MyError::Decode(format!("Failed to decode hex: {}", err)))?;
    let tx: Transaction = deserialize(&tx_hex).map_err(|err| MyError::Decode(format!("Failed to deserialize transaction: {}", err)))?;
    for output in tx.output {
        if let Some(op_return_data) = extract_op_return_data(&output.script_pubkey) {
            println!("OP_RETURN data: {:?}", String::from_utf8_lossy(&op_return_data));
        }
    }
    Ok(())
}
