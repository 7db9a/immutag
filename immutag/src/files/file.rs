use immutag_file::{value, Document, read_to_string};
pub use fixture::{Fixture, directorate, command_assistors};
pub use immutag_file::{ErrorKind, ImmutagFileError, ImmutagFileState};

// get_nickname
//
//
//

/// Creates a Immutag file with basic info.
pub fn init<T: AsRef<str>>(path: T, version: T) -> Result<(), ImmutagFileError> {
    let (filepath, _) = filemaker(path);
    let doc = immutag_file::init(filepath.as_ref(), version.as_ref());

    immutag_file::write(doc?, filepath)
}

pub fn add_filesystem<T: AsRef<str>>(
    path: T,
    bitcoin_addr: T,
    xpriv: T
) -> Result<(), ImmutagFileError> {
    let (filepath, _) = filemaker(path);
    let doc = open(filepath.clone()).unwrap();
    let doc = add_entry(
        &doc,
        Some(bitcoin_addr.as_ref()),
        "xpriv",
        xpriv.as_ref(),
    );

    write(doc?, filepath)
}

/// Valid if the version field can be read. Should rename pass
/// toml value into method, that other fields can be validated.
pub fn is_valid(doc: &Document) -> ImmutagFileState {
    immutag_file::is_valid(doc)
}

pub fn get_xpriv<T: AsRef<str>>(
    path: T,
    bitcoin_addr: T,
) -> Result<String, ImmutagFileError> {
    let (filepath, _) = filemaker(path);
    let doc = open(filepath.clone()).unwrap();

    immutag_file::immutag(&doc, Some(bitcoin_addr.as_ref()), "xpriv")
}

pub fn get_mnemonic<T: AsRef<str>>(
    path: T,
    bitcoin_addr: T,
) -> Result<String, ImmutagFileError> {
    let (filepath, _) = filemaker(path);
    let doc = open(filepath.clone()).unwrap();

    immutag_file::immutag(&doc, Some(bitcoin_addr.as_ref()), "mnemonic")
}
///! Retrieve field data from a Immutag file. For example, if the file name is provided, it will attempt to retrieve the field `immutag` nested in the `README.md` entry.
///!  ```ignore
///!  [README.md]
///!  immutag = "The README."
///!  ```
///! If no file name is given, it will retrieve all the nested value in the key and not necessarily a specific field.
fn immutag<T: AsRef<str>>(
    doc: &Document,
    file_name: Option<T>,
    key: T,
) -> Result<String, ImmutagFileError> {
    immutag_file::immutag(doc, file_name, key)
}

pub fn entry_exists<T: AsRef<str>>(doc: &Document, key: T, key_nested: Option<T>) -> bool {
    immutag_file::entry_exists(doc, key, key_nested)
}

/// A crude way to find if an entry exits. Doesn't work for nested etnries.
/// `path` paramaters is the path to the Immutag file.
pub fn exists<T: AsRef<str>>(path: T, name: T) -> bool {
    immutag_file::exists(path, name)
}

pub fn insert_entry<T: AsRef<str>>(
    doc: &Document,
    file_name: Option<T>,
    key: T,
    immutag: T,
) -> Result<Document, ImmutagFileError> {
    immutag_file::insert_entry(doc, file_name, key, immutag)
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

fn open<T: AsRef<str>>(path: T) -> Result<Document, ImmutagFileError> {
    immutag_file::open(path)
}

fn write<T: AsRef<str>>(toml_doc: Document, path: T) -> Result<(), ImmutagFileError> {
    immutag_file::write(toml_doc, path)
}
fn dirpath<T: AsRef<str>>(path: T) -> String {
    let mut path = path.as_ref().to_string();
    path = fixture::directorate(path);
    path = fixture::directorate(path + ".immutag");

    path
}

fn paths<T: AsRef<str>>(path: T) -> (String, String) {
    let dirpath = dirpath(path);
    let filepath = fixture::filefy(dirpath.clone() + "/Immutag");

    (dirpath, filepath)
}

fn filemaker<T: AsRef<str>>(path: T) -> (String, Fixture) {
    let (dirpath, filepath) = paths(path);

    let fixture = Fixture::new()
        .add_dirpath(dirpath.clone())
        .build();

    (filepath, fixture)
}

#[cfg(test)]
mod integration {
    use super::*;
    use fixture::Fixture;

    #[test]
    fn immutagfile_init() {
        let path = "/tmp/immutag_tests";
        let (filepath, mut fixture) = filemaker(path);
        init(path, "0.1.0");
        let doc = open(filepath.clone()).unwrap();
        let is_valid = is_valid(&doc);
        let doc = open(filepath).unwrap();
        let expected = r#"['immutag']
version = "0.1.0"
"#;
        fixture.teardown(true);

        assert_eq!(is_valid, ImmutagFileState::Valid);
        assert_eq!(doc.to_string(), expected);
    }

    #[test]
    fn immutagfile_add_filesystem() {
        let path = "/tmp/immutag_tests";
        init(path, "0.1.0");
        add_filesystem(
            path,
            "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG",
            "XPRIV",
        )
        .unwrap();
        let xpriv = get_xpriv(path, "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG").unwrap();
        Fixture::new()
            .add_dirpath(path.to_string())
            .teardown(true);

        assert_eq!(xpriv, "XPRIV");
    }

    #[test]
    fn immutagfile_error_add_entry() {
        let path = "/tmp/immutag_tests";
        init(path, "0.1.0");
        add_filesystem(
            path,
            "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG",
            "XPRIV",
        )
        .unwrap();

        let xpriv = get_xpriv(path, "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG").unwrap();
        let add_again_res = add_filesystem(
            path,
            "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG",
            "XPRIV-WRONG",
        );

        Fixture::new()
            .add_dirpath(path.to_string())
            .teardown(true);

        assert_eq!(xpriv, "XPRIV");
        assert!(add_again_res.is_err());
    }


    // Verifies there is no unexpected whitespace or formatting issuees for a basic case.
    #[test]
    fn format_immutagfile_file_add_entry() {
        let path = "/tmp/immutag_tests";
        let (filepath, mut fixture) = filemaker(path);
        init(path, "0.1.0");

        let doc = open(filepath.clone()).unwrap();

        let doc = add_filesystem(
            path,
            "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG",
            "XPRIV",
        )
        .unwrap();

        // Focus of test.
        let toml_string = read_to_string(filepath.clone()).expect("failed to read immutagfile");

        let doc = open(filepath).unwrap();

        //let mut doc = toml_string.parse::<Document>().expect("failed to get toml doc");
        //doc["1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG"].as_inline_table_mut().map(|t| t.fmt());
        let expected = r#"['immutag']
version = "0.1.0"

['1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG']
xpriv = "XPRIV"
"#;

        fixture.teardown(true);

        assert_eq!(doc.to_string(), expected);
        assert_eq!(toml_string, expected);
    }

    #[test]
    fn immutagfile_entry_exists() {
        let path = "/tmp/immutag_tests";
        let (filepath, mut fixture) = filemaker(path);
        init(path, "0.1.0");
        add_filesystem(
            path,
            "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG",
            "XPRIV",
        )
        .unwrap();
        let doc = open(filepath.clone()).unwrap();

        assert_eq!(entry_exists(&doc, "NOT_REAL_BITCON_ADD_A", None), false);
        assert_eq!(entry_exists(&doc, "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG", None), true);
        assert_eq!(exists(filepath.clone(), "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG".to_string()), true);
        assert_eq!(exists(filepath, "NOT_REAL_BITCOIN_ADD_B".to_string()), false);

        fixture.teardown(true);
    }

    fn helper_immutagfile_delete_entry_thorough_check<T: AsRef<str>>(path_to_dir: T) {
        let path = "/tmp/immutag_tests";
        let (filepath, _) = filemaker(path);
        init(path, "0.1.0");
        add_filesystem(
            path,
            "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG",
            "XPRIV",
        )
        .unwrap();
        let doc = open(filepath.clone()).unwrap();

        let lib_exists = entry_exists(&doc, "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG", None);

        let doc = add_entry(&doc, Some("1JvFXyZMC31ShnD8PSKgN1HKQ2kGQLVpCt"), "xpriv", "XPRIV").unwrap();

        write(doc.clone(), filepath.clone()).expect("failed to write toml to disk");

        let new_doc = delete_entry(doc, "1JvFXyZMC31ShnD8PSKgN1HKQ2kGQLVpCt").unwrap();
        write(new_doc.clone(), filepath).expect("failed to write toml to disk");

        let expected = r#"['immutag']
version = "0.1.0"

['1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG']
xpriv = "XPRIV"
"#;

        assert_eq!(lib_exists, true);
        assert_eq!(new_doc.to_string(), expected)
    }

     #[test]
     fn immutagfile_delete_entry_thorough_assert() {
         let path = "/tmp/immutag_tests";
         helper_immutagfile_delete_entry_thorough_check(path);

         Fixture::new().add_dirpath(path.to_string()).teardown(true);
     }


    #[test]
    fn immutagfile_delete_file_entry() {
        let path = "/tmp/immutag_tests";
        let (filepath, mut fixture) = filemaker(path);
        init(path, "0.1.0");
        add_filesystem(
            path,
            "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG",
            "XPRIV",
        )
        .unwrap();
        let doc = open(filepath.clone()).unwrap();
        write(doc.clone(), filepath.clone()).expect("failed to write toml to disk");

        let xpriv = get_xpriv(path, "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG").unwrap();
        assert_eq!(xpriv, "XPRIV");

        // Focus of test.
        let doc = open(filepath.clone()).unwrap();
        let doc = delete_entry(doc.clone(), "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG").expect("failed to delete entry");
        write(doc, filepath).expect("failed to write toml to disk");

        let result = {
            let doc = open("/tmp/immutag_tests/.immutag/Immutag").unwrap();

            get_xpriv(path, "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG")
        };

        assert_eq!(result.is_ok(), false);

        fixture.teardown(true);
    }
}
