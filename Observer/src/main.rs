// Including external crates
extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;

const USAGE: &'static str = "
Crust::Observer: Used to observe changes in a git repository and report it to a server.

Usage:
    observer [options] <directory>

Options:
    -h --help                       Show this screen
    -i --interval=<time_interval>   Time interval between repository observations(in seconds)
    -a --address=<network_address>  Network address to post the response of the observation
    -p --port=<port_number>         Port on the given network address to which the response is to be sent
";

// Docopt helps us derive the arguments and parse them into relevant types.
// Reference:
// -g            => flag_g
// --group       => flag_group
// --group <arg> => flag_group
// FILE          => arg_FILE
// <file>        => arg_file

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
    println!("{:?}", args);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
