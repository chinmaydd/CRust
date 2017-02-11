// Features intended:
// Observe (some) specific repos (maybe have each thread monitor one)
// Check for change in commit ID of HEAD -> Report results to someone
// If the dispatched module is attached, notify to dispatcher to run said tests
// Maybe have certain tags in the commit message to check for no tests?
// completely configurable in terms of functionality
// Should be implemented as a trait
// Async requests
// Maybe have a listener module to send in special requests
// Have the observer dispatch commits to a later commit?
// Run dispatchers for all new commits if the time interval involves multiple non-tested commits?

// Ideally it should handle any resource placed anywhere.
#[derive(Debug, PartialEq)]
pub enum Path {
    WebResource(String),
    Local(String),
}

impl<T> From<T> for Path where T: AsRef<str> {
    fn from(path: T) -> Path {
        let path_str = path.as_ref();
        if path_str.contains("www") {
            Path::WebResource(path_str.to_owned())
        } else {
            Path::Local(path_str.to_owned())
        }
    }
}

pub struct Observer {
    resource_paths: Vec<Path>, 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_web_resource_for_path() {
        let a: Path = Path::from("www.google.com".to_owned());
        assert_eq!(a, Path::WebResource("www.google.com".to_owned()))
    }

    #[test]
    fn check_local_path() {
        let a: Path = Path::from("/home/chinmay_dd/defragger.txt".to_owned());
        assert_eq!(a, Path::Local("/home/chinmay_dd/defragger.txt".to_owned()))
    }
}
