extern crate sv;


#[cfg(test)]
mod  bitcoin_integration {
    use super::{sv,};
    use sv::wallet;

   #[test]
   fn mnemonic_to_xpriv() {
       wallet::mnemonic_decode;
       assert_eq!(true, false);
   }
}
