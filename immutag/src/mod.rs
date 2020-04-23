pub mod bitcoin;

use bitcoin::wallet;
use wallet::{ Language, ExtendedPrivateKey, ExtendedPublicKey };

pub fn mnemonic_to_xpriv(mnemonic: Vec<String>, language: Language) -> ExtendedPrivateKey {
    wallet::mnemonic_to_xpriv(mnemonic, language)
}

pub fn mnemonic_to_xpub(mnemonic: Vec<String>, language: Language) -> ExtendedPublicKey {
    wallet::mnemonic_to_xpub(mnemonic, language)
}
