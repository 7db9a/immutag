use fixture::Fixture;

fn path_filesys<T: AsRef<str>>(immutag_file_path: T, bitcoin_addr: T) -> String {
    let bitcoin_addr = fixture::directorate(bitcoin_addr.as_ref().to_string());
    let path = fixture::directorate(immutag_file_path.as_ref().to_string());
    let path = fixture::directorate(path + bitcoin_addr.as_ref());

    path
}

fn path_versionstore<T: AsRef<str>>(filesys_path: T) -> String {
    let path = fixture::directorate(filesys_path.as_ref().to_string());
    let path = fixture::directorate(filesys_path.as_ref().to_string() + "version-store");

    path
}

fn path_metadata<T: AsRef<str>>(filesys_path: T) -> String {
    let path = fixture::filefy(filesys_path.as_ref().to_string());
    let path = fixture::filefy(filesys_path.as_ref().to_string() + "metadata");

    path
}

fn mkdir_versionstore<T: AsRef<str>>(path: T) {
    let mut fixture = Fixture::new()
       .add_dirpath(path.as_ref().to_string())
       .build();
}

fn mkdir_metadata<T: AsRef<str>>(path: T) {
    let mut fixture = Fixture::new()
       .add_file(path.as_ref().to_string())
       .build();
}

fn mkdir_filesys<T: AsRef<str>>(filesys_path: T, versionstore_path: T, metadata_path: T) {
    let mut fixture = Fixture::new()
       .add_dirpath(filesys_path.as_ref().to_string())
       .add_dirpath(versionstore_path.as_ref().to_string())
       .add_file(metadata_path.as_ref().to_string())
       .add_git(filesys_path.as_ref().to_string())
       .build();
}


pub fn add_filesys<T: AsRef<str>>(immutag_file_path: T, bitcoin_addr: T) {
    let filesys_path = path_filesys(immutag_file_path, bitcoin_addr);
    let versionstore_path = path_versionstore(filesys_path.clone());
    let metadata_path = path_metadata(filesys_path.clone());
    mkdir_filesys(filesys_path, versionstore_path, metadata_path);
}

#[cfg(test)]
mod tests {

    #[test]
    fn path_filesys() {
        let path = super::path_filesys("/tmp/immutag_test/.immutag", "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG");

        assert_eq!(
            path,
            "/tmp/immutag_test/.immutag/1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG/".to_string()
        );
    }


    #[test]
    fn path_versionstore() {
        let path = super::path_versionstore("/tmp/immutag_test/.immutag/1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG/");

        assert_eq!(
            path,
            "/tmp/immutag_test/.immutag/1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG/version-store/".to_string()
        );
    }

    #[test]
    fn path_metadata() {
        let path = super::path_metadata("/tmp/immutag_test/.immutag/1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG/");

        assert_eq!(
            path,
            "/tmp/immutag_test/.immutag/1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG/metadata".to_string()
        );
    }
}

#[cfg(test)]
mod integration {
    use std::path::Path;
    use std::fs::metadata;

    #[test]
    fn add_filesys() {
        super::add_filesys("/tmp/immutag_test/.immutag", "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG");
        let filesys_path = "/tmp/immutag_test/.immutag/1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG";
        let versionstore_path = "/tmp/immutag_test/.immutag/1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG/version-store";
        let metadata_path = "/tmp/immutag_test/.immutag/1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG/metadata";
        let git_path = "/tmp/immutag_test/.immutag/1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG/.git";
        let md_filesys = metadata(filesys_path).unwrap();
        let md_versionstore = metadata(versionstore_path).unwrap();
        let md_metadata = metadata(metadata_path).unwrap();
        let md_git = metadata(git_path).unwrap();

        assert_eq!(true, md_filesys.is_dir());
        assert_eq!(true, md_versionstore.is_dir());
        assert_eq!(true, md_metadata.is_file());
        assert_eq!(true, md_git.is_dir());

        super::Fixture::new()
           .add_dirpath("/tmp/immutag_test/".to_string())
           .teardown(true);
    }

}
