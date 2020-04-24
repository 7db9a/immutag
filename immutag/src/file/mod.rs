use immutag_file;
use immutag_file::{value, Document, ErrorKind, ImmutagFileError};

/// Creates a Immutag file with basic info.
pub fn init<T: AsRef<str>>(path: T, version: T) {
    immutag_file::init(path, version);
}

pub fn open<T: AsRef<str>>(path: T) -> Result<Document, ImmutagFileError> {
    immutag_file::open(path)
}

pub fn write<T: AsRef<str>>(toml_doc: Document, path: T) -> Result<(), ImmutagFileError> {
    immutag_file::write(toml_doc, path)
}

///! Retrieve field data from a Immutag file. For example, if the file name is provided, it will attempt to retrieve the field `immutag` nested in the `README.md` entry.
///!  ```ignore
///!  [README.md]
///!  immutag = "The README."
///!  ```
///! If no file name is given, it will retrieve all the nested value in the key and not necessarily a specific field.
pub fn immutag<T: AsRef<str>>(
    doc: &Document,
    file_name: Option<T>,
    key: T,
) -> Result<String, ImmutagFileError> {
    immutag_file::immutag(doc, file_name, key)
}

pub fn entry_exists<T: AsRef<str>>(doc: &Document, key: T, key_nested: Option<T>) -> bool {
    immutag_file::entry_exists(doc, key, key_nested)
}

pub fn add_entry<T: AsRef<str>>(
    doc: &Document,
    file_name: Option<T>,
    name: T,
    immutag: T,
) -> Result<Document, ImmutagFileError> {
    immutag_file::add_entry(doc, file_name, name, immutag)
}

pub fn delete_entry<T: AsRef<str>>(
    doc: Document,
    file_name: T,
) -> Result<Document, ImmutagFileError> {
    immutag_file::delete_entry(doc, file_name)
}
