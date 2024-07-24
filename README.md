# Blockchain-Analyse-Bibliothek

Diese Rust-Bibliothek ermöglicht die Interaktion mit einem Bitcoin-Node über das Bitcoin Core RPC-Interface. Sie bietet Funktionen zum Abrufen von Blockchain-Informationen, Block-Informationen und OP_RETURN-Daten aus Transaktionen. Ziel dieses Projekts ist es, eine flexible und leistungsstarke Bibliothek für Entwickler bereitzustellen, die mit der Bitcoin-Blockchain arbeiten möchten.

## Projektübersicht

### Zielsetzung

Die Blockchain-Analyse-Bibliothek ist darauf ausgelegt, Entwicklern die Werkzeuge zur Verfügung zu stellen, um verschiedene Aspekte der Bitcoin-Blockchain zu analysieren. Die Bibliothek umfasst Funktionen zur Abfrage und Verarbeitung von Blockchain- und Blockdaten sowie zur Extraktion von eingebetteten Nachrichten und OP_RETURN-Daten.

### Hauptfunktionen

1. **Abrufen von Blockchain-Informationen**: 
   - Funktionen zum Abrufen allgemeiner Informationen über die Bitcoin-Blockchain, einschließlich der aktuellen Blockhöhe, der Schwierigkeit, der Anzahl der Bestätigungen und anderer relevanter Metriken.

2. **Abrufen von Block-Informationen**: 
   - Detaillierte Informationen über spezifische Blöcke, einschließlich des Block-Hashes, der Anzahl der Transaktionen, der Zeit, die benötigt wurde, um den Block zu minen, und andere spezifische Blockdaten.

3. **Nachrichten im `script_sig` der ersten Transaktion eines Blocks extrahieren**:
   - Funktionen zum Decodieren und Anzeigen von Nachrichten, die in den `script_sig`-Feldern der ersten Transaktionen eines Blocks eingebettet sind.

4. **OP_RETURN-Daten in Transaktionen extrahieren**:
   - Funktionen zum Abrufen und Anzeigen von Daten, die in OP_RETURN-Skripten innerhalb von Transaktionen gespeichert sind.

## Projektstruktur

Die Bibliothek ist in mehrere Module unterteilt, um die Codebasis sauber und wartbar zu halten. 

### Verzeichnisstruktur

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

### Module

- **blockchain**:
  - `blockchain_info.rs`: Enthält Funktionen zum Abrufen von allgemeinen Informationen über die Bitcoin-Blockchain.
  - `block_info.rs`: Enthält Funktionen zum Abrufen detaillierter Informationen über spezifische Blöcke und zum Extrahieren von Nachrichten im `script_sig`.
  
- **rpc**:
  - `rpc_client.rs`: Beinhaltet die Logik zur Erstellung und Verwaltung des RPC-Clients für die Kommunikation mit dem Bitcoin-Node.
  
- **utils**:
  - `decode_hex.rs`: Bietet Funktionen zum Decodieren von Hex-Strings.
  - `input_helpers.rs`: Enthält Hilfsfunktionen zum Lesen und Verarbeiten von Benutzereingaben aus der Konsole.
  
- **errors.rs**: Definiert benutzerdefinierte Fehler, die in der Bibliothek verwendet werden.

## Abhängigkeiten

1. **bitcoincore-rpc**
   - Version: `0.19.0`
   - Zweck: Ermöglicht die Interaktion mit einem Bitcoin-Node über das Bitcoin Core RPC-Interface. Diese Bibliothek stellt Funktionen zur Verfügung, um verschiedene Daten vom Node abzurufen und Aktionen durchzuführen, wie z.B. Block- und Transaktionsinformationen abzurufen.

2. **chrono**
   - Version: `0.4.38`
   - Zweck: Bietet leistungsstarke und flexible Datums- und Zeitmanipulationen, die zur Berechnung und Darstellung von Zeitdifferenzen verwendet werden.

3. **dotenv**
   - Version: `0.15.0`
   - Zweck: Ermöglicht das Laden von Umgebungsvariablen aus einer `.env`-Datei. Dies ist nützlich, um sensible Konfigurationsdaten wie API-Schlüssel oder Datenbankverbindungsinformationen sicher zu speichern und zu laden.

4. **lazy_static**
   - Version: `1.5.0`
   - Zweck: Ermöglicht die Erstellung von statischen Variablen in Rust, die einmal initialisiert und während der gesamten Programmlaufzeit verwendet werden. Diese Variablen werden nur bei Bedarf initialisiert und bieten eine effiziente Möglichkeit, globale Zustände zu verwalten.

## Tests

Die Bibliothek umfasst sowohl Integrationstests als auch Unittests, um die Funktionalität und Zuverlässigkeit der verschiedenen Module und Funktionen sicherzustellen. 

- **Integrationstests**: Überprüfen die korrekte Integration der verschiedenen Module und Funktionen sowie deren Zusammenspiel mit einem Bitcoin-Node.
- **Unittests**: Überprüfen die korrekte Funktionalität einzelner Funktionen und Module isoliert.

Um die Tests auszuführen, verwenden Sie den folgenden Befehl:

```sh
cargo test
```

## Fazit

Die Blockchain-Analyse-Bibliothek ist ein leistungsstarkes Werkzeug für Entwickler, die mit der Bitcoin-Blockchain arbeiten möchten. Durch die Modularität und die umfangreiche Nutzung von Rusts sicherheits- und leistungsorientierten Features bietet die Bibliothek eine zuverlässige und flexible Grundlage für Blockchain-Analyseprojekte.