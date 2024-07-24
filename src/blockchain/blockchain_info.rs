use bitcoincore_rpc::RpcApi;
use crate::errors::MyError;
use crate::rpc::rpc_client::CLIENT;

// Funktion zum Abrufen der Blockchain-Informationen
pub fn get_blockchain_info() -> Result<(), MyError> {
    let client = CLIENT.lock().unwrap();
    let info = client.get_blockchain_info()
        .map_err(|err| MyError::Rpc(format!("Failed to get blockchain info: {}", err)))?;
    println!("Blockchain Information:");
    println!("  Chain: {}", info.chain);
    println!("  Blocks: {}", info.blocks);
    println!("  Headers: {}", info.headers);
    println!("  Best Block Hash: {}", info.best_block_hash);
    println!("  Difficulty: {}", info.difficulty);
    println!("  Median Time: {}", info.median_time);
    println!("  Verification Progress: {}", info.verification_progress);
    println!("  Initial Block Download: {}", info.initial_block_download);
    println!("  Chain Work: {:?}", info.chain_work);
    println!("  Size on Disk: {}", info.size_on_disk);
    println!("  Pruned: {}", info.pruned);
    if let Some(prune_height) = info.prune_height {
        println!("  Prune Height: {}", prune_height);
    }
    println!("  Warnings: {:?}", info.warnings);
    Ok(())
}
