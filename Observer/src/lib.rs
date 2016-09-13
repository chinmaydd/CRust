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

    let int_string: String = interval.to_string();

    env::set_var(key_dir, temp_directory);
    env::set_var(key_int, int_string);
}

pub fn observe() -> Oid {
    let key_dir = "dir";
    let directory = match env::var(key_dir) {
        Ok(directory) => directory,
        Err(e) => panic!("An error ocurred: {}", e),
    };
    
    let repo = match Repository::open(directory) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open: {}", e),
    };

   let commit_val = repo.revparse("HEAD")
                        .unwrap();
    
    let object = match Revspec::from(&commit_val) {
        Some(object) => object,
        None => panic!("Failed to load commit object."),
    };
    
    let object_id = Object::id(&object);
    return object_id;
}

