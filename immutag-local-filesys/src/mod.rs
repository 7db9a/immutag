use fixture::{Fixture};


fn mkdir_filesys<T: AsRef<str>>(path: T) {
   let fixture = Fixture::new()
       .add_dirpath(path.as_ref().to_string());
       //.build();
}

#[cfg(test)]
mod integration {

    #[test]
    fn mkdir_filesys() {
        super::mkdir_filesys("path");
        assert_eq!(true, false)
    }
}
