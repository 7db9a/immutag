pub mod wallet;

use wallet::{ Language, ExtendedPrivateKey, ExtendedPublicKey };

pub fn mnemonic_to_xpriv(mnemonic: Vec<String>, language: Language) -> ExtendedPrivateKey {
    wallet::mnemonic_to_xpriv(mnemonic, language)
}

pub fn mnemonic_to_xpub(mnemonic: Vec<String>, language: Language) -> ExtendedPublicKey {
    wallet::mnemonic_to_xpub(mnemonic, language)
}


#[cfg(test)]
mod  bitcoin_integration {
    use super::{
        mnemonic_to_xpriv,
        mnemonic_to_xpub,
        ExtendedPrivateKey,
        ExtendedPublicKey,
        Language,
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
