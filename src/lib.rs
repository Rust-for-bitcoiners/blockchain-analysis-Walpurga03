/// Hauptmodul für die Blockchain-Analyse-Bibliothek
extern crate lazy_static;

/// Enthält benutzerdefinierte Fehlerdefinitionen und -implementierungen
pub mod errors;

/// Enthält allgemeine Hilfsfunktionen, z.B. für Hex-Dekodierung und Benutzereingaben
pub mod utils {
    /// Funktionen zur Hex-Dekodierung
    pub mod decode_hex;
    /// Funktionen zur Handhabung von Benutzereingaben
    pub mod input_helpers;
}

/// Funktionen und Strukturen zur Blockchain-Analyse
pub mod blockchain;

/// Funktionen zur Interaktion mit einem Bitcoin-Node über RPC
pub mod rpc;

/// Funktionen zur Verarbeitung und Analyse von OP_RETURN Daten in Transaktionen
pub mod op_return;