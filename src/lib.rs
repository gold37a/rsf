use bdk::database::{BatchDatabase, Database};
use rand::{thread_rng, RngCore};
use std::fs::File;
use std::io::Write;
use std::{fs, io};

use bdk::{bitcoin::Network, database::MemoryDatabase, Wallet};
use bip39::Mnemonic;
use bitcoin::bip32::ExtendedPrivKey;
use bitcoin::network::constants::Network as BitcoinNetwork;
// use rand::seq::SliceRandom;

// pub fn get_balance_from_phrase(phrase: &str) -> Result<u64, String> {
//     // Validate the phrase
//     match Mnemonic::parse_in(Language::English, phrase) {
//         Ok(m) => {
//             // Derive the master private key from the seed
//             match ExtendedPrivKey::new_master(BitcoinNetwork::Bitcoin, &m.to_seed("")) {
//                 Ok(epk) =>  match get_wallet_from_master_key(epk) {
//         Ok(w) => match w.get_balance() {
//             Ok(balance) => Ok(balance.confirmed),
//             Err(e) => Err(format!("Encountered an error while trying to get balance of wallet with mnemonic phrase: {}. Error: {}", phrase, e.to_string()))
//         }
//         Err(e) => Err(format!("Encountered an error while trying to get wallet with mnemonic phrase: {}. Error: {}", phrase, e.to_string()))
//     }
//                 Err(e) => Err(format!("Encountered an error trying to get the master key for the following phrase: {}. Error: {}", phrase, e.to_string()))
//             }
//         },
//         Err(e) => Err(format!("Encountered an error trying to parse the following phrase (while trying to get its master key): {}. Error: {}", phrase, e.to_string()))
//     }
// }

// pub fn create_mnemonic_phrase() -> Result<String, String> {
//     match get_words() {
//         Ok(words) => {
//             let mut rng = rand::thread_rng();
//             let amount = *vec![12, 15, 18, 21, 24].choose(&mut rng).unwrap();
//             let selected_words: Vec<_> = words
//                 .choose_multiple(&mut rng, amount)
//                 .map(|w| w.to_string())
//                 .collect();
//             Ok(selected_words.join(" "))
//         }
//         Err(e) => Err(format!("Error while in create_mnemonic_phrase(): {}", e)),
//     }
// }

pub fn master_key_from_mnemonic(mnemonic: Mnemonic) -> Result<ExtendedPrivKey, String> {
    match ExtendedPrivKey::new_master(BitcoinNetwork::Bitcoin, &mnemonic.to_seed("")) {
        Ok(epk) => Ok(epk),
        Err(e) => Err(format!(
            "Encountered an error trying to get the master key. Error: {}",
            e.to_string()
        )),
    }
}

pub fn wallet_from_master_key(
    key: ExtendedPrivKey,
) -> Result<Wallet<impl Database + BatchDatabase>, String> {
    match Wallet::new(
        &format!("wpkh({}/84'/0'/0'/0/*)", key),
        None,
        Network::Bitcoin,
        MemoryDatabase::default(),
    ) {
        Ok(w) => Ok(w),
        Err(e) => Err(format!("Error in get_wallet: {}", e.to_string())),
    }
}

pub fn wallet_balance(wallet: Wallet<impl Database + BatchDatabase>) -> Result<u64, String> {
    match wallet.get_balance() {
        Ok(balance) => Ok(balance.confirmed),
        Err(e) => Err(format!(
            "Encountered an error while trying to get balance of wallet. Error: {}",
            e.to_string()
        )),
    }
}

pub fn words() -> io::Result<Vec<String>> {
    Ok(fs::read_to_string("english_words.txt")?
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>())
}

pub fn create_mnemonic() -> Result<Mnemonic, String> {
    let mut entropy = [0u8; 32];
    thread_rng().fill_bytes(&mut entropy);
    match Mnemonic::from_entropy(&entropy) {
        Ok(m) => Ok(m),
        Err(e) => Err(format!(
            "Error while trying to create mnemonic code: {}",
            e.to_string()
        )),
    }
}

pub fn create_wallet_get_balance() -> Result<(String, u64), String> {
    let mnemonic = create_mnemonic()?;
    Ok((
        mnemonic.to_string(),
        wallet_balance(wallet_from_master_key(master_key_from_mnemonic(mnemonic)?)?)?,
    ))
}

pub fn sort_balance(
    account: (String, u64),
    above_10000_file: &mut File,
    above_1000_file: &mut File,
    above_100_file: &mut File,
    below_100_file: &mut File
) {
    match account {
        (mnemonic, balance) if balance > 10000 => {
            writeln!(above_10000_file, "wallet mnemonic: {}\nbalance: {}\n\n", mnemonic, balance).expect(&format!(
                "Encountered error writing `{} : {}` to above_10000_file",
                mnemonic, balance
            ));
        }
        (mnemonic, balance) if balance > 1000 => {
            writeln!(above_1000_file, "wallet mnemonic: {}\nbalance: {}\n\n", mnemonic, balance).expect(&format!(
                "Encountered error writing `{} : {}` to above_1000_file",
                mnemonic, balance
            ));
        }
        (mnemonic, balance) if balance > 100 => {
            writeln!(above_100_file, "wallet mnemonic: {}\nbalance: {}\n\n", mnemonic, balance).expect(&format!(
                "Encountered error writing `{} : {}` to above_100_file",
                mnemonic, balance
            ));
        }
        (mnemonic,balance) => {
            writeln!(below_100_file, "wallet mnemonic: {}\nbalance: {}\n\n", mnemonic, balance).expect(&format!(
                "Encountered error writing `{} : {}` to below_100_file",
                mnemonic, balance
            ));
        }
    }
}
