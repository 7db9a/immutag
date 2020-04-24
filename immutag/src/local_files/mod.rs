pub mod immutag_file;
// init
pub fn immutag_file_init<T: AsRef<str>>(path: T, version: T) {
    immutag_file::init(path, version);
}

pub fn add_filesystem<T: AsRef<str>>(
    path: T,
    bitcoin_addr: T,
    xpriv: T
) {
}

// write(toml: String)

// get_xpriv

// get_mnemonic

// get_nickname
