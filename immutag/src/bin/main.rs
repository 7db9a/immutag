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
                   SubCommand::with_name("add")
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
                       .arg(
                           Arg::with_name("master-xpriv")
                               .takes_value(true)
                               .help("Set the master extended private key of the HD wallet.")
                               //.required(true)
                               .long("master-xpriv")
                       ),
                )
                .subcommand(
                   SubCommand::with_name("update")
                        .subcommand(
                           SubCommand::with_name("alias")
                               .arg(
                                   Arg::with_name("ALIAS")
                                       .required(true)
                                       .index(1)
                               ),
                        )
                )
        )
        .get_matches();

    //if let Some(matches) = matches.subcommand_matches("init") {
    //    let mut path: &'static str;
    //    path = "Immutag/";
    //    local_files::immutag_file_init(path, "0.1.0");
    //    println!("Initialized immutag in the current directory.");
    //}
    //if let Some(matches) = matches.subcommand_matches("add-fs") {
    //    let mut path: &'static str;
    //    path = "Immutag/";
    //    let mut ledgeraddr = "";
    //    if let Some(mut in_ledgeraddr) = matches.values_of("ledger-addr") {
    //        ledgeraddr = in_ledgeraddr.next().unwrap();
    //        //println!("Add filesystem: match ledger addr: {:#?}.", ledgeraddr);
    //    } else {
    //        println!("Shouldn't be allowed.");
    //    }

    //    if let Some(mut in_masterxpriv) = matches.values_of("master-xpriv") {
    //        let masterxpriv = in_masterxpriv.next().unwrap();
    //        if ledgeraddr != "" {
    //            local_files::add_filesystem(path, &ledgeraddr, masterxpriv);
    //            //println!("Add filesystem: match master-xpriv-: {:#?}.", masterxpriv);
    //            println!("Add filesystem.");
    //        }

    //    }
    //}
}
