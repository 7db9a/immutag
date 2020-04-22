extern crate sv;
extern crate ring;
extern crate hex;

#[cfg(test)]
mod  bitcoin_integration {
    use super::{sv, ring, hex};
    use sv::wallet;
    use wallet::ExtendedKey;
    use sv::network::Network;
    use ring::digest::SHA512;
    use ring::hmac;
    use std::str;

    /// Maximum private key value (exclusive)
    const SECP256K1_CURVE_ORDER: [u8; 32] = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe,
        0xba, 0xae, 0xdc, 0xe6, 0xaf, 0x48, 0xa0, 0x3b, 0xbf, 0xd2, 0x5e, 0x8c, 0xd0, 0x36, 0x41, 0x41,
    ];

    fn master_private_key(seed: &Vec<u8>) -> ExtendedKey {
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
   fn is_private_key_valid(key: &[u8]) -> bool {
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

   #[test]
   fn mnemonic_to_xpriv() {
       let w = wallet::Wordlist::English;
       let wordlist = wallet::load_wordlist(w);
       //let m = ["legal".to_string(), "winner".to_string(), "thank".to_string(), "year".to_string(), "wave".to_string(), "sausage".to_string(), "worth".to_string(), "useful".to_string(), "legal".to_string(), "winner".to_string(), "thank".to_string(), "year".to_string(), "wave".to_string(), "sausage".to_string(), "worth".to_string(), "useful".to_string(), "legal".to_string(), "will".to_string()];

       //let m = ["legal".to_string(), "winner".to_string(), "thank".to_string(), "year".to_string(), "wave".to_string(), "sausage".to_string(), "worth".to_string(), "useful".to_string(), "legal".to_string(), "winner".to_string(), "thank".to_string(), "yellow".to_string()];

       let m = ["certain".to_string(), "dust".to_string(), "pave".to_string(), "crane".to_string(), "renew".to_string(), "multiply".to_string(), "stone".to_string(), "stuff".to_string(), "proud".to_string(), "flee".to_string(), "fancy".to_string(), "knee".to_string()];

       let data: &Vec<u8> = &wallet::mnemonic_decode(&m, &wordlist).unwrap();

       let key = master_private_key(data);
       let xpriv = key.private_key().unwrap();
       //println!("xpriv: {:#?}", xpriv);

       //let seed = match str::from_utf8(&data) {
       //    Ok(v) => v,
       //    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
       //};

       let valid_privkey: bool = is_private_key_valid(&xpriv);

       assert_eq!(true, valid_privkey);
   }
}
