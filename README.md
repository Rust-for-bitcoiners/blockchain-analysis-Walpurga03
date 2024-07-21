# Blockchain-Analyse-Bibliothek

Diese Bibliothek bietet Funktionen zur Interaktion mit der Bitcoin-Blockchain, einschließlich des Abrufens von Blockchain-Informationen, Block-Informationen und OP_RETURN-Daten.

## Inhaltsverzeichnis

1. [Einführung](#einführung)
2. [Installation](#installation)
3. [Verwendung](#verwendung)
4. [Projektstruktur](#projektstruktur)
5. [Abhängigkeiten](#abhängigkeiten)
6. [Tests](#tests)
7. [Lizenz](#lizenz)

## Einführung

Diese Rust-Bibliothek ermöglicht die Kommunikation mit einem Bitcoin-Node über das Bitcoin Core RPC-Interface. Sie können Informationen über die Blockchain und spezifische Blöcke abrufen sowie Nachrichten und OP_RETURN-Daten aus Transaktionen extrahieren.

## Installation

1. **Klonen Sie das Repository:**
    ```sh
    git clone https://github.com/xxxx
    cd xxx
    ```

2. **Abhängigkeiten installieren:**
    Stellen Sie sicher, dass Sie Rust und Cargo installiert haben. Installieren Sie die notwendigen Abhängigkeiten durch Ausführen von:
    ```sh
    cargo build
    ```

3. **Umgebungsvariablen konfigurieren:**
    Erstellen Sie eine `.env`-Datei im Projektverzeichnis und fügen Sie die folgenden Variablen hinzu:
    ```sh
    BITCOIN_RPC_URL=http://localhost:8332
    BITCOIN_RPC_USER=user
    BITCOIN_RPC_PASSWORD=password
    ```

## Verwendung

1. **Starten Sie das Hauptprogramm:**
    ```sh
    cargo run
    ```

2. **Menüoptionen:**
    Das Programm bietet ein einfaches Menü zur Auswahl der folgenden Optionen:
    - `1`: Blockchain-Informationen abrufen
    - `2`: Block-Informationen abrufen
    - `3`: Nachrichten im Block abrufen
    - `4`: OP_RETURN-Daten in einer Transaktion abrufen
    - `0`: Beenden

## Projektstruktur

```plaintext
.
├── src
│   ├── blockchain
│   │   ├── blockchain_info.rs
│   │   ├── block_info.rs
│   ├── rpc
│   │   ├── rpc_client.rs
│   ├── utils
│   │   ├── decode_hex.rs
│   │   ├── input_helpers.rs
│   ├── errors.rs
│   ├── lib.rs
│   ├── main.rs
├── tests
│   ├── integration_test.rs
│   ├── unit_test.rs
│   ├── error_tests.rs
├── .env
├── Cargo.toml
├── README.md