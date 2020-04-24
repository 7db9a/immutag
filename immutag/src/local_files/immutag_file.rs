use immutag_local_files;

//use limmutag_file::{};

use immutag_local_files::{value, Document, ErrorKind, ImmutagFileError, ImmutagFileState, common, read_to_string};
use common::Fixture;

// init

// set_filesystem(bitcoin-addr, xpriv, mnemonic)

// write(toml: String)

// get_xpriv

// get_mnemonic

// get_nickname

/// Creates a Immutag file with basic info.
pub fn init<T: AsRef<str>>(path: T, version: T) {
    let fixture = Fixture::new()
        .add_dirpath(path.as_ref().to_string())
        .build();

    let gpath = common::directorate(path.as_ref().to_string())+ "Immutag";

    immutag_local_files::init(gpath, version.as_ref().to_string());
}

pub fn add_filesystem<T: AsRef<str>>(
    path: T,
    bitcoin_addr: T,
    xpriv: T
) -> Result<(), ImmutagFileError> {
    let gpath = common::directorate(path.as_ref().to_string())+ "Immutag";
    let doc = open(gpath.clone()).unwrap();
    let doc = add_entry(
        &doc,
        Some(bitcoin_addr.as_ref()),
        "xpriv",
        xpriv.as_ref(),
    )
    .unwrap();

    write(doc.clone(), gpath)
}

pub fn open<T: AsRef<str>>(path: T) -> Result<Document, ImmutagFileError> {
    immutag_local_files::open(path)
}

pub fn write<T: AsRef<str>>(toml_doc: Document, path: T) -> Result<(), ImmutagFileError> {
    immutag_local_files::write(toml_doc, path)
}

/// Valid if the version field can be read. Should rename pass
/// toml value into method, that other fields can be validated.
pub fn is_valid(doc: &Document) -> ImmutagFileState {
    immutag_local_files::is_valid(doc)
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
    immutag_local_files::immutag(doc, file_name, key)
}

pub fn entry_exists<T: AsRef<str>>(doc: &Document, key: T, key_nested: Option<T>) -> bool {
    immutag_local_files::entry_exists(doc, key, key_nested)
}

/// A crude way to find if an entry exits. Doesn't work for nested etnries.
/// `path` paramaters is the path to the Immutag file.
pub fn exists<T: AsRef<str>>(path: T, name: T) -> bool {
    immutag_local_files::exists(path, name)
}

pub fn insert_entry<T: AsRef<str>>(
    doc: &Document,
    file_name: Option<T>,
    key: T,
    immutag: T,
) -> Result<Document, ImmutagFileError> {
    immutag_local_files::insert_entry(doc, file_name, key, immutag)
}

pub fn add_entry<T: AsRef<str>>(
    doc: &Document,
    file_name: Option<T>,
    name: T,
    immutag: T,
) -> Result<Document, ImmutagFileError> {
    immutag_local_files::add_entry(doc, file_name, name, immutag)
}

pub fn delete_entry<T: AsRef<str>>(
    doc: Document,
    file_name: T,
) -> Result<Document, ImmutagFileError> {
    immutag_local_files::delete_entry(doc, file_name)
}

#[cfg(test)]
mod integration {
    use super::*;
    use common::Fixture;

    pub fn setup_test<T: AsRef<str>>(
        path: T,
        version: T
    ) -> Fixture {
        let fixture = Fixture::new()
            .add_dirpath(path.as_ref().to_string())
            .build();

        init(
            path.as_ref().to_string() + "/Immutag",
            version.as_ref().to_string(),
        );

        fixture
    }

    pub fn setup_add<T: AsRef<str>>(
        immutag_path: T,
    ) -> (Document, Result<String, ImmutagFileError>) {
        let doc = open(immutag_path.as_ref()).unwrap();
        let doc = add_entry(
            &doc,
            Some("1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG"),
            "xpriv",
            "XPRIV",
        )
        .unwrap();
        write(doc.clone(), immutag_path.as_ref()).expect("failed to write toml to disk");
        let immutag_res = immutag(&doc, Some("1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG"), "xpriv");

        (doc, immutag_res)
    }

    #[test]
    fn immutagfile_init() {
        let path = ".immutag/.immutag_tests";
        let gpath = ".immutag/.immutag_tests/Immutag";

        init(path, "0.1.0");

        let doc = open(gpath).unwrap();
        let is_valid = is_valid(&doc);
        let doc = open(gpath).unwrap();
        let expected = r#"['immutag']
version = "0.1.0"
"#;
        Fixture::new()
            .add_dirpath(path.to_string())
            .teardown(true);

        assert_eq!(is_valid, ImmutagFileState::Valid);
        assert_eq!(doc.to_string(), expected);
    }

    #[test]
    fn immutagfile_add_filesystem() {
        let path = ".immutag/.immutag_tests";
        let gpath = ".immutag/.immutag_tests/Immutag";

        init(path, "0.1.0");

        let doc = open(gpath).unwrap();

        let doc = add_filesystem(
            path,
            "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG",
            "XPRIV",
        )
        .unwrap();

        let doc = open(gpath).unwrap();
        let immutag_res = immutag(&doc, Some("1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG"), "xpriv").unwrap();
        Fixture::new()
            .add_dirpath(path.to_string())
            .teardown(true);
        assert_eq!(immutag_res, "XPRIV");
    }

    #[test]
    fn immutagfile_error_add_entry() {
        let path = ".immutag/.immutag_tests";
        let gpath = ".immutag/.immutag_tests/Immutag";
        let mut fixture = setup_test(path, "0.1.0");
        let (doc, immutag) = setup_add(gpath);

        // Focus of test.
        let result = add_entry(
            &doc,
            Some("1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG"),
            "xpriv",
            "XPRIV_OTHER",
        );

        fixture.teardown(true);

        assert_eq!(immutag.unwrap(), "XPRIV");
        assert!(result.is_err());
    }


    // Verifies there is no unexpected whitespace or formatting issuees for a basic case.
    #[test]
    fn format_immutagfile_file_add_entry() {
        let path = ".immutag/.immutag_tests";
        let gpath = ".immutag/.immutag_tests/Immutag";

        init(path, "0.1.0");

        let doc = open(gpath).unwrap();

        let doc = add_filesystem(
            path,
            "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG",
            "XPRIV",
        )
        .unwrap();

        // Focus of test.
        let toml_string = read_to_string(gpath).expect("failed to read immutagfile");

        let doc = open(gpath).unwrap();

        //let mut doc = toml_string.parse::<Document>().expect("failed to get toml doc");
        //doc["1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG"].as_inline_table_mut().map(|t| t.fmt());
        let expected = r#"['immutag']
version = "0.1.0"

['1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG']
xpriv = "XPRIV"
"#;

        Fixture::new()
            .add_dirpath(path.to_string())
            .teardown(true);

        assert_eq!(doc.to_string(), expected);
        assert_eq!(toml_string, expected);
    }

    #[test]
    fn immutagfile_entry_exists() {
        let path = ".immutag/.immutag_tests";
        let gpath = ".immutag/.immutag_tests/Immutag";
        init(path, "0.1.0");
        add_filesystem(
            path,
            "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG",
            "XPRIV",
        )
        .unwrap();
        let doc = open(gpath).unwrap();

        assert_eq!(entry_exists(&doc, "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG", None), true);

        assert_eq!(exists(gpath, "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG"), true);

        assert_eq!(entry_exists(&doc, "NOT_REAL_BITCON_ADD_A", None), false);

        assert_eq!(exists(gpath, "NOT_REAL_BITCOIN_ADD_B"), false);

        Fixture::new()
            .add_dirpath(path.to_string())
            .teardown(true);
    }

    fn helper_immutagfile_delete_entry_thorough_check<T: AsRef<str>>(path_to_dir: T) {
        let path = path_to_dir;
        let gpath = path.as_ref().to_string() + "/Immutag";
        let _fixture = setup_test(path.as_ref(), "0.1.0") ;

        let (doc, _) = setup_add(gpath.as_str());

        let lib_exists = entry_exists(&doc, "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG", None);

        let doc = add_entry(&doc, Some("1JvFXyZMC31ShnD8PSKgN1HKQ2kGQLVpCt"), "xpriv", "XPRIV").unwrap();

        write(doc.clone(), gpath.as_str()).expect("failed to write toml to disk");

        let new_doc = delete_entry(doc, "1JvFXyZMC31ShnD8PSKgN1HKQ2kGQLVpCt").unwrap();
        write(new_doc.clone(), gpath).expect("failed to write toml to disk");

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
         let path = ".immutag/.immutag_tests";
         helper_immutagfile_delete_entry_thorough_check(path);

         Fixture::new().add_dirpath(path.to_string()).teardown(true);
     }


    #[test]
    fn immutagfile_delete_file_entry() {
        let path = ".immutag/.immutag_tests";
        let gpath = ".immutag/.immutag_tests/Immutag";
        let mut fixture = setup_test(path, "0.1.0");
        let doc = open(gpath).unwrap();
        let doc = add_entry(
            &doc,
            Some("1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG"),
            "xpriv",
            "XPRIV",
        )
        .unwrap();
        write(doc.clone(), gpath).expect("failed to write toml to disk");
        let immutag_res = immutag(&doc, Some("1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG"), "xpriv").unwrap();

        assert_eq!(immutag_res, "XPRIV");

        // Focus of test.
        let doc = open(gpath).unwrap();
        let doc = delete_entry(doc.clone(), "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG").expect("failed to delete entry");
        write(doc, gpath).expect("failed to write toml to disk");

        let result = {
            let doc = open(".immutag/.immutag_tests/Immutag").unwrap();
            immutag(&doc, Some("1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG"), "xpriv")
        };

        assert_eq!(result.is_ok(), false);

        fixture.teardown(true);
    }
}
