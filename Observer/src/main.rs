// Including external crates
extern crate docopt;
extern crate rustc_serialize;
extern crate git2;
extern crate hyper;

use docopt::Docopt;
use std::process::exit;
use std::thread;
use std::time::Duration;
use std::env;
use git2::Oid;
use hyper::Client;

mod lib;

// Helper string for `-h` option.
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
    // Decode the arguments into the Docopt structure.
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

    // Configure the environment variables of repository and interval.
    lib::configure(directory, interval);

    // Infinite observer loop
    loop {
        // Sleep for `interval` seconds.
        thread::sleep(Duration::from_secs(interval as u64));

        // The observe function is supposed to return the latest commit ID.
        let latest_commit_id = lib::observe().to_string();
        
        // Get the commit ID of the previous commit in the env variable.
        let previous_commit_id = match env::var("comm_id") {
            Ok(comm_id) => comm_id,
            Err(e) => panic!("Could not fetch the commit ID from the env: {}", e),
        };

        // Check if there have been any more commits.
        if latest_commit_id.eq(&previous_commit_id) {
            println!("Same!");
        } else {
            // Set the new latest commit id in the env variable.
            env::set_var("comm_id", latest_commit_id.clone());
            // println!("Different!");

            // Set the latest commit string to be sent to the dispatcher.
            let temp_id: String = latest_commit_id.to_owned();
            let mut request_string: String = "comm_id=".to_owned();
            request_string.push_str(&temp_id);

            println!("{}", request_string);
           
            // Set up a new client.
            // Make a post request to the dispatcher end point.
            // This contains the commit ID of the latest commit ID.
            let client = Client::new();
            let res = client.post("localhost:4002")
                            .body(&request_string)
                            .send()
                            .unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
