pub mod immutag_file;
// init
pub fn immutag_file_init<T: AsRef<str>>(path: T, version: T) {
    immutag_file::init(path, version);
}

// set_filesystem(bitcoin-addr, xpriv, mnemonic)

// write(toml: String)

// get_xpriv

// get_mnemonic

// get_nickname
