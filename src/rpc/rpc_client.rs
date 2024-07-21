use bitcoincore_rpc::{Auth, Client};
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CLIENT: Mutex<Client> = {
        let rpc_url = std::env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL must be set");
        let rpc_user = std::env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
        let rpc_password = std::env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set");
        
        Mutex::new(Client::new(
            &rpc_url,
            Auth::UserPass(rpc_user, rpc_password),
        ).expect("Failed to create client"))
    };
}
