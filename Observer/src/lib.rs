extern crate git2;

use self::git2::{Repository, Revspec, Object, Oid};
use std::env;

// Function to test how calling function from another file works
// pub fn test_function() {
//    println!("Calling function from another file!");
// }

pub fn configure(directory: String, interval: i32) {
    let temp_directory = "/home/chinmay_dd/Projects/Code";
    
    // Assuming for research purposes that the directory is constant:
    // "/home/chinmay_dd/Projects/Code
    let key_dir = "dir";
    let key_int = "int";
    let key_comm = "comm_id";

    let int_string: String = interval.to_string();

    // Set the following environment variables.
    // Directory
    // Interval
    // Intial commit ID.
    env::set_var(key_dir, temp_directory);
    env::set_var(key_int, int_string);
    env::set_var(key_comm, "00");
}

// Observe function runs in an infinite loop.
// It is used to report the current commit ID of the object where the 
// HEAD of the repository points to.
// Returns an Oid.
pub fn observe() -> Oid {
    let key_dir = "dir";

    // Get directory string.
    let directory = match env::var(key_dir) {
        Ok(directory) => directory,
        Err(e) => panic!("An error ocurred: {}", e),
    };
    
    // Using the Repository Struct provided in the git2lib,
    // Open the repository.
    let repo = match Repository::open(directory) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open: {}", e),
    };

    // Reverse parsing to get the HEAD of the repo.
   let commit_val = repo.revparse("HEAD")
                        .unwrap();
    
    // Get the commit object.
    let object = match Revspec::from(&commit_val) {
        Some(object) => object,
        None => panic!("Failed to load commit object."),
    };
    
    // Get the SHA1 hash of the commit object.
    let object_id = Object::id(&object);
    return object_id;
}

