#[macro_use]
extern crate clap;
extern crate immutag;

use clap::{App, Arg, SubCommand};
use immutag::{bitcoin, local_files};

fn main() {
    let matches = App::new("immutag")
        .version(crate_version!())
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialize immutag filesystems.")
        )
        .subcommand(
            SubCommand::with_name("add-fs")
                .about("Add filesystem.")
                .arg(
                    Arg::with_name("LEDGER-ADDR")
                        .help("Set the address to the filesystem from the HD wallet (e.g. Bitcoin).")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::with_name("MASTER-XPRIV")
                        .help("Set the master extended private key of the HD wallet.")
                        .required(true)
                        .index(2)
                ),
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("init") {
        let mut path: &'static str;
        path = "Immutag/";
        local_files::immutag_file_init(path, "0.1.0");
        println!("Initialized immutag in the current directory.");
    }
    if let Some(matches) = matches.subcommand_matches("add-fs") {
        let mut path: &'static str;
        path = "Immutag/";
        println!("Add filesystem.");
        if let Some(mut in_ledgeraddr) = matches.values_of("LEDGER-ADDR") {
            let ledgeraddr = in_ledgeraddr.next().unwrap();
            if let Some(mut in_masterxpriv) = matches.values_of("MASTER-XPRIV") {
                let masterxpriv = in_masterxpriv.next().unwrap();
                local_files::add_filesystem(path, ledgeraddr, masterxpriv);
            }
        } else {
            println!("Shouldn't be allowed.");
        }
    }

    // If we set the multiple() option of a flag we can check how many times the user specified
    //
    // Note: if we did not specify the multiple() option, and the user used "awesome" we would get
    // a 1 (no matter how many times they actually used it), or a 0 if they didn't use it at all
    //match matches.occurrences_of("awesome") {
    //    0 => println!("Nothing is awesome"),
    //    1 => println!("Some things are awesome"),
    //    2 => println!("Lots of things are awesome"),
    //    3 | _ => println!("EVERYTHING is awesome!"),
    //}

    // Continued program logic goes here...
}


/// Switch back and forth between paths when executing test commands.
mod command_assistors {
    use std::env;
    use std::path::Path;

    pub struct PathCache<'s> {
        from_path: Box<Path>,
        to_path: &'s Path,
    }

    impl<'s> PathCache<'s> {
        pub fn new(to_path: &Path) -> PathCache {
            let current_dir = env::current_dir().expect("failed to get current dir");
            let from_path = current_dir.into_boxed_path();

            PathCache { from_path, to_path }
        }

        pub fn switch(&mut self) {
            if env::set_current_dir(&self.to_path).is_err() {
                panic!("failed to switch back to original dir")
            }
        }

        pub fn switch_back(&mut self) {
            if env::set_current_dir(&self.from_path).is_err() {
                panic!("failed to switch back to original dir")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{local_files, command_assistors};
    use local_files::Fixture;
    use std::path::Path;
    use std::process::Command;

    #[test]
    fn echo() {
        let output = Command::new("echo")
            .arg("test")
            .output()
            .expect("failed to execute process");

        assert_eq!(String::from_utf8_lossy(&output.stdout), ("test\n"))
    }

    #[test]
    fn cli_init() {
        // If you use ./target/debug/[package], it won't
        // reflect any re-compilation the test did.
        // Therefore, using the cargo command to run the
        // package binary is best.

        let test_path = std::path::Path::new("/tmp/immutag_tests");

        let mut fixture = Fixture::new()
           .add_dirpath(test_path.to_str().unwrap().to_string())
           .build();

        let mut path_cache = command_assistors::PathCache::new(&test_path);

        // Changing directories.
        path_cache.switch();

        let output = Command::new("/immutag/target/debug/immutag")
            .arg("init")
            .output()
            .expect("failed to execute immutag init process");

        path_cache.switch_back();

        fixture.teardown(true);

        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            "Initialized immutag in the current directory.\n"
        );
    }
}
