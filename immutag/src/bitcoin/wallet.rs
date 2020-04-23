use std::fmt;
use std::str::FromStr;
use immutag_bitcoin;
use immutag_file;
use immutag_bitcoin::sv_wallet;
use immutag_bitcoin::is_private_key_valid;

pub use sv_wallet::ExtendedKey as ExtendedKey;

#[derive(Debug, PartialEq, Clone)]
pub struct ExtendedPrivateKey {
    value: String
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExtendedPublicKey {
    value: String
}

/// Language of bitcoin wordlist
pub enum Language {
    ChineseSimplified,
    ChineseTraditional,
    English,
    French,
    Italian,
    Japanese,
    Korean,
    Spanish,
}

pub fn mnemonic_to_xpriv(mnemonic: Vec<String>, language: Language) -> ExtendedPrivateKey {
    let key = mnemonic_to_xkey(mnemonic, language);
    let xpriv: ExtendedPrivateKey = key.encode().parse().unwrap();

    xpriv
}

pub fn mnemonic_to_xpub(mnemonic: Vec<String>, language: Language) -> ExtendedPublicKey {
    let key = mnemonic_to_xkey(mnemonic, language);
    let xpub_str = key.extended_public_key().unwrap().encode();
    let xpub: ExtendedPublicKey = xpub_str.parse().unwrap();

    xpub
}

pub fn mnemonic_to_xkey(mnemonic: Vec<String>, language: Language) -> sv_wallet::ExtendedKey {
    let wordlist = match language {
        Language::ChineseSimplified => sv_wallet::Wordlist::ChineseSimplified,
        Language::ChineseTraditional => sv_wallet::Wordlist::ChineseTraditional,
        Language::English => sv_wallet::Wordlist::English,
        Language::French => sv_wallet::Wordlist::French,
        Language::Italian => sv_wallet::Wordlist::Italian,
        Language::Japanese => sv_wallet::Wordlist::Japanese,
        Language::Korean => sv_wallet::Wordlist::Korean,
        Language::Spanish => sv_wallet::Wordlist::Spanish,
    };

    immutag_bitcoin::mnemonic_to_xpriv(mnemonic, wordlist)
}

// Doesn't do anything, yet.
fn key_validation(data: &[u8]) -> bool {
     true
}

impl FromStr for ExtendedPrivateKey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if key_validation(s.as_bytes()) {
            Ok(ExtendedPrivateKey {
                value: s.to_string()
            })
        } else {
            Err(())
        }

    }
}

impl FromStr for ExtendedPublicKey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if key_validation(s.as_bytes()) {
            Ok(ExtendedPublicKey {
                value: s.to_string()
            })
        } else {
            Err(())
        }
    }
}

impl fmt::Display for ExtendedPrivateKey {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let str = self.value.as_ref();
        fmt.write_str(str)
    }
}

impl fmt::Display for ExtendedPublicKey {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let str = self.value.as_ref();
        fmt.write_str(str)
    }
}

#[cfg(test)]
mod  wallet_integration {
    use super::{
        sv_wallet::Wordlist,
        Language,
        sv_wallet::ExtendedKey,
        immutag_bitcoin::is_private_key_valid,
        immutag_bitcoin::master_private_key,
        mnemonic_to_xpriv,
        mnemonic_to_xpub,
        ExtendedPrivateKey,
        ExtendedPublicKey,
    };

    #[test]
    fn test_mnemonic_to_xpriv() {
        //let m = ["legal".to_string(), "winner".to_string(), "thank".to_string(), "year".to_string(), "wave".to_string(), "sausage".to_string(), "worth".to_string(), "useful".to_string(), "legal".to_string(), "winner".to_string(), "thank".to_string(), "year".to_string(), "wave".to_string(), "sausage".to_string(), "worth".to_string(), "useful".to_string(), "legal".to_string(), "will".to_string()];
        //let m = ["legal".to_string(), "winner".to_string(), "thank".to_string(), "year".to_string(), "wave".to_string(), "sausage".to_string(), "worth".to_string(), "useful".to_string(), "legal".to_string(), "winner".to_string(), "thank".to_string(), "yellow".to_string()];
        let m = ["certain".to_string(), "dust".to_string(), "pave".to_string(), "crane".to_string(), "renew".to_string(), "multiply".to_string(), "stone".to_string(), "stuff".to_string(), "proud".to_string(), "flee".to_string(), "fancy".to_string(), "knee".to_string()];
        let xpriv = mnemonic_to_xpriv(m.to_vec(), Language::English);
        // Double checked by getting the xpub from the xpriv using moneybutton's bsv in nodejs.
        let expected_xpriv = "xprv9s21ZrQH143K29TJGFSiEAAQM8SMBH2V6x5Aaf9bqvXftrs1v274STWWKfz8svukBLGEQgWqkgRhpt2CNFY89CFaqdsA3gicZeqexk2itxf";

        let expected_xpriv: ExtendedPrivateKey = expected_xpriv.parse().unwrap();

        assert_eq!(xpriv, expected_xpriv);
    }

    #[test]
    fn test_mnemonic_to_xpub() {
        //let m = ["legal".to_string(), "winner".to_string(), "thank".to_string(), "year".to_string(), "wave".to_string(), "sausage".to_string(), "worth".to_string(), "useful".to_string(), "legal".to_string(), "winner".to_string(), "thank".to_string(), "year".to_string(), "wave".to_string(), "sausage".to_string(), "worth".to_string(), "useful".to_string(), "legal".to_string(), "will".to_string()];
        //let m = ["legal".to_string(), "winner".to_string(), "thank".to_string(), "year".to_string(), "wave".to_string(), "sausage".to_string(), "worth".to_string(), "useful".to_string(), "legal".to_string(), "winner".to_string(), "thank".to_string(), "yellow".to_string()];
        let m = ["certain".to_string(), "dust".to_string(), "pave".to_string(), "crane".to_string(), "renew".to_string(), "multiply".to_string(), "stone".to_string(), "stuff".to_string(), "proud".to_string(), "flee".to_string(), "fancy".to_string(), "knee".to_string()];
        let xpub = mnemonic_to_xpub(m.to_vec(), Language::English);
        // Double checked by getting the xpub from the xpriv using moneybutton's bsv in nodejs.
        let expected_xpub = "xpub661MyMwAqRbcEdXmNGyibJ78uAGqajkLUAzmP3ZDQG4emfCATZRJzFpzAxQRUsGxfvEEpTKBusBe42vEkdA1JTtevFo1f2JFDrqP5ui6syN";

        let expected_xpub: ExtendedPublicKey = expected_xpub.parse().unwrap();

        assert_eq!(xpub, expected_xpub)
    }
}

