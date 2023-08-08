use bitcoin::{Address, Client, GetBlockTemplateModes, Network, PrivateKey};
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use bitcoin::util::bip32::{ChildNumber, ExtendedPrivKey, ExtendedPubKey};
use bitcoin::network::constants::Network;

pub fn get_private_key_from_phrase(phrase: String) {
    // Validate the phrase
    let mnemonic = Mnemonic::from_phrase(phrase, Language::English).unwrap();
    assert_eq!(phrase, mnemonic.phrase());

    // Convert the phrase to a seed integer
    let seed = Seed::new(&mnemonic, "");

    // Derive the master private key from the seed
    let master = ExtendedPrivKey::new_master(Network::Bitcoin, seed.as_bytes()).unwrap();

    // Derive the first account private key from the master key
    return master
        .ckd_priv(
            &*bitcoin::SECP256K1,
            ChildNumber::from_hardened_idx(0).unwrap(),
        )
        .unwrap();
}

pub fn get_balance_from_private_key(key: String) {
    // Create a private key from a hex string
    let privkey =
        PrivateKey::from_wif("L1r2pKfupJBzXn2S4NXgHCwfBtQYv2MhJZK4gmB7L59D1oe1mTav").unwrap();

    // Generate a compressed P2PKH address from the private key
    let address = Address::p2pkh(&privkey.public_key(), Network::Bitcoin);

    // Print the address
    println!("Address: {}", address);

    // Create a client to connect to a Bitcoin node
    let client = Client::new("http://user:pass@localhost:8332").unwrap();

    // Get the balance of the address in satoshis
    let balance = client.get_address_balance(&address).unwrap();

    return balance.confirmed
}
