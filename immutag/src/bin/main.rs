#[macro_use]
extern crate clap;
extern crate immutag;

use clap::{App, Arg, SubCommand};
use immutag::{bitcoin, local_files};

//use git_metadata::common;
//use git_metadata::metadata;
//use git_metadata::metadata::*;

fn main() {
    let matches = App::new("immutag")
        .version(crate_version!())
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialize immutag filesystems.")
                .arg(
                    Arg::with_name("test")
                        .takes_value(false)
                        .help("Run command in test mode")
                        .long("test"),
                ),
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("init") {
        let mut path: &'static str;
        if let Some(_in_test) = matches.values_of("test") {
            path = ".immutag/.immutag_tests/Immutag";
            println!("Initialized immutag in the current directory in test mode.");
        } else {
            path = ".immutag/.immutag_tests";
        }
        local_files::immutag_file_init(path, "0.1.0");
        println!("Initialized immutag in the current directory.");
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

#[cfg(test)]
mod tests {
}
