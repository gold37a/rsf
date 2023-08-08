use bdk::{bitcoin::Network, database::MemoryDatabase, Wallet};
use bip39::{Language, Mnemonic};
use bitcoin::bip32::ExtendedPrivKey;
use bitcoin::network::constants::Network as BitcoinNetwork;

pub fn get_balance_from_phrase(phrase: &str) -> Result<u64, String> {
    // Validate the phrase
    match Mnemonic::parse_in(Language::English, phrase) {
        Ok(m) => {
            // Derive the master private key from the seed
            match ExtendedPrivKey::new_master(BitcoinNetwork::Bitcoin, &m.to_seed("")) {
                Ok(epk) =>  match Wallet::new(&format!("wpkh({}/84'/0'/0'/0/*)", epk), None, Network::Bitcoin, MemoryDatabase::default(), ) {
        Ok(w) => match w.get_balance() {
            Ok(balance) => Ok(balance.confirmed),
            Err(e) => Err(format!("Encountered an error while trying to get balance of wallet with mnemonic phrase: {}. Error: {}", phrase, e.to_string()))
        }
        Err(e) => Err(format!("Encountered an error while trying to get wallet with mnemonic phrase: {}. Error: {}", phrase, e.to_string()))
    }
                Err(e) => Err(format!("Encountered an error trying to get the master key for the following phrase: {}. Error: {}", phrase, e.to_string()))
            }
        },
        Err(e) => Err(format!("Encountered an error trying to parse the following phrase (while trying to get its master key): {}. Error: {}", phrase, e.to_string()))
    }
}
