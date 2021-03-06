#[macro_use]
extern crate clap;
extern crate immutag;

use clap::{App, Arg, SubCommand};
use immutag::{bitcoin, files, filesys};


fn main() {
    let matches = App::new("immutag")
        .version(crate_version!())
        .subcommand(
            SubCommand::with_name("init")
                .arg(
                    Arg::with_name("PATH")
                    .index(1)
                ),
        )
        .subcommand(
            SubCommand::with_name("filesys")
                .subcommand(
                   SubCommand::with_name("import")
                       .arg(
                           Arg::with_name("LEDGER-ADDR")
                               .required(true)
                               .index(1)
                       )
                       .arg(
                           Arg::with_name("MASTER-XPRIV")
                               .required(true)
                               .index(2)
                       )
                       .arg(
                           Arg::with_name("PATH")
                               .index(3)
                       ),
                )
        )
        .subcommand(
            SubCommand::with_name("file")
                .subcommand(
                   SubCommand::with_name("add")
                       .subcommand(
                          SubCommand::with_name("content")
                              .arg(
                                  Arg::with_name("FILE")
                                      .required(true)
                              )
                              .arg(
                                  Arg::with_name("set-alias")
                                      .takes_value(true)
                                      .help("Set an alias for the file.")
                                      .long("alias")
                              ),
                       )
                       .subcommand(
                          SubCommand::with_name("tag")
                              .arg(
                                  Arg::with_name("TAG")
                                      .required(true)
                                      .index(1)
                              )
                              .arg(
                                  Arg::with_name("FILE")
                                      .required(true)
                                      .index(2)
                              )
                       )
                       .subcommand(
                          SubCommand::with_name("type")
                              .arg(
                                  Arg::with_name("FILE-TYPE")
                                      .required(true)
                                      .index(1)
                              )
                              .arg(
                                  Arg::with_name("FILE")
                                      .required(true)
                                      .index(2)
                              )
                       )
                       .subcommand(
                          SubCommand::with_name("msg")
                              .arg(
                                  Arg::with_name("MESSAGE")
                                      .required(true)
                                      .index(1)
                              )
                              .arg(
                                  Arg::with_name("FILE")
                                      .required(true)
                                      .index(2)
                              )
                       )
                )
                .subcommand(
                   SubCommand::with_name("update")
                       .arg(
                           Arg::with_name("FILE")
                               .required(true)
                               .index(1)
                       )
                       .arg(
                           Arg::with_name("alias")
                               .takes_value(true)
                               .help("Set an alias for the file.")
                               .long("alias")
                       ),
                )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        let mut path = matches.value_of("PATH");
        if let Some(p) = path {
            let mut path = files::directorate(p.to_string());
            files::immutag_file_init(path, "0.1.0".to_string());
            println!("Initialized immutag in {}.", p);
        } else {
            files::immutag_file_init("", "0.1.0");
            println!("Initialized immutag in the current directory.")

        }
    }

    if let Some(matches) = matches.subcommand_matches("filesys") {
        if let Some(matches) = matches.subcommand_matches("import") {
            let ledger_addr = matches.value_of("LEDGER-ADDR");
            let xpriv = matches.value_of("MASTER-XPRIV");
            let path = matches.value_of("PATH");

            if let Some(l) = ledger_addr {
                if let Some(x) = xpriv {
                    if let Some(p) = path {
                        println!("Adding filesys {}", l);
                        files::add_filesystem(p, l, x);
                        filesys::add_filesys(p, l);
                    } else {
                        let current_path = "";
                        println!("Adding filesys {}", l);
                        files::add_filesystem(current_path, l, x);
                        filesys::add_filesys(current_path, l);
                    }
                }
            } else {
                println!("filesys command fail")

            }
         }

    }

    if let Some(matches) = matches.subcommand_matches("file") {
        if let Some(matches) = matches.subcommand_matches("add") {
            if let Some(matches) = matches.subcommand_matches("content") {
                let file = matches.value_of("FILE");
                let alias = matches.value_of("set-alias");
                if let Some(f) = file {
                    if let Some(a) = alias {
                        println!("file: {}\nalias: {}", f, a);
                    } else {
                        println!("file: {}", f);
                    }
                } else {
                    println!("file command fail")

                }
            }
            if let Some(matches) = matches.subcommand_matches("tag") {
                let tag = matches.value_of("TAG");
                let file = matches.value_of("FILE");
                if let Some(t) = tag {
                    if let Some(f) = file {
                        println!("file: {}\ntag: {}", f, t);
                    }
                }
            }
         }

    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use std::fs::metadata;
    use super::{files, filesys};
    use files::command_assistors;
    use files::Fixture;
    use std::path::Path;
    use std::process::Command;

    #[test]
    fn cli_init() {
        // If you use ./target/debug/[package], it won't
        // reflect any re-compilation the test did.
        // Therefore, using the cargo command to run the
        // package binary is best.

        let test_path = std::path::Path::new("/tmp/immutag_test");

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

        let immutag_file_content = read_to_string("/tmp/immutag_test/.immutag/Immutag").unwrap();

        fixture.teardown(true);

        assert_eq!(
            &immutag_file_content,
            "[\'immutag\']\nversion = \"0.1.0\"\n"
        );

        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            "Initialized immutag in the current directory.\n"
        );
    }

    #[test]
    fn cli_init_path() {
        // If you use ./target/debug/[package], it won't
        // reflect any re-compilation the test did.
        // Therefore, using the cargo command to run the
        // package binary is best.

        let test_path = std::path::Path::new("/tmp/immutag_test");
        let mut test_path_string = test_path.to_str().unwrap().to_string();

        let mut test_path_string = files::directorate(test_path_string.clone());
        let mut fixture = Fixture::new()
           .add_dirpath(test_path_string + "here")
           .build();

        let mut path_cache = command_assistors::PathCache::new(&test_path);

        // Changing directories.
        path_cache.switch();

        let output = Command::new("/immutag/target/debug/immutag")
            .arg("init")
            .arg("here")
            .output()
            .expect("failed to execute immutag init process");

        path_cache.switch_back();

        let immutag_file_content = read_to_string("/tmp/immutag_test/here/.immutag/Immutag").unwrap();

        fixture.teardown(true);

        assert_eq!(
            &immutag_file_content,
            "[\'immutag\']\nversion = \"0.1.0\"\n"
        );

        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            "Initialized immutag in here.\n"
        );
    }

    #[test]
    fn cli_importfilesys() {
        let test_path = std::path::Path::new("/tmp/immutag_test");
        let mut test_path_string = test_path.to_str().unwrap().to_string();

        let mut test_path_string = files::directorate(test_path_string.clone());
        let mut fixture = Fixture::new()
           .add_dirpath(test_path_string.clone())
           .build();

        let mut path_cache = command_assistors::PathCache::new(&test_path);

        // Changing directories.
        path_cache.switch();

        let output_init = Command::new("/immutag/target/debug/immutag")
            .arg("init")
            .output()
            .expect("failed to execute immutag init process");

        let output_filesys_import = Command::new("/immutag/target/debug/immutag")
            .arg("filesys")
            .arg("import")
            .arg("1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG")
            .arg("XPRIV")
            .output()
            .expect("failed to execute immutag addfilesys process");

        path_cache.switch_back();

        let immutag_file_content = read_to_string("/tmp/immutag_test/.immutag/Immutag").unwrap();

        let xpriv = files::get_xpriv(
            test_path_string,
            "1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG".to_string()
        );

        assert_eq!(
            &immutag_file_content,
            "[\'immutag\']\nversion = \"0.1.0\"\n\n[\'1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG\']\nxpriv = \"XPRIV\"\n"
        );

        assert_eq!(
            String::from_utf8_lossy(&output_filesys_import.stdout),
            "Adding filesys 1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG\n"
        );

        assert_eq!(xpriv.unwrap(), "XPRIV");

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

        fixture.teardown(true);
    }

    #[test]
    fn cli_output_addfile() {
        let output_no_option = Command::new("/immutag/target/debug/immutag")
            .arg("file")
            .arg("add")
            .arg("content")
            .arg("FILE")
            .output()
            .expect("failed to execute immutag add content process");

        let output_option = Command::new("/immutag/target/debug/immutag")
            .arg("file")
            .arg("add")
            .arg("content")
            .arg("--alias")
            .arg("ALIAS")
            .arg("FILE")
            .output()
            .expect("failed to execute immutag add content process");

        let output_option_position = Command::new("/immutag/target/debug/immutag")
            .arg("file")
            .arg("add")
            .arg("content")
            .arg("FILE")
            .arg("--alias") // Optional arg after required arg.
            .arg("ALIAS")
            .output()
            .expect("failed to execute immutag add content process");

        assert_eq!(
            String::from_utf8_lossy(&output_no_option.stdout),
            "file: FILE\n"
        );
        assert_eq!(
            String::from_utf8_lossy(&output_option.stdout),
            "file: FILE\nalias: ALIAS\n"
        );
        assert_eq!(
            String::from_utf8_lossy(&output_option_position.stdout),
            "file: FILE\nalias: ALIAS\n"
        );
    }

    #[test]
    fn cli_output_addtag() {
        let output = Command::new("/immutag/target/debug/immutag")
            .arg("file")
            .arg("add")
            .arg("tag")
            .arg("TAG")
            .arg("FILE")
            .output()
            .expect("failed to execute immutag add tag process");

        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            "file: FILE\ntag: TAG\n"
        );
    }
}
