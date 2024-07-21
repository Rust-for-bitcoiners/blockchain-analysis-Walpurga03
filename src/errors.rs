use std::fmt;

/// Enum zur Definition benutzerdefinierter Fehler in der Bibliothek
#[derive(Debug)]
pub enum MyError {
    /// Fehler bei der Erstellung des RPC-Clients
    ClientCreation(String),
    /// Fehler bei einem RPC-Aufruf
    Rpc(String),
    /// Fehler beim Dekodieren von Daten
    Decode(String),
}

impl fmt::Display for MyError {
    /// Definiert, wie die Fehlernachrichten formatiert werden sollen
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::ClientCreation(msg) => write!(f, "Client creation error: {}", msg),
            MyError::Rpc(msg) => write!(f, "RPC error: {}", msg),
            MyError::Decode(msg) => write!(f, "Decode error: {}", msg),
        }
    }
}

impl std::error::Error for MyError {}
