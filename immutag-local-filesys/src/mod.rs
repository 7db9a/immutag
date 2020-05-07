use fixture::{Fixture};


fn mkdir_filesys<T: AsRef<str>>(path: T) -> String {
   let fixture = Fixture::new()
       .add_dirpath(path.as_ref().to_string());
        path.as_ref().to_string()
       //.build();
}

#[cfg(test)]
mod integration {

    #[test]
    fn mkdir_filesys() {
        let filesys_path = super::mkdir_filesys("path");
        assert_eq!(
            filesys_path,
            "/tmp/immutag_test/.immutag/1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG".to_string()
        )
    }
}
