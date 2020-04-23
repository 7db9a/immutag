extern crate sv;
extern crate ring;
extern crate hex;

use sv::network::Network;
use sv::wallet;
use sv::wallet::Wordlist;
use sv::wallet::ExtendedKey;
use ring::digest::SHA512;
use ring::hmac;

pub use sv::wallet as sv_wallet;
pub use sv::network as sv_network;

/// Maximum private key value (exclusive)
const SECP256K1_CURVE_ORDER: [u8; 32] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe,
    0xba, 0xae, 0xdc, 0xe6, 0xaf, 0x48, 0xa0, 0x3b, 0xbf, 0xd2, 0x5e, 0x8c, 0xd0, 0x36, 0x41, 0x41,
];

pub fn master_private_key(seed: &Vec<u8>) -> ExtendedKey {
    //let seed = hex::decode(data).unwrap();
    let key = "Bitcoin seed".to_string();
    let key = hmac::SigningKey::new(&SHA512, &key.as_bytes());
    let hmac = hmac::sign(&key, &seed);
    ExtendedKey::new_private_key(
        Network::Mainnet,
        0,
        &[0; 4],
        0,
        &hmac.as_ref()[32..],
        &hmac.as_ref()[0..32],
    )
    .unwrap()
}

/// Checks that a private key is in valid SECP256K1 range
pub fn is_private_key_valid(key: &[u8]) -> bool {
    let mut is_below_order = false;
    if key.len() != 32 {
        return false;
    }
    for i in 0..32 {
        if key[i] < SECP256K1_CURVE_ORDER[i] {
            is_below_order = true;
            break;
        }
    }
    if !is_below_order {
        return false;
    }
    for i in 0..32 {
        if key[i] != 0 {
            return true;
        }
    }
    return false;
}

pub fn mnemonic_to_xpriv(mnemonic: Vec<String>, wordlist: Wordlist) -> ExtendedKey {
    let w = wallet::load_wordlist(wordlist);
    let data: &Vec<u8> = &wallet::mnemonic_decode(&mnemonic, &w).unwrap();
    let m = master_private_key(data);

    wallet::derive_extended_key(&m, "m").unwrap()
}

#[cfg(test)]
mod  bitcoin {
    use super::{
        sv::wallet,
        sv::wallet::Wordlist,
        wallet::ExtendedKey,
        sv::network::Network,
        ring::digest::SHA512,
        ring::hmac,
        is_private_key_valid,
        master_private_key,
        mnemonic_to_xpriv
    };

    #[test]
    fn test_mnemonic_to_xpriv() {
        //let m = ["legal".to_string(), "winner".to_string(), "thank".to_string(), "year".to_string(), "wave".to_string(), "sausage".to_string(), "worth".to_string(), "useful".to_string(), "legal".to_string(), "winner".to_string(), "thank".to_string(), "year".to_string(), "wave".to_string(), "sausage".to_string(), "worth".to_string(), "useful".to_string(), "legal".to_string(), "will".to_string()];
        //let m = ["legal".to_string(), "winner".to_string(), "thank".to_string(), "year".to_string(), "wave".to_string(), "sausage".to_string(), "worth".to_string(), "useful".to_string(), "legal".to_string(), "winner".to_string(), "thank".to_string(), "yellow".to_string()];
        let m = ["certain".to_string(), "dust".to_string(), "pave".to_string(), "crane".to_string(), "renew".to_string(), "multiply".to_string(), "stone".to_string(), "stuff".to_string(), "proud".to_string(), "flee".to_string(), "fancy".to_string(), "knee".to_string()];
        let xpriv = mnemonic_to_xpriv(m.to_vec(), Wordlist::English);
        let xpriv_string = xpriv.encode();
        // Double checked by getting the xpub from the xpriv using moneybutton's bsv in nodejs.
        let expected_xpriv = "xprv9s21ZrQH143K29TJGFSiEAAQM8SMBH2V6x5Aaf9bqvXftrs1v274STWWKfz8svukBLGEQgWqkgRhpt2CNFY89CFaqdsA3gicZeqexk2itxf";

        assert_eq!(xpriv_string, expected_xpriv);
    }

    #[test]
    fn test_mnemonic_to_xpub() {
        //let m = ["legal".to_string(), "winner".to_string(), "thank".to_string(), "year".to_string(), "wave".to_string(), "sausage".to_string(), "worth".to_string(), "useful".to_string(), "legal".to_string(), "winner".to_string(), "thank".to_string(), "year".to_string(), "wave".to_string(), "sausage".to_string(), "worth".to_string(), "useful".to_string(), "legal".to_string(), "will".to_string()];
        //let m = ["legal".to_string(), "winner".to_string(), "thank".to_string(), "year".to_string(), "wave".to_string(), "sausage".to_string(), "worth".to_string(), "useful".to_string(), "legal".to_string(), "winner".to_string(), "thank".to_string(), "yellow".to_string()];
        let m = ["certain".to_string(), "dust".to_string(), "pave".to_string(), "crane".to_string(), "renew".to_string(), "multiply".to_string(), "stone".to_string(), "stuff".to_string(), "proud".to_string(), "flee".to_string(), "fancy".to_string(), "knee".to_string()];
        let xpriv = mnemonic_to_xpriv(m.to_vec(), Wordlist::English);
        let xpub_string = xpriv.extended_public_key().unwrap().encode();
        // Double checked by getting the xpub from the xpriv using moneybutton's bsv in nodejs.
        let expected_xpub = "xpub661MyMwAqRbcEdXmNGyibJ78uAGqajkLUAzmP3ZDQG4emfCATZRJzFpzAxQRUsGxfvEEpTKBusBe42vEkdA1JTtevFo1f2JFDrqP5ui6syN";

        assert_eq!(xpub_string, expected_xpub);
    }
}
