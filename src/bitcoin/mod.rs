extern crate sv;


#[cfg(test)]
mod  bitcoin_integration {
    use super::{sv,};
    use sv::wallet;


   #[test]
   fn mnemonic_to_xpriv() {
       let w = wallet::Wordlist::English;
       let wordlist = wallet::load_wordlist(w);
       let m = ["legal".to_string(), "winner".to_string(), "thank".to_string(), "year".to_string(), "wave".to_string(), "sausage".to_string(), "worth".to_string(), "useful".to_string(), "legal".to_string(), "winner".to_string(), "thank".to_string(), "yellow".to_string()];
       wallet::mnemonic_decode(&m, &wordlist).unwrap();
       assert_eq!(true, false);
   }
}
