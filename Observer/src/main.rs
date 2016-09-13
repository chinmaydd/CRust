// Including external crates
extern crate docopt;
extern crate rustc_serialize;
extern crate git2;

use docopt::Docopt;
use std::process::exit;
use std::thread;
use std::time::Duration;
use std::env;
use git2::Oid;

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

    // Setting a default value for the commit ID.
    let comm_id_string = "comm_id";
    env::set_var(comm_id_string, "000");

    // Infinite observer loop
    loop {
        thread::sleep(Duration::from_secs(interval as u64));
        let latest_commit_id = lib::observe().to_string();
        
        let previous_commit_id = match env::var(comm_id_string) {
            Ok(comm_id) => comm_id,
            Err(e) => panic!("Could not fetch the commit ID from the env: {}", e),
        };

        if latest_commit_id.eq(&previous_commit_id) {
            println!("Same!");
        } else {
            env::set_var(comm_id_string, latest_commit_id);
            println!("Different!");
            // do something here.
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
