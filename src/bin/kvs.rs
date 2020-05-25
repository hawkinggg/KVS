use std::process;
use std::env;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("kvs")
                    .version(env::var("CARGO_PKG_VERSION").unwrap().as_str())
                    .author("hong")
                    .about("A simple in-memory key value store")
                    .subcommand(SubCommand::with_name("set")
                        .about("set value for key")
                        .arg(Arg::with_name("key")
                            .index(1))
                        .arg(Arg::with_name("value")
                            .index(2)))
                    .subcommand(SubCommand::with_name("get")
                        .about("get value by key")
                        .arg(Arg::with_name("key")
                            .index(1)))
                    .subcommand(SubCommand::with_name("rm")
                        .about("remove value by key")
                        .arg(Arg::with_name("key")
                            .index(1)))
                    .get_matches();

    if matches.is_present("set") {
        eprintln!("sorry, unimplemented.");
        process::exit(1);
    }

    if matches.is_present("get") {
        eprintln!("sorry, unimplemented.");
        process::exit(1);
    }

    if matches.is_present("rm") {
        eprintln!("sorry, unimplemented.");
        process::exit(1);
    }

    process::exit(1);
}
    
