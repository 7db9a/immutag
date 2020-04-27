pub mod immutag_file;

use immutag_file::{ErrorKind, ImmutagFileError};

pub fn immutag_file_init<T: AsRef<str>>(path: T, version: T) {
    immutag_file::init(path, version);
}

pub fn add_filesystem<T: AsRef<str>>(
    path: T,
    bitcoin_addr: T,
    xpriv: T
) -> Result<(), ImmutagFileError> {
    immutag_file::add_filesystem(path, bitcoin_addr, xpriv)
}

pub fn get_xpriv<T: AsRef<str>>(
    path: T,
    file_addr: T
) -> Result<String, ImmutagFileError> {
    immutag_file::get_xpriv(path, file_addr)
}

pub fn get_mnemonic<T: AsRef<str>>(
    path: T,
    file_addr: T
) -> Result<String, ImmutagFileError> {
    immutag_file::get_mnemonic(path, file_addr)
}
