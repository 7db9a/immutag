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

    fn master_private_key(seed: &str) -> ExtendedKey {
        let seed = hex::decode(seed).unwrap();
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


   #[test]
   fn mnemonic_to_xpriv() {
       let w = wallet::Wordlist::English;
       let wordlist = wallet::load_wordlist(w);
       let m = ["legal".to_string(), "winner".to_string(), "thank".to_string(), "year".to_string(), "wave".to_string(), "sausage".to_string(), "worth".to_string(), "useful".to_string(), "legal".to_string(), "winner".to_string(), "thank".to_string(), "year".to_string(), "wave".to_string(), "sausage".to_string(), "worth".to_string(), "useful".to_string(), "legal".to_string(), "will".to_string()];

       //let m = ["legal".to_string(), "winner".to_string(), "thank".to_string(), "year".to_string(), "wave".to_string(), "sausage".to_string(), "worth".to_string(), "useful".to_string(), "legal".to_string(), "winner".to_string(), "thank".to_string(), "yellow".to_string()];

       let data: &Vec<u8> = &wallet::mnemonic_decode(&m, &wordlist).unwrap();

       let data: String = hex::encode(data);

       //let seed = match str::from_utf8(data) {
       //    Ok(v) => v,
       //    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
       //};

       println!("seed: {:#?}", data);

       let key = master_private_key(data.as_str());
       let data = key.private_key().unwrap();
       println!("xpriv: {:#?}", data);

       assert_eq!(true, false);
   }
}
