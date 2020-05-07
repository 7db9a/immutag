use fixture::Fixture;

fn path_filesys<T: AsRef<str>>(immutag_file_path: T, bitcoin_addr: T) -> String {
    let bitcoin_addr = fixture::directorate(bitcoin_addr.as_ref().to_string());
    let path = fixture::directorate(immutag_file_path.as_ref().to_string());
    let path = fixture::directorate(path + bitcoin_addr.as_ref());

    path
}

fn mkdir_filesys<T: AsRef<str>>(path: T) {
    let mut fixture = Fixture::new()
       .add_dirpath(path.as_ref().to_string())
       .build();
}

fn add_filesys<T: AsRef<str>>(immutag_file_path: T, bitcoin_addr: T) {
    let path = path_filesys(immutag_file_path, bitcoin_addr);
    mkdir_filesys(path);
}

#[cfg(test)]
mod tests {
    fn path_filesys() {
        let path = super::path_filesys("/tmp/immutag_test/.immutag", "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG");

        assert_eq!(
            path,
            "/tmp/immutag_test/.immutag/1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG/".to_string()
        );
    }
}

#[cfg(test)]
mod integration {
    use std::path::Path;

    #[test]
    fn add_filesys() {
        super::add_filesys("/tmp/immutag_test/.immutag", "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG");

        assert_eq!(
           Path::new("/tmp/immutag_test/.immutag/1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG/").exists()
           , false);
    }
}
