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
                   SubCommand::with_name("new")
                       .subcommand(
                          SubCommand::with_name("content")
                              .arg(
                                  Arg::with_name("FILE")
                                      .required(true)
                                      .index(1)
                              )
                              .arg(
                                  Arg::with_name("set-alias")
                                      .takes_value(true)
                                      .help("Set an alias for the file.")
                                      .long("alias")
                              ),
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
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn cli_addfilesys() {
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

}
