/*
This module manages the specifics of the Immutag file.
*/
extern crate toml;
extern crate toml_edit;
use toml_edit::{value, Document};

pub mod err;
pub mod common;
use err::Error;
pub use err::{ErrorKind, ImmutagFileError};

use std::fs::{read_to_string, File};
use std::io::Write; // Not sure why, but file.write_all doesn't work without it. Not explicit to me.

/// Reveals the state of the Immutag file.
#[derive(Clone, Debug, PartialEq)]
pub enum ImmutagFileState {
    NonExistant,
    Valid,
    Invalid,
}

/// Creates a Immutag file with basic info.
pub fn init<T: AsRef<str>>(path: T, version: T, name: T, author: T) {
    let toml = format!(
        r#"['about']
version = "{}"
name = "{}"
author = "{}""#,
        version.as_ref(),
        name.as_ref(),
        author.as_ref()
    );

    let doc = toml.parse::<Document>().expect("invalid doc");
    write(doc, path).expect("failed to write toml to disk");
}

/// Open a Immutag file.
pub fn open<T: AsRef<str>>(path: T) -> Result<Document, ImmutagFileError> {
    let data = read_to_string(path.as_ref())?;
    let doc = data.parse::<Document>()?;

    Ok(doc)
}

/// Write to a Immutag file.
pub fn write<T: AsRef<str>>(toml_doc: Document, path: T) -> Result<(), ImmutagFileError> {
    let toml_string = toml_doc.to_string();
    let mut file = File::create(path.as_ref())?;
    file.write_all(toml_string.as_bytes())?;

    Ok(())
}

/// Valid if the version field can be read. Should rename pass
/// toml value into method, that other fields can be validated.
pub fn is_valid(doc: &Document) -> ImmutagFileState {
    let mut valid: ImmutagFileState;
    let version = entry_exists(&doc, "about", Some("version"));
    let name = entry_exists(&doc, "about", Some("name"));
    let author = entry_exists(&doc, "about", Some("author"));

    if version {
        valid = ImmutagFileState::Valid;
    } else {
        valid = ImmutagFileState::Invalid;
    }
    if !name {
        valid = ImmutagFileState::Invalid;
    }
    if !author {
        valid = ImmutagFileState::Invalid;
    }

    valid
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
    if file_name.is_some() {
        if let Some(data) = doc[file_name.unwrap().as_ref()][key.as_ref()].as_str() {
            Ok(data.to_string())
        } else {
            let err = Error::new(
                "Invalid nested entry in immutag file",
                ErrorKind::InvalidKey,
            );
            Err(ImmutagFileError::from(err))
        }
    } else if let Some(data) = doc[key.as_ref()].as_str() {
        Ok(data.to_string())
    } else {
        let err = Error::new("Invalid entry in immutag file", ErrorKind::InvalidKey);
        Err(ImmutagFileError::from(err))
    }
}

/// A crude way to find if an entry exits. Doesn't work for nested etnries.
/// `path` paramaters is the path to the Immutag file.
pub fn exists<T: AsRef<str>>(path: T, name: T) -> bool {
    let doc = open(path.as_ref()).unwrap();
    immutag(&doc, Some(name.as_ref()), "immutag").is_ok()
}

/// See if an entry exists, with an optional nested key.
/// `path` paramater is the path to the Immutag file.
pub fn entry_exists<T: AsRef<str>>(doc: &Document, key: T, key_nested: Option<T>) -> bool {
    if let Some(_key_nested) = key_nested {
        if let Some(table) = doc[key.as_ref()].as_table() {
            table.contains_key(_key_nested.as_ref())
        } else {
            false
        }
    } else {
        let table = doc.as_table();
        table.contains_key(key.as_ref())
    }
}

fn insert_entry<T: AsRef<str>>(
    doc: &Document,
    file_name: Option<T>,
    key: T,
    immutag: T,
) -> Result<Document, ImmutagFileError> {
    let status = is_valid(&doc);
    if status == ImmutagFileState::Valid {
        insert_entry_same_doc(&doc, file_name, key, immutag)
    } else if status == ImmutagFileState::NonExistant && file_name.is_some() {
        insert_entry_new_doc(&doc, file_name.unwrap(), key, immutag)
    } else {
        // Invalid
        let err = Error::new("invalid immutag file", ErrorKind::InvalidFile);
        Err(ImmutagFileError::from(err))
    }
}

fn insert_entry_new_doc<T: AsRef<str>>(
    doc: &Document,
    file_name: T,
    key: T,
    immutag: T,
) -> Result<Document, ImmutagFileError> {
    let mut toml_add: String;
    let toml = doc.to_string();
    if key.as_ref() == "immutag" {
        toml_add = format!(
            r#"
['{}']
immutag = "{}""#,
            file_name.as_ref(),
            immutag.as_ref()
        );
    } else {
        toml_add = format!("['{}']", file_name.as_ref());
    }

    let toml = toml + &toml_add;

    Ok(toml.parse::<Document>()?)
}

fn insert_entry_same_doc<T: AsRef<str>>(
    doc: &Document,
    file_name: Option<T>,
    key: T,
    immutag: T,
) -> Result<Document, ImmutagFileError> {
    if let Some(_file_name) = file_name {
        let mut doc = doc.clone();
        if !entry_exists(&doc, _file_name.as_ref(), None) {
            let toml = doc.to_string();
            if key.as_ref() == "immutag" {
                let toml_add = format!(
                    r#"
['{}']
immutag = "{}""#,
                    _file_name.as_ref(),
                    immutag.as_ref()
                );

                let toml = toml + &toml_add;
                doc = toml.parse::<Document>().expect("failed to get toml doc");

                Ok(doc)
            } else {
                let err = Error::new(
                    "no sub-keys to file/dir entries other than 'immutag' is allowed",
                    ErrorKind::InvalidKey,
                );
                Err(ImmutagFileError::from(err))
            }
        } else {
            doc[_file_name.as_ref()][key.as_ref()] = value(immutag.as_ref());

            Ok(doc)
        }
    } else {
        let mut doc = doc.clone();
        doc[key.as_ref()] = value(immutag.as_ref());

        Ok(doc)
    }
}

pub fn add_entry<T: AsRef<str>>(
    doc: &Document,
    file_name: Option<T>,
    name: T,
    immutag: T,
) -> Result<Document, ImmutagFileError> {
    let file_state = is_valid(&doc);
    if file_state == ImmutagFileState::NonExistant {
        let err = Error::new("immutag file doesn't exist", ErrorKind::NoFile);
        Err(ImmutagFileError::from(err))
    } else if file_name.is_none() {
        let entry_exists = entry_exists(&doc, "about", Some(name.as_ref()));
        if !entry_exists {
            insert_entry(&doc, None, name.as_ref(), immutag.as_ref())
        } else {
            let err = Error::new(
                "failed to add sub-entry to about field immutag file",
                ErrorKind::DuplicateKey,
            );
            Err(ImmutagFileError::from(err))
        }
    } else {
        let file_name = file_name.unwrap();
        let entry_exists = entry_exists(&doc, file_name.as_ref(), None);
        if !entry_exists {
            insert_entry(
                &doc,
                Some(file_name.as_ref()),
                name.as_ref(),
                immutag.as_ref(),
            )
        } else {
            let err = Error::new(
                "failed to add entry to immutag file",
                ErrorKind::DuplicateKey,
            );
            Err(ImmutagFileError::from(err))
        }
    }
}

pub fn update_entry<T: AsRef<str>>(
    doc: &Document,
    file_name: Option<T>,
    name: T,
    immutag: T,
) -> Result<Document, ImmutagFileError> {
    let file_state = is_valid(&doc);
    if file_state == ImmutagFileState::NonExistant {
        let err = Error::new("immutag file doesn't exist", ErrorKind::InvalidFile);
        Err(ImmutagFileError::from(err))
    } else if file_name.is_some() {
        let file_name = file_name.unwrap();
        let entry_exists = entry_exists(&doc, file_name.as_ref(), None);
        if entry_exists {
            insert_entry(
                &doc,
                Some(file_name.as_ref()),
                name.as_ref(),
                immutag.as_ref(),
            )
        } else {
            let err = Error::new(
                "immutag entry doesn't exist in immutagfile",
                ErrorKind::InvalidKey,
            );
            Err(ImmutagFileError::from(err))
        }
    } else {
        let entry_exists = entry_exists(&doc, "about", Some(name.as_ref()));
        if entry_exists {
            insert_entry(&doc, Some("about"), name.as_ref(), immutag.as_ref())
        } else {
            let err = Error::new(
                "file entry doesn't exist in immutag file",
                ErrorKind::InvalidKey,
            );
            Err(ImmutagFileError::from(err))
        }
    }
}

pub fn delete_entry<T: AsRef<str>>(
    doc: Document,
    file_name: T,
) -> Result<Document, ImmutagFileError> {
    let doc: Result<Document, ImmutagFileError> = {
        let mut _doc = doc.clone();
        let table = _doc.as_table_mut();
        table.set_implicit(true);
        let item = table.remove(file_name.as_ref());
        if let Some(mut _item) = item {
            let doc = {
                table.set_implicit(true);
                let table_string = table.to_string();
                table_string.parse::<Document>()?
            };

            Ok(doc)
        } else {
            let err = Error::new(
                "failed to delete entry in immutag file",
                ErrorKind::InvalidKey,
            );
            Err(ImmutagFileError::from(err))
        }
    };

    doc
}

#[cfg(test)]
mod toml_edit_integration {
    use super::*;

    #[test]
    fn toml_edit_insert() {
        let toml = r#"
"hello" = 'toml!' # comment
['a'.b]
        "#;
        let mut doc = toml.parse::<Document>().expect("invalid doc");
        assert_eq!(doc.to_string(), toml);
        // let's add a new key/value pair inside a.b: c = {d = "hello"}
        doc["a"]["b"]["c"]["d"] = value("hello");
        // autoformat inline table a.b.c: { d = "hello" }
        doc["a"]["b"]["c"].as_inline_table_mut().map(|t| t.fmt());

        let expected = r#"
"hello" = 'toml!' # comment
['a'.b]
c = { d = "hello" }
        "#;
        assert_eq!(doc.to_string(), expected);
    }

    #[test]
    fn toml_edit_set() {
        let toml = "";
        let mut doc = toml.parse::<Document>().expect("invalid doc");
        assert_eq!(doc.to_string(), toml);
        // let's add a new key/value
        doc["a"] = value("hello");

        let expected = r#"a = "hello"
"#;
        assert_eq!(doc.to_string(), expected);
    }

    #[test]
    fn toml_edit_set_nested() {
        let toml = r#"
['a'.b]
        "#;
        let mut doc = toml.parse::<Document>().expect("invalid doc");
        assert_eq!(doc.to_string(), toml);
        // let's add a new key/value pair inside a.b: c = {d = "hello"}
        doc["a"]["b"]["c"]["d"] = value("hello");
        // autoformat inline table a.b.c: { d = "hello" }
        doc["a"]["b"]["c"].as_inline_table_mut().map(|t| t.fmt());

        let expected = r#"
['a'.b]
c = { d = "hello" }
        "#;
        assert_eq!(doc.to_string(), expected);
    }

    #[test]
    fn toml_edit_set_file_realistic() {
        let toml = r#"
['src/lib.rs']
immutag = "The libraries entry point."
        "#;
        let mut doc = toml.parse::<Document>().expect("invalid doc");
        assert_eq!(doc.to_string(), toml);
        doc["src/lib.rs"]["version"] = value("0.0.1");
        // Commenting out won't fail test.
        doc["src/lib.rs"].as_inline_table_mut().map(|t| t.fmt());

        let expected = r#"
['src/lib.rs']
immutag = "The libraries entry point."
version = "0.0.1"
        "#;
        assert_eq!(doc.to_string(), expected);
    }

    #[test]
    fn toml_edit_get_nested_item() {
        let toml = r#"
['src/lib.rs']
immutag = "The libraries entry point."
        "#;
        let doc = toml.parse::<Document>().expect("invalid doc");
        let immutag = doc["src/lib.rs"]["immutag"].as_str();
        let expected_immutag = "The libraries entry point.";

        assert_eq!(immutag.unwrap(), expected_immutag)
    }

    #[test]
    fn toml_edit_set_get_nested_realistic() {
        let toml = r#"
['src/lib.rs']
immutag = "The libraries entry point."
        "#;
        let mut doc = toml.parse::<Document>().expect("invalid doc");
        assert_eq!(doc.to_string(), toml);
        doc["src/lib.rs"]["version"] = value("0.0.1");
        // Commenting out won't fail test.
        doc["src/lib.rs"].as_inline_table_mut().map(|t| t.fmt());

        let expected = r#"
['src/lib.rs']
immutag = "The libraries entry point."
version = "0.0.1"
        "#;

        assert_eq!(doc.to_string(), expected);
        assert_eq!(
            doc["src/lib.rs"]["immutag"].as_str().unwrap(),
            "The libraries entry point."
        );
        assert_eq!(doc["src/lib.rs"]["version"].as_str().unwrap(), "0.0.1")
    }

    #[test]
    fn toml_append() {
        let immutag_fields = r#"['about']
version = "0.1.0"
name = "NAME"
author = "AUTHOR""#;

        let toml = immutag_fields
            .parse::<Document>()
            .expect("invalid doc");
        let toml_string = toml.to_string();

        let immutag_fields = r#"
['src/lib.rs']
immutag = "The libraries entry point."
version = "0.0.1""#;

        let expected = r#"['about']
version = "0.1.0"
name = "NAME"
author = "AUTHOR"

['src/lib.rs']
immutag = "The libraries entry point."
version = "0.0.1"
"#;

        let new_toml_string = toml_string + immutag_fields;
        let new_toml = new_toml_string.parse::<Document>().expect("invalid doc");

        assert_eq!(new_toml.to_string(), expected);
    }
}

#[cfg(test)]
mod integration {
    use super::*;
    use common::Fixture;

    pub fn setup_test<T: AsRef<str>>(
        path: T,
        version: T,
        name: T,
        author: T,
    ) -> Fixture {
        let fixture = Fixture::new()
            .add_dirpath(path.as_ref().to_string())
            .build();

        init(
            path.as_ref().to_string() + "/Immutag",
            version.as_ref().to_string(),
            name.as_ref().to_string(),
            author.as_ref().to_string(),
        );

        fixture
    }

    pub fn setup_add<T: AsRef<str>>(
        immutag_path: T,
    ) -> (Document, Result<String, ImmutagFileError>) {
        let doc = open(immutag_path.as_ref()).unwrap();
        let doc = add_entry(
            &doc,
            Some("src/lib.rs"),
            "immutag",
            "Entry point to the library.",
        )
        .unwrap();
        write(doc.clone(), immutag_path.as_ref()).expect("failed to write toml to disk");
        let immutag_res = immutag(&doc, Some("src/lib.rs"), "immutag");

        (doc, immutag_res)
    }

    #[test]
    fn immutagfile_init() {
        let path = ".immutag/.immutag_tests";
        let gpath = ".immutag/.immutag_tests/Immutag";
        let mut fixture = setup_test(path, "0.1.0", "NAME", "AUTHOR");
        let doc = open(gpath).unwrap();
        let is_valid = is_valid(&doc);
        let doc = open(gpath).unwrap();
        let expected = r#"['about']
version = "0.1.0"
name = "NAME"
author = "AUTHOR"
"#;
        fixture.teardown(true);
        assert_eq!(is_valid, ImmutagFileState::Valid);
        assert_eq!(doc.to_string(), expected);
    }

    #[test]
    fn immutagfile_add_entry() {
        let path = ".immutag/.immutag_tests";
        let gpath = ".immutag/.immutag_tests/Immutag";
        let mut fixture = setup_test(path, "0.1.0", "NAME", "AUTHOR");
        let doc = open(gpath).unwrap();
        let doc = add_entry(
            &doc,
            Some("src/lib.rs"),
            "immutag",
            "Entry point to the library.",
        )
        .unwrap();
        write(doc.clone(), gpath).expect("failed to write toml to disk");
        let immutag_res = immutag(&doc, Some("src/lib.rs"), "immutag").unwrap();
        fixture.teardown(true);
        assert_eq!(immutag_res, "Entry point to the library.");
    }

    #[test]
    fn immutagfile_add_dir_entry() {
        let path = ".immutag/.immutag_tests";
        let gpath = ".immutag/.immutag_tests/Immutag";
        let mut fixture = setup_test(path, "0.1.0", "NAME", "AUTHOR");
        let doc = open(gpath).unwrap();
        let doc = add_entry(&doc, Some("src/"), "immutag", "The source code.").unwrap();
        write(doc.clone(), gpath).expect("failed to write toml to disk");
        let immutag_res = immutag(&doc, Some("src/"), "immutag").unwrap();
        fixture.teardown(true);
        assert_eq!(immutag_res, "The source code.");
    }

    #[test]
    fn immutagfile_error_update_entry() {
        let path = ".immutag/.immutag_tests";
        let gpath = ".immutag/.immutag_tests/Immutag";
        let mut fixture = setup_test(path, "0.1.0", "NAME", "AUTHOR");
        let doc = open(gpath).unwrap();
        let result = update_entry(
            &doc,
            Some("src/lib.rs"),
            "immutag",
            "Entry point to the library.",
        );

        fixture.teardown(true);

        assert!(result.is_err());
    }

    // Verifies there is no unexpected whitespace or formatting issuees for a basic case.
    #[test]
    fn format_immutagfile_file_add_entry() {
        let path = ".immutag/.immutag_tests";
        let gpath = ".immutag/.immutag_tests/Immutag";
        let mut fixture = setup_test(path, "0.1.0", "NAME", "AUTHOR");
        let (_, _) = setup_add(gpath);

        // Focus of test.
        let toml_string = read_to_string(gpath).expect("failed to read immutagfile");

        let doc = open(gpath).unwrap();

        //let mut doc = toml_string.parse::<Document>().expect("failed to get toml doc");
        //doc["src/lib.rs"].as_inline_table_mut().map(|t| t.fmt());
        let expected = r#"['about']
version = "0.1.0"
name = "NAME"
author = "AUTHOR"

['src/lib.rs']
immutag = "Entry point to the library."
"#;

        fixture.teardown(true);

        assert_eq!(doc.to_string(), expected);
        assert_eq!(toml_string, expected);
    }

    #[test]
    fn immutagfile_entry_exists() {
        let path = ".immutag/.immutag_tests";
        let gpath = ".immutag/.immutag_tests/Immutag";
        let mut fixture = setup_test(path, "0.1.0", "NAME", "AUTHOR");
        let (doc, _) = setup_add(gpath);

        assert_eq!(entry_exists(&doc, "src/lib.rs", None), true);

        assert_eq!(exists(gpath, "src/lib.rs"), true);

        assert_eq!(entry_exists(&doc, "NOT_REAL.md", None), false);

        assert_eq!(exists(gpath, "NOT_REAL.md"), false);

        fixture.teardown(true);
    }

    #[test]
    fn immutagfile_update_entry() {
        let path = ".immutag/.immutag_tests";
        let gpath = ".immutag/.immutag_tests/Immutag";
        let mut fixture = setup_test(path, "0.1.0", "NAME", "AUTHOR");
        let (doc, immutag_res) = setup_add(gpath);
        // Focus of test.
        let doc = update_entry(
            &doc,
            Some("src/lib.rs"),
            "immutag",
            "Like main.rs, but for libraries.",
        )
        .unwrap();
        write(doc.clone(), gpath).expect("failed to write toml to disk");
        let updated_immutag_res = immutag(&doc, Some("src/lib.rs"), "immutag").unwrap();

        fixture.teardown(true);

        assert_eq!(immutag_res.unwrap(), "Entry point to the library.");
        assert_eq!(updated_immutag_res, "Like main.rs, but for libraries.");
    }

    #[test]
    fn immutagfile_error_add_entry() {
        let path = ".immutag/.immutag_tests";
        let gpath = ".immutag/.immutag_tests/Immutag";
        let mut fixture = setup_test(path, "0.1.0", "NAME", "AUTHOR");
        let (doc, immutag) = setup_add(gpath);

        // Focus of test.
        let result = add_entry(
            &doc,
            Some("src/lib.rs"),
            "immutag",
            "Like main.rs, but for libraries.",
        );

        fixture.teardown(true);

        assert_eq!(immutag.unwrap(), "Entry point to the library.");
        assert!(result.is_err());
    }

    fn helper_immutagfile_delete_entry_thorough_check<T: AsRef<str>>(path_to_dir: T) {
        let path = path_to_dir;
        let gpath = path.as_ref().to_string() + "/Immutag";
        let _fixture = setup_test(path.as_ref(), "0.1.0", "NAME", "AUTHOR");

        let (doc, _) = setup_add(gpath.as_str());

        let lib_exists = entry_exists(&doc, "src/lib.rs", None);

        let doc = add_entry(&doc, Some("README.md"), "immutag", "The README.").unwrap();

        write(doc.clone(), gpath.as_str()).expect("failed to write toml to disk");

        let new_doc = delete_entry(doc, "README.md").unwrap();
        write(new_doc.clone(), gpath).expect("failed to write toml to disk");

        let expected = r#"['about']
version = "0.1.0"
name = "NAME"
author = "AUTHOR"

['src/lib.rs']
immutag = "Entry point to the library."
"#;

        assert_eq!(lib_exists, true);
        assert_eq!(new_doc.to_string(), expected)
    }

    #[test]
    fn immutagfile_delete_entry_thorough_assert() {
        let path = ".immutag/.immutag_tests";
        helper_immutagfile_delete_entry_thorough_check(path);

        Fixture::new().add_dirpath(path.to_string()).teardown(true);
    }

    #[test]
    fn immutagfile_complicated() {
        let path = ".immutag/.immutag_tests";
        helper_immutagfile_delete_entry_thorough_check(path);

        let doc = open(path.to_string() + "/Immutag").unwrap();

        let modified_doc = {
            let update_version =
                || -> Document { update_entry(&doc, None, "version", "0.2.0").unwrap() };

            let update_author = || -> Document {
                update_entry(&update_version(), None, "author", "CHANGED_AUTHOR").unwrap()
            };

            let add_main = || -> Document {
                add_entry(
                    &update_author(),
                    Some("src/main.rs"),
                    "immutag",
                    "Like lib.rs, but for apps.",
                )
                .unwrap()
            };

            add_entry(
                &add_main(),
                Some(".gitignore"),
                "immutag",
                "Tells git which files to ignore.",
            )
            .unwrap()
        };

        write(modified_doc, path.to_string() + "/Immutag").expect("failed to write toml to disk");

        let expected = r#"['about']
version = "0.2.0"
name = "NAME"
author = "CHANGED_AUTHOR"

['src/lib.rs']
immutag = "Entry point to the library."

['src/main.rs']
immutag = "Like lib.rs, but for apps."

['.gitignore']
immutag = "Tells git which files to ignore."
"#;

        let doc = open(path.to_string() + "/Immutag").unwrap();

        Fixture::new().add_dirpath(path.to_string()).teardown(true);

        assert_eq!(doc.to_string(), expected)
    }

    #[test]
    fn immutagfile_delete_file_entry() {
        let path = ".immutag/.immutag_tests";
        let gpath = ".immutag/.immutag_tests/Immutag";
        let mut fixture = setup_test(path, "0.1.0", "NAME", "AUTHOR");
        let doc = open(gpath).unwrap();
        let doc = add_entry(
            &doc,
            Some("src/lib.rs"),
            "immutag",
            "Entry point to the library.",
        )
        .unwrap();
        write(doc.clone(), gpath).expect("failed to write toml to disk");
        let immutag_res = immutag(&doc, Some("src/lib.rs"), "immutag").unwrap();

        assert_eq!(immutag_res, "Entry point to the library.");

        // Focus of test.
        let doc = open(gpath).unwrap();
        let doc = delete_entry(doc.clone(), "src/lib.rs").expect("failed to delete entry");
        write(doc, gpath).expect("failed to write toml to disk");

        let result = {
            let doc = open(".immutag/.immutag_tests/Immutag").unwrap();
            immutag(&doc, Some("src/lib.rs"), "immutag")
        };

        assert_eq!(result.is_ok(), false);

        fixture.teardown(true);
    }

    #[test]
    fn immutagfile_delete_dir_entry() {
        let path = ".immutag/.immutag_tests";
        let gpath = ".immutag/.immutag_tests/Immutag";
        let mut fixture = setup_test(path, "0.1.0", "NAME", "AUTHOR");
        let doc = open(gpath).unwrap();
        let doc = add_entry(&doc, Some("src/"), "immutag", "The source code.").unwrap();
        write(doc.clone(), gpath).expect("failed to write toml to disk");
        let immutag_res = immutag(&doc, Some("src/"), "immutag").unwrap();

        assert_eq!(immutag_res, "The source code.");

        // Focus of test.
        let doc = open(gpath).unwrap();
        let doc = delete_entry(doc.clone(), "src/").expect("failed to delete entry");
        write(doc, gpath).expect("failed to write toml to disk");

        let result = {
            let doc = open(".immutag/.immutag_tests/Immutag").unwrap();
            immutag(&doc, Some("src/"), "immutag")
        };

        assert_eq!(result.is_ok(), false);

        fixture.teardown(true);
    }

    #[test]
    fn error_immutagfile_delete_dir_entry() {
        let path = ".immutag/.immutag_tests";
        let gpath = ".immutag/.immutag_tests/Immutag";
        let mut fixture = setup_test(path, "0.1.0", "NAME", "AUTHOR");
        let doc = open(gpath).unwrap();
        let doc = add_entry(&doc, Some("src/"), "immutag", "The source code.").unwrap();
        write(doc.clone(), gpath).expect("failed to write toml to disk");
        let immutag_res = immutag(&doc, Some("src/"), "immutag").unwrap();

        assert_eq!(immutag_res, "The source code.");

        // Focus of test.
        let doc = open(gpath).unwrap();
        let doc = delete_entry(doc.clone(), "src"); // name must be exact, "src/"

        match doc {
            Err(err) => match err {
                ImmutagFileError::Error(e) => assert_eq!(ErrorKind::InvalidKey, e.kind),
                _ => panic!("wrong error type"),
            },
            _ => panic!("expected an error"),
        }

        fixture.teardown(true);
    }
}
