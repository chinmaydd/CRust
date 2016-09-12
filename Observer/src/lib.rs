// Function to test how calling function from another file works
pub fn test_function() {
    println!("Calling function from another file!");
}

pub fn observe(directory: String, interval: i32) {
    println!("The directory name is: {} and the interval is {}.", directory, interval);
}
