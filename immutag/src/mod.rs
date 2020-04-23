pub mod bitcoin;

use bitcoin::wallet;

pub fn mnemonic_to_xpriv(mnemonic: Vec<String>, language: wallet::Language) -> wallet::ExtendedKey {
    wallet::mnemonic_to_xpriv(mnemonic, language)
}
