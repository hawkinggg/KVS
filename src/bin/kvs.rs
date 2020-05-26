use clap::{App, AppSettings, Arg, SubCommand};
use std::process;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("set")
                .about("set value for key, usage: kvs set key1 value1")
                .arg(Arg::with_name("key").index(1))
                .arg(Arg::with_name("value").index(2)),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("get value by key, usage: kvs get key1")
                .arg(Arg::with_name("key").index(1)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("remove value by key, usage: kvs rm key1")
                .arg(Arg::with_name("key").index(1)),
        )
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
}
