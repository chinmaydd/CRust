// Including external crates
extern crate docopt;
extern crate rustc_serialize;
extern crate git2;

use docopt::Docopt;
use std::process::exit;
use std::thread;
use std::time::Duration;

mod lib;

const USAGE: &'static str = "
Crust::Observer: Used to observe changes in a git repository and report it to a server.

Usage:
    observer [options] [<directory>]

Options:
    -h --help                       Show this screen
    -i --interval=<time_interval>   Time interval between repository observations(in seconds)
    -a --address=<network_address>  Network address to post the response of the observation
    -p --port=<port_number>         Port on the given network address to which the response is to be sent
";

// Docopt helps us cast the given input into Rust types!
#[derive(Debug, RustcDecodable)]
struct Args {
    flag_help: bool,
    flag_interval: Option<i32>,
    flag_address: Option<String>,
    flag_port: Option<i32>,
    arg_directory: Option<String>
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    // If there is no directory name given, we exit the program.
    if args.arg_directory.is_none() {
        println!("Please input the directory name.");
        exit(0);
    }
 
    // Since the type of each of the flags and arguments is `Option` we need to unwrap them first.
    let directory = args.arg_directory.unwrap();

    // Setting the default interval value as 5 if no options are given.
    let interval = args.flag_interval.unwrap_or_else(|| 5);

    // Configure the environment variables.
    lib::configure(directory, interval);

    // Infinite observer loop
    loop {
        thread::sleep(Duration::from_secs(interval as u64));
        let comm = lib::observe();
        println!("Current HEAD Commit ID: {}", comm);
    }
    // lib::test_function();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
