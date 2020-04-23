pub mod bitcoin;

use bitcoin::wallet;
use wallet::{ Language, ExtendedPrivateKey };

pub fn mnemonic_to_xpriv(mnemonic: Vec<String>, language: Language) -> ExtendedPrivateKey {
    wallet::mnemonic_to_xpriv(mnemonic, language)
}
