use immutag_bitcoin;
use immutag_file;


#[cfg(test)]
mod  bitcoin_integration {
    use super::{
        immutag_bitcoin::Wordlist,
        immutag_bitcoin::ExtendedKey,
        immutag_bitcoin::is_private_key_valid,
        immutag_bitcoin::master_private_key,
        immutag_bitcoin::mnemonic_to_xpriv
    };

    #[test]
    fn test_mnemonic_to_xpriv() {
        //let m = ["legal".to_string(), "winner".to_string(), "thank".to_string(), "year".to_string(), "wave".to_string(), "sausage".to_string(), "worth".to_string(), "useful".to_string(), "legal".to_string(), "winner".to_string(), "thank".to_string(), "year".to_string(), "wave".to_string(), "sausage".to_string(), "worth".to_string(), "useful".to_string(), "legal".to_string(), "will".to_string()];
        //let m = ["legal".to_string(), "winner".to_string(), "thank".to_string(), "year".to_string(), "wave".to_string(), "sausage".to_string(), "worth".to_string(), "useful".to_string(), "legal".to_string(), "winner".to_string(), "thank".to_string(), "yellow".to_string()];
        let m = ["certain".to_string(), "dust".to_string(), "pave".to_string(), "crane".to_string(), "renew".to_string(), "multiply".to_string(), "stone".to_string(), "stuff".to_string(), "proud".to_string(), "flee".to_string(), "fancy".to_string(), "knee".to_string()];
        let xpriv = mnemonic_to_xpriv(m.to_vec(), Wordlist::English);
        let xpriv_string = xpriv.encode();
        let xpub_string = xpriv.extended_public_key().unwrap().encode();
        // Double checked by getting the xpub from the xpriv using moneybutton's bsv in nodejs.
        let expected_xpriv = "xprv9s21ZrQH143K29TJGFSiEAAQM8SMBH2V6x5Aaf9bqvXftrs1v274STWWKfz8svukBLGEQgWqkgRhpt2CNFY89CFaqdsA3gicZeqexk2itxf";
        let expected_xpub = "xpub661MyMwAqRbcEdXmNGyibJ78uAGqajkLUAzmP3ZDQG4emfCATZRJzFpzAxQRUsGxfvEEpTKBusBe42vEkdA1JTtevFo1f2JFDrqP5ui6syN";

        assert_eq!(xpriv_string, expected_xpriv);
        assert_eq!(xpub_string, expected_xpub);
    }
}

