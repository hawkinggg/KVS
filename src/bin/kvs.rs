use structopt::StructOpt;
use std::process;

#[derive(StructOpt, Debug)]
#[structopt(about, author)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(about = "set value for key, usage: kvs set key1 value1")]
    Set {
        key: String,
        value: String
    },
    #[structopt(about = "get value by key, usage: kvs get key1")]
    Get {
        key: String
    },
    #[structopt(about = "remove value by key, usage: kvs rm key1")]
    Rm {
        key: String
    }
}

fn main() {
    let matches = Opt::clap().get_matches();

    if matches.is_present("set") {
        eprintln!("Sorry, method unimplemented.");
        process::exit(1);
    }

    if matches.is_present("get") {
        eprintln!("Sorry, method unimplemented.");
        process::exit(1);
    }

    if matches.is_present("rm") {
        eprintln!("Sorry, method unimplemented.");
        process::exit(1);
    }
}
