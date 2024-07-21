use rfb_2_2024_4::blockchain::blockchain_info::get_blockchain_info;
use rfb_2_2024_4::blockchain::block_info::{get_block_info, get_block_messages};
use rfb_2_2024_4::op_return::get_op_return_data;
use rfb_2_2024_4::utils::input_helpers::{read_input, read_block_height, read_txid};
use rfb_2_2024_4::rpc::rpc_client::CLIENT;
use std::io::{self, Write};

fn main() {
    dotenv::dotenv().ok();  // Lädt Umgebungsvariablen aus einer .env Datei

    loop {
        println!("Menu:");
        println!("1: Blockchain Info");
        println!("2: Block Info");
        println!("3: Block Messages");
        println!("4: OP_RETURN Data in Transaction");
        println!("0: Quit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();  // Stellt sicher, dass die Ausgabe sofort angezeigt wird

        let choice = read_input().trim().to_string();  // Liest die Benutzereingabe

        match choice.as_str() {
            "1" => {
                if let Err(e) = get_blockchain_info() {  // Ruft Blockchain-Informationen ab
                    println!("Error: {}", e);
                }
            }
            "2" => {
                if let Some(height) = read_block_height() {  // Liest die Blockhöhe vom Benutzer
                    if let Err(e) = get_block_info(&CLIENT.lock().unwrap(), height) {  // Ruft Block-Informationen ab
                        println!("Error: {}", e);
                    }
                }
            }
            "3" => {
                if let Some(height) = read_block_height() {  // Liest die Blockhöhe vom Benutzer
                    if let Err(e) = get_block_messages(&CLIENT.lock().unwrap(), height) {  // Ruft Nachrichten im Block ab
                        println!("Error: {}", e);
                    }
                }
            }
            "4" => {
                if let Some(txid) = read_txid() {  // Liest die Transaktions-ID vom Benutzer
                    if let Err(e) = get_op_return_data(&txid) {  // Ruft OP_RETURN-Daten ab
                        println!("Error: {}", e);
                    }
                }
            }
            "0" => break,  // Beendet die Schleife und damit das Programm
            _ => println!("Invalid choice!"),  // Ungültige Auswahl
        }
    }
}
