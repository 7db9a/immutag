use immutag_file;
use immutag_file::{value, Document, ErrorKind, ImmutagFileError};

/// Creates a Immutag file with basic info.
pub fn init<T: AsRef<str>>(path: T, version: T) {
}

//pub fn open<T: AsRef<str>>(path: T) -> Result<Document, ImmutagFileError> {
//}
//
//pub fn write<T: AsRef<str>>(toml_doc: Document, path: T) -> Result<(), ImmutagFileError> {
//}
//
/////! Retrieve field data from a Immutag file. For example, if the file name is provided, it will attempt to retrieve the field `immutag` nested in the `README.md` entry.
/////!  ```ignore
/////!  [README.md]
/////!  immutag = "The README."
/////!  ```
/////! If no file name is given, it will retrieve all the nested value in the key and not necessarily a specific field.
//pub fn immutag<T: AsRef<str>>(
//    doc: &Document,
//    file_name: Option<T>,
//    key: T,
//) -> Result<String, ImmutagFileError> {
//}
//
//pub fn entry_exists<T: AsRef<str>>(doc: &Document, key: T, key_nested: Option<T>) -> bool {
//}
//
//
//pub fn add_entry<T: AsRef<str>>(
//    doc: &Document,
//    file_name: Option<T>,
//    name: T,
//    immutag: T,
//) -> Result<Document, ImmutagFileError> {
//
//pub fn delete_entry<T: AsRef<str>>(
//    doc: Document,
//    file_name: T,
//) -> Result<Document, ImmutagFileError> {
//}
