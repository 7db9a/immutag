use immutag_filesys;
use fixture;

pub fn add_filesys<T: AsRef<str>>(path: T, bitcoin_addr: T) {
    let mut path = fixture::directorate(path.as_ref().to_string());
    path = fixture::directorate(path.to_string() + ".immutag");

    immutag_filesys::add_filesys(path, bitcoin_addr.as_ref().to_string())
}

#[cfg(test)]
mod integration {
    use std::path::Path;
    use std::fs::metadata;
    use fixture;

    #[test]
    fn add_filesys() {
        super::add_filesys("/tmp/immutag_test/", "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG");
        let filesys_path = "/tmp/immutag_test/.immutag/1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG";
        let versionstore_path = "/tmp/immutag_test/.immutag/1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG/version-store";
        let metadata_path = "/tmp/immutag_test/.immutag/1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG/metadata";
        let git_path = "/tmp/immutag_test/.immutag/1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG/.git";
        let md_filesys = metadata(filesys_path).unwrap();
        let md_versionstore = metadata(versionstore_path).unwrap();
        let md_metadata = metadata(metadata_path).unwrap();
        let md_git = metadata(git_path).unwrap();
        let is_git = fixture::is_git(git_path);
        let is_git_versionstore = fixture::is_git(versionstore_path);

        assert_eq!(true, md_filesys.is_dir());
        assert_eq!(true, md_versionstore.is_dir());
        assert_eq!(true, md_metadata.is_file());
        assert_eq!(true, md_git.is_dir());
        assert_eq!(true, is_git);
        assert_eq!(false, is_git_versionstore);

        fixture::Fixture::new()
           .add_dirpath("/tmp/immutag_test/".to_string())
           .teardown(true);
    }

}
