extern crate git2;

use self::git2::Repository;
use self::git2::Revspec;
use self::git2::Object;

// Function to test how calling function from another file works
pub fn test_function() {
    println!("Calling function from another file!");
}

pub fn observe(directory: String, interval: i32) {
    println!("The directory name is: {} and the interval is {}.", directory, interval);

    // Now that we have the variables we need, we need to write code to look into teh .git folder
    // of the directory and check for the commit ID of the latest commit.
    let repo = match Repository::open("/home/chinmay_dd/Projects/Code") {
        Ok(repo) => repo,
        Err(e)  => panic!("Failed to open: {}", e),
    };

    let commit_val = repo.revparse("HEAD")
                          .unwrap();
    let object = match Revspec::from(&commit_val) {
        Some(object) => object,
        None => panic!("Failed to load commit object."),
    };
    let object_id = Object::id(&object);
    println!("{}", object_id);
}

