use fixture::{Fixture};


fn mkdir_filesys<T: AsRef<str>>(immutag_file_path: T, bitcoin_addr: T) -> String {
    let bitcoin_addr = fixture::directorate(bitcoin_addr.as_ref().to_string());
    let path = fixture::directorate(immutag_file_path.as_ref().to_string());
    let path = fixture::directorate(path + bitcoin_addr.as_ref());

    path
}

#[cfg(test)]
mod integration {

    #[test]
    fn mkdir_filesys() {
        let filesys_path = super::mkdir_filesys("tmp", "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG");
        assert_eq!(
            filesys_path,
            "/tmp/immutag_test/.immutag/1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG".to_string()
        )
    }
}
