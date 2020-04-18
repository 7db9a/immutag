/*
This module manages the specifics of the Metadata file.
*/
extern crate toml;
extern crate toml_edit;
use toml_edit::{value, Document};

pub mod err;
use err::Error;
pub use err::{ErrorKind, MetadataFileError};

use std::fs::{read_to_string, File};
use std::io::Write; // Not sure why, but file.write_all doesn't work without it. Not explicit to me.

/// Reveals the state of the Metadata file.
#[derive(Clone, Debug, PartialEq)]
pub enum MetadataFileState {
    NonExistant,
    Valid,
    Invalid,
}

/// Creates a Metadata file with basic info.
pub fn init<T: AsRef<str>>(path: T, version: T, name: T, author: T, git_metadata_version: T) {
    let toml = format!(
        r#"['about']
version = "{}"
name = "{}"
author = "{}"
git-metadata-version = "{}""#,
        version.as_ref(),
        name.as_ref(),
        author.as_ref(),
        git_metadata_version.as_ref()
    );

    let doc = toml.parse::<Document>().expect("invalid doc");
    write(doc, path).expect("failed to write toml to disk");
}

/// Open a Metadata file.
pub fn open<T: AsRef<str>>(path: T) -> Result<Document, MetadataFileError> {
    let data = read_to_string(path.as_ref())?;
    let doc = data.parse::<Document>()?;

    Ok(doc)
}

/// Write to a Metadata file.
pub fn write<T: AsRef<str>>(toml_doc: Document, path: T) -> Result<(), MetadataFileError> {
    let toml_string = toml_doc.to_string();
    let mut file = File::create(path.as_ref())?;
    file.write_all(toml_string.as_bytes())?;

    Ok(())
}

/// Valid if the version field can be read. Should rename pass
/// toml value into method, that other fields can be validated.
pub fn is_valid(doc: &Document) -> MetadataFileState {
    let mut valid: MetadataFileState;
    let version = entry_exists(&doc, "about", Some("version"));
    let name = entry_exists(&doc, "about", Some("name"));
    let author = entry_exists(&doc, "about", Some("author"));
    let g_version = entry_exists(&doc, "about", Some("git-metadata-version"));

    if version {
        valid = MetadataFileState::Valid;
    } else {
        valid = MetadataFileState::Invalid;
    }
    if !name {
        valid = MetadataFileState::Invalid;
    }
    if !author {
        valid = MetadataFileState::Invalid;
    }
    if !g_version {
        valid = MetadataFileState::Invalid;
    }

    valid
}

///! Retrieve field data from a Metadata file. For example, if the file name is provided, it will attempt to retrieve the field `metadata` nested in the `README.md` entry.
///!  ```ignore
///!  [README.md]
///!  metadata = "The README."
///!  ```
///! If no file name is given, it will retrieve all the nested value in the key and not necessarily a specific field.
pub fn metadata<T: AsRef<str>>(
    doc: &Document,
    file_name: Option<T>,
    key: T,
) -> Result<String, MetadataFileError> {
    if file_name.is_some() {
        if let Some(data) = doc[file_name.unwrap().as_ref()][key.as_ref()].as_str() {
            Ok(data.to_string())
        } else {
            let err = Error::new(
                "Invalid nested entry in metadata file",
                ErrorKind::InvalidKey,
            );
            Err(MetadataFileError::from(err))
        }
    } else if let Some(data) = doc[key.as_ref()].as_str() {
        Ok(data.to_string())
    } else {
        let err = Error::new("Invalid entry in metadata file", ErrorKind::InvalidKey);
        Err(MetadataFileError::from(err))
    }
}

/// A crude way to find if an entry exits. Doesn't work for nested etnries.
/// `path` paramaters is the path to the Metadata file.
pub fn exists<T: AsRef<str>>(path: T, name: T) -> bool {
    let doc = open(path.as_ref()).unwrap();
    metadata(&doc, Some(name.as_ref()), "metadata").is_ok()
}

/// See if an entry exists, with an optional nested key.
/// `path` paramater is the path to the Metadata file.
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
    metadata: T,
) -> Result<Document, MetadataFileError> {
    let status = is_valid(&doc);
    if status == MetadataFileState::Valid {
        insert_entry_same_doc(&doc, file_name, key, metadata)
    } else if status == MetadataFileState::NonExistant && file_name.is_some() {
        insert_entry_new_doc(&doc, file_name.unwrap(), key, metadata)
    } else {
        // Invalid
        let err = Error::new("invalid metadata file", ErrorKind::InvalidFile);
        Err(MetadataFileError::from(err))
    }
}

fn insert_entry_new_doc<T: AsRef<str>>(
    doc: &Document,
    file_name: T,
    key: T,
    metadata: T,
) -> Result<Document, MetadataFileError> {
    let mut toml_add: String;
    let toml = doc.to_string();
    if key.as_ref() == "metadata" {
        toml_add = format!(
            r#"
['{}']
metadata = "{}""#,
            file_name.as_ref(),
            metadata.as_ref()
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
    metadata: T,
) -> Result<Document, MetadataFileError> {
    if let Some(_file_name) = file_name {
        let mut doc = doc.clone();
        if !entry_exists(&doc, _file_name.as_ref(), None) {
            let toml = doc.to_string();
            if key.as_ref() == "metadata" {
                let toml_add = format!(
                    r#"
['{}']
metadata = "{}""#,
                    _file_name.as_ref(),
                    metadata.as_ref()
                );

                let toml = toml + &toml_add;
                doc = toml.parse::<Document>().expect("failed to get toml doc");

                Ok(doc)
            } else {
                let err = Error::new(
                    "no sub-keys to file/dir entries other than 'metadata' is allowed",
                    ErrorKind::InvalidKey,
                );
                Err(MetadataFileError::from(err))
            }
        } else {
            doc[_file_name.as_ref()][key.as_ref()] = value(metadata.as_ref());

            Ok(doc)
        }
    } else {
        let mut doc = doc.clone();
        doc[key.as_ref()] = value(metadata.as_ref());

        Ok(doc)
    }
}

pub fn add_entry<T: AsRef<str>>(
    doc: &Document,
    file_name: Option<T>,
    name: T,
    metadata: T,
) -> Result<Document, MetadataFileError> {
    let file_state = is_valid(&doc);
    if file_state == MetadataFileState::NonExistant {
        let err = Error::new("metadata file doesn't exist", ErrorKind::NoFile);
        Err(MetadataFileError::from(err))
    } else if file_name.is_none() {
        let entry_exists = entry_exists(&doc, "about", Some(name.as_ref()));
        if !entry_exists {
            insert_entry(&doc, None, name.as_ref(), metadata.as_ref())
        } else {
            let err = Error::new(
                "failed to add sub-entry to about field metadata file",
                ErrorKind::DuplicateKey,
            );
            Err(MetadataFileError::from(err))
        }
    } else {
        let file_name = file_name.unwrap();
        let entry_exists = entry_exists(&doc, file_name.as_ref(), None);
        if !entry_exists {
            insert_entry(
                &doc,
                Some(file_name.as_ref()),
                name.as_ref(),
                metadata.as_ref(),
            )
        } else {
            let err = Error::new(
                "failed to add entry to metadata file",
                ErrorKind::DuplicateKey,
            );
            Err(MetadataFileError::from(err))
        }
    }
}

pub fn update_entry<T: AsRef<str>>(
    doc: &Document,
    file_name: Option<T>,
    name: T,
    metadata: T,
) -> Result<Document, MetadataFileError> {
    let file_state = is_valid(&doc);
    if file_state == MetadataFileState::NonExistant {
        let err = Error::new("metadata file doesn't exist", ErrorKind::InvalidFile);
        Err(MetadataFileError::from(err))
    } else if file_name.is_some() {
        let file_name = file_name.unwrap();
        let entry_exists = entry_exists(&doc, file_name.as_ref(), None);
        if entry_exists {
            insert_entry(
                &doc,
                Some(file_name.as_ref()),
                name.as_ref(),
                metadata.as_ref(),
            )
        } else {
            let err = Error::new(
                "git-metadata entry doesn't exist in metadatafile",
                ErrorKind::InvalidKey,
            );
            Err(MetadataFileError::from(err))
        }
    } else {
        let entry_exists = entry_exists(&doc, "about", Some(name.as_ref()));
        if entry_exists {
            insert_entry(&doc, Some("about"), name.as_ref(), metadata.as_ref())
        } else {
            let err = Error::new(
                "file entry doesn't exist in metadata file",
                ErrorKind::InvalidKey,
            );
            Err(MetadataFileError::from(err))
        }
    }
}

pub fn delete_entry<T: AsRef<str>>(
    doc: Document,
    file_name: T,
) -> Result<Document, MetadataFileError> {
    let doc: Result<Document, MetadataFileError> = {
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
                "failed to delete entry in metadata file",
                ErrorKind::InvalidKey,
            );
            Err(MetadataFileError::from(err))
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
metadata = "The libraries entry point."
        "#;
        let mut doc = toml.parse::<Document>().expect("invalid doc");
        assert_eq!(doc.to_string(), toml);
        doc["src/lib.rs"]["version"] = value("0.0.1");
        // Commenting out won't fail test.
        doc["src/lib.rs"].as_inline_table_mut().map(|t| t.fmt());

        let expected = r#"
['src/lib.rs']
metadata = "The libraries entry point."
version = "0.0.1"
        "#;
        assert_eq!(doc.to_string(), expected);
    }

    #[test]
    fn toml_edit_get_nested_item() {
        let toml = r#"
['src/lib.rs']
metadata = "The libraries entry point."
        "#;
        let doc = toml.parse::<Document>().expect("invalid doc");
        let metadata = doc["src/lib.rs"]["metadata"].as_str();
        let expected_metadata = "The libraries entry point.";

        assert_eq!(metadata.unwrap(), expected_metadata)
    }

    #[test]
    fn toml_edit_set_get_nested_realistic() {
        let toml = r#"
['src/lib.rs']
metadata = "The libraries entry point."
        "#;
        let mut doc = toml.parse::<Document>().expect("invalid doc");
        assert_eq!(doc.to_string(), toml);
        doc["src/lib.rs"]["version"] = value("0.0.1");
        // Commenting out won't fail test.
        doc["src/lib.rs"].as_inline_table_mut().map(|t| t.fmt());

        let expected = r#"
['src/lib.rs']
metadata = "The libraries entry point."
version = "0.0.1"
        "#;

        assert_eq!(doc.to_string(), expected);
        assert_eq!(
            doc["src/lib.rs"]["metadata"].as_str().unwrap(),
            "The libraries entry point."
        );
        assert_eq!(doc["src/lib.rs"]["version"].as_str().unwrap(), "0.0.1")
    }

    #[test]
    fn toml_append() {
        let git_metadata_fields = r#"['about']
version = "0.1.0"
name = "NAME"
author = "AUTHOR"
git-metadata-version = "0.1.0""#;

        let toml = git_metadata_fields
            .parse::<Document>()
            .expect("invalid doc");
        let toml_string = toml.to_string();

        let metadata_fields = r#"
['src/lib.rs']
metadata = "The libraries entry point."
version = "0.0.1""#;

        let expected = r#"['about']
version = "0.1.0"
name = "NAME"
author = "AUTHOR"
git-metadata-version = "0.1.0"

['src/lib.rs']
metadata = "The libraries entry point."
version = "0.0.1"
"#;

        let new_toml_string = toml_string + metadata_fields;
        let new_toml = new_toml_string.parse::<Document>().expect("invalid doc");

        assert_eq!(new_toml.to_string(), expected);
    }
}

#[cfg(test)]
mod integration {
    use super::*;
    use crate::common::Fixture;

    pub fn setup_test<T: AsRef<str>>(
        path: T,
        version: T,
        name: T,
        author: T,
        git_metadata_version: T,
    ) -> Fixture {
        let fixture = Fixture::new()
            .add_dirpath(path.as_ref().to_string())
            .build();

        init(
            path.as_ref().to_string() + "/Metadata",
            version.as_ref().to_string(),
            name.as_ref().to_string(),
            author.as_ref().to_string(),
            git_metadata_version.as_ref().to_string(),
        );

        fixture
    }

    pub fn setup_add<T: AsRef<str>>(
        metadata_path: T,
    ) -> (Document, Result<String, MetadataFileError>) {
        let doc = open(metadata_path.as_ref()).unwrap();
        let doc = add_entry(
            &doc,
            Some("src/lib.rs"),
            "metadata",
            "Entry point to the library.",
        )
        .unwrap();
        write(doc.clone(), metadata_path.as_ref()).expect("failed to write toml to disk");
        let metadata_res = metadata(&doc, Some("src/lib.rs"), "metadata");

        (doc, metadata_res)
    }

    #[test]
    fn metadatafile_init() {
        let path = ".metadata/.metadata_tests";
        let gpath = ".metadata/.metadata_tests/Metadata";
        let mut fixture = setup_test(path, "0.1.0", "NAME", "AUTHOR", "0.1.0");
        let doc = open(gpath).unwrap();
        let is_valid = is_valid(&doc);
        let doc = open(gpath).unwrap();
        let expected = r#"['about']
version = "0.1.0"
name = "NAME"
author = "AUTHOR"
git-metadata-version = "0.1.0"
"#;
        fixture.teardown(true);
        assert_eq!(is_valid, MetadataFileState::Valid);
        assert_eq!(doc.to_string(), expected);
    }

    #[test]
    fn metadatafile_add_entry() {
        let path = ".metadata/.metadata_tests";
        let gpath = ".metadata/.metadata_tests/Metadata";
        let mut fixture = setup_test(path, "0.1.0", "NAME", "AUTHOR", "0.1.0");
        let doc = open(gpath).unwrap();
        let doc = add_entry(
            &doc,
            Some("src/lib.rs"),
            "metadata",
            "Entry point to the library.",
        )
        .unwrap();
        write(doc.clone(), gpath).expect("failed to write toml to disk");
        let metadata_res = metadata(&doc, Some("src/lib.rs"), "metadata").unwrap();
        fixture.teardown(true);
        assert_eq!(metadata_res, "Entry point to the library.");
    }

    #[test]
    fn metadatafile_add_dir_entry() {
        let path = ".metadata/.metadata_tests";
        let gpath = ".metadata/.metadata_tests/Metadata";
        let mut fixture = setup_test(path, "0.1.0", "NAME", "AUTHOR", "0.1.0");
        let doc = open(gpath).unwrap();
        let doc = add_entry(&doc, Some("src/"), "metadata", "The source code.").unwrap();
        write(doc.clone(), gpath).expect("failed to write toml to disk");
        let metadata_res = metadata(&doc, Some("src/"), "metadata").unwrap();
        fixture.teardown(true);
        assert_eq!(metadata_res, "The source code.");
    }

    #[test]
    fn metadatafile_error_update_entry() {
        let path = ".metadata/.metadata_tests";
        let gpath = ".metadata/.metadata_tests/Metadata";
        let mut fixture = setup_test(path, "0.1.0", "NAME", "AUTHOR", "0.1.0");
        let doc = open(gpath).unwrap();
        let result = update_entry(
            &doc,
            Some("src/lib.rs"),
            "metadata",
            "Entry point to the library.",
        );

        fixture.teardown(true);

        assert!(result.is_err());
    }

    // Verifies there is no unexpected whitespace or formatting issuees for a basic case.
    #[test]
    fn format_metadatafile_file_add_entry() {
        let path = ".metadata/.metadata_tests";
        let gpath = ".metadata/.metadata_tests/Metadata";
        let mut fixture = setup_test(path, "0.1.0", "NAME", "AUTHOR", "0.1.0");
        let (_, _) = setup_add(gpath);

        // Focus of test.
        let toml_string = read_to_string(gpath).expect("failed to read metadatafile");

        let doc = open(gpath).unwrap();

        //let mut doc = toml_string.parse::<Document>().expect("failed to get toml doc");
        //doc["src/lib.rs"].as_inline_table_mut().map(|t| t.fmt());
        let expected = r#"['about']
version = "0.1.0"
name = "NAME"
author = "AUTHOR"
git-metadata-version = "0.1.0"

['src/lib.rs']
metadata = "Entry point to the library."
"#;

        fixture.teardown(true);

        assert_eq!(doc.to_string(), expected);
        assert_eq!(toml_string, expected);
    }

    #[test]
    fn metadatafile_entry_exists() {
        let path = ".metadata/.metadata_tests";
        let gpath = ".metadata/.metadata_tests/Metadata";
        let mut fixture = setup_test(path, "0.1.0", "NAME", "AUTHOR", "0.1.0");
        let (doc, _) = setup_add(gpath);

        assert_eq!(entry_exists(&doc, "src/lib.rs", None), true);

        assert_eq!(exists(gpath, "src/lib.rs"), true);

        assert_eq!(entry_exists(&doc, "NOT_REAL.md", None), false);

        assert_eq!(exists(gpath, "NOT_REAL.md"), false);

        fixture.teardown(true);
    }

    #[test]
    fn metadatafile_update_entry() {
        let path = ".metadata/.metadata_tests";
        let gpath = ".metadata/.metadata_tests/Metadata";
        let mut fixture = setup_test(path, "0.1.0", "NAME", "AUTHOR", "0.1.0");
        let (doc, metadata_res) = setup_add(gpath);
        // Focus of test.
        let doc = update_entry(
            &doc,
            Some("src/lib.rs"),
            "metadata",
            "Like main.rs, but for libraries.",
        )
        .unwrap();
        write(doc.clone(), gpath).expect("failed to write toml to disk");
        let updated_metadata_res = metadata(&doc, Some("src/lib.rs"), "metadata").unwrap();

        fixture.teardown(true);

        assert_eq!(metadata_res.unwrap(), "Entry point to the library.");
        assert_eq!(updated_metadata_res, "Like main.rs, but for libraries.");
    }

    #[test]
    fn metadatafile_error_add_entry() {
        let path = ".metadata/.metadata_tests";
        let gpath = ".metadata/.metadata_tests/Metadata";
        let mut fixture = setup_test(path, "0.1.0", "NAME", "AUTHOR", "0.1.0");
        let (doc, metadata) = setup_add(gpath);

        // Focus of test.
        let result = add_entry(
            &doc,
            Some("src/lib.rs"),
            "metadata",
            "Like main.rs, but for libraries.",
        );

        fixture.teardown(true);

        assert_eq!(metadata.unwrap(), "Entry point to the library.");
        assert!(result.is_err());
    }

    fn helper_metadatafile_delete_entry_thorough_check<T: AsRef<str>>(path_to_dir: T) {
        let path = path_to_dir;
        let gpath = path.as_ref().to_string() + "/Metadata";
        let _fixture = setup_test(path.as_ref(), "0.1.0", "NAME", "AUTHOR", "0.1.0");

        let (doc, _) = setup_add(gpath.as_str());

        let lib_exists = entry_exists(&doc, "src/lib.rs", None);

        let doc = add_entry(&doc, Some("README.md"), "metadata", "The README.").unwrap();

        write(doc.clone(), gpath.as_str()).expect("failed to write toml to disk");

        let new_doc = delete_entry(doc, "README.md").unwrap();
        write(new_doc.clone(), gpath).expect("failed to write toml to disk");

        let expected = r#"['about']
version = "0.1.0"
name = "NAME"
author = "AUTHOR"
git-metadata-version = "0.1.0"

['src/lib.rs']
metadata = "Entry point to the library."
"#;

        assert_eq!(lib_exists, true);
        assert_eq!(new_doc.to_string(), expected)
    }

    #[test]
    fn metadatafile_delete_entry_thorough_assert() {
        let path = ".metadata/.metadata_tests";
        helper_metadatafile_delete_entry_thorough_check(path);

        Fixture::new().add_dirpath(path.to_string()).teardown(true);
    }

    #[test]
    fn metadatafile_complicated() {
        let path = ".metadata/.metadata_tests";
        helper_metadatafile_delete_entry_thorough_check(path);

        let doc = open(path.to_string() + "/Metadata").unwrap();

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
                    "metadata",
                    "Like lib.rs, but for apps.",
                )
                .unwrap()
            };

            add_entry(
                &add_main(),
                Some(".gitignore"),
                "metadata",
                "Tells git which files to ignore.",
            )
            .unwrap()
        };

        write(modified_doc, path.to_string() + "/Metadata").expect("failed to write toml to disk");

        let expected = r#"['about']
version = "0.2.0"
name = "NAME"
author = "CHANGED_AUTHOR"
git-metadata-version = "0.1.0"

['src/lib.rs']
metadata = "Entry point to the library."

['src/main.rs']
metadata = "Like lib.rs, but for apps."

['.gitignore']
metadata = "Tells git which files to ignore."
"#;

        let doc = open(path.to_string() + "/Metadata").unwrap();

        Fixture::new().add_dirpath(path.to_string()).teardown(true);

        assert_eq!(doc.to_string(), expected)
    }

    #[test]
    fn metadatafile_delete_file_entry() {
        let path = ".metadata/.metadata_tests";
        let gpath = ".metadata/.metadata_tests/Metadata";
        let mut fixture = setup_test(path, "0.1.0", "NAME", "AUTHOR", "0.1.0");
        let doc = open(gpath).unwrap();
        let doc = add_entry(
            &doc,
            Some("src/lib.rs"),
            "metadata",
            "Entry point to the library.",
        )
        .unwrap();
        write(doc.clone(), gpath).expect("failed to write toml to disk");
        let metadata_res = metadata(&doc, Some("src/lib.rs"), "metadata").unwrap();

        assert_eq!(metadata_res, "Entry point to the library.");

        // Focus of test.
        let doc = open(gpath).unwrap();
        let doc = delete_entry(doc.clone(), "src/lib.rs").expect("failed to delete entry");
        write(doc, gpath).expect("failed to write toml to disk");

        let result = {
            let doc = open(".metadata/.metadata_tests/Metadata").unwrap();
            metadata(&doc, Some("src/lib.rs"), "metadata")
        };

        assert_eq!(result.is_ok(), false);

        fixture.teardown(true);
    }

    #[test]
    fn metadatafile_delete_dir_entry() {
        let path = ".metadata/.metadata_tests";
        let gpath = ".metadata/.metadata_tests/Metadata";
        let mut fixture = setup_test(path, "0.1.0", "NAME", "AUTHOR", "0.1.0");
        let doc = open(gpath).unwrap();
        let doc = add_entry(&doc, Some("src/"), "metadata", "The source code.").unwrap();
        write(doc.clone(), gpath).expect("failed to write toml to disk");
        let metadata_res = metadata(&doc, Some("src/"), "metadata").unwrap();

        assert_eq!(metadata_res, "The source code.");

        // Focus of test.
        let doc = open(gpath).unwrap();
        let doc = delete_entry(doc.clone(), "src/").expect("failed to delete entry");
        write(doc, gpath).expect("failed to write toml to disk");

        let result = {
            let doc = open(".metadata/.metadata_tests/Metadata").unwrap();
            metadata(&doc, Some("src/"), "metadata")
        };

        assert_eq!(result.is_ok(), false);

        fixture.teardown(true);
    }

    #[test]
    fn error_metadatafile_delete_dir_entry() {
        let path = ".metadata/.metadata_tests";
        let gpath = ".metadata/.metadata_tests/Metadata";
        let mut fixture = setup_test(path, "0.1.0", "NAME", "AUTHOR", "0.1.0");
        let doc = open(gpath).unwrap();
        let doc = add_entry(&doc, Some("src/"), "metadata", "The source code.").unwrap();
        write(doc.clone(), gpath).expect("failed to write toml to disk");
        let metadata_res = metadata(&doc, Some("src/"), "metadata").unwrap();

        assert_eq!(metadata_res, "The source code.");

        // Focus of test.
        let doc = open(gpath).unwrap();
        let doc = delete_entry(doc.clone(), "src"); // name must be exact, "src/"

        match doc {
            Err(err) => match err {
                MetadataFileError::Error(e) => assert_eq!(ErrorKind::InvalidKey, e.kind),
                _ => panic!("wrong error type"),
            },
            _ => panic!("expected an error"),
        }

        fixture.teardown(true);
    }
}
