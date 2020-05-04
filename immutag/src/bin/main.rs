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
                .arg(
                    Arg::with_name("path")
                        .takes_value(true)
                        .help("Set path to directory to be initialized.")
                        .short("p")
                        .long("path")
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

    //if let Some(matches) = matches.subcommand_matches("init") {
    //    let mut path: &'static str;
    //    path = "Immutag/";
    //    local_files::immutag_file_init(path, "0.1.0");
    //    println!("Initialized immutag in the current directory.");
    //}

    if let Some(matches) = matches.subcommand_matches("filesys") {
        if let Some(matches) = matches.subcommand_matches("import") {
            let ledger_addr = matches.value_of("LEDGER-ADDR");
            let xpriv = matches.value_of("MASTER-XPRIV");

            if let Some(l) = ledger_addr {
                if let Some(x) = xpriv {
                    println!("ledger-addr :{}\nxpriv: {}", l, x);
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
    use std::process::Command;

    #[test]
    fn cli_init() {
    }

    #[test]
    fn cli_importfilesys() {
        let output = Command::new("/immutag/target/debug/immutag")
            .arg("filesys")
            .arg("import")
            .arg("LEDGER-ADDR")
            .arg("XPRIV")
            .output()
            .expect("failed to execute immutag addfilesys process");

        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            "ledger-addr :LEDGER-ADDR\nxpriv: XPRIV\n"
        );
    }

    #[test]
    fn cli_addfile() {
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
    fn cli_addtag() {
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
