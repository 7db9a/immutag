use fixture;

fn mkdir_filesys() {
}

#[cfg(test)]
mod integration {

    #[test]
    fn mkdir_filesys() {
        super::mkdir_filesys();
        assert_eq!(true, false)
    }
}
