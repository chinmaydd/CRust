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

/// Path of a certain git repository to be observed.
/// Could be a WebResource(URI) or a Local(Path) resource.
#[derive(Debug, PartialEq, Clone)]
pub enum Path {
    WebResource(String),
    Local(String),
}

/// Implemented such that if the URI contains the substring "www",
/// it will be categorized as a WebResource.
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

/// Generic Observer trait which specifies functions which an Observer should implement
pub trait GenericObserver {
    type ObserverErrorType;

    fn new() -> Self;
    fn observe<T: AsRef<str>>(&mut self, resource: T) -> Result<(), Self::ObserverErrorType>;
    fn forget<T: AsRef<str>>(&mut self, resource: T) -> Result<(), Self::ObserverErrorType>;
}

/// Observer struct.
pub struct Observer {
    resource_paths: Vec<Path>, 
}

/// Error type for reporting Observer errors.
pub enum ObsError {
    AlreadyObserving,
    NotObserving,
}

/// Implenting utility functions for implementing GenericObserver trait on our Observer
impl Observer {
    fn already_observing(&self, resource: Path) -> bool {
        if self.resource_paths.contains(&resource) {
            true
        } else {
            false
        }
    }
}

/// Implementing trait GenericObserver for our default Observer
impl GenericObserver for Observer {
    type ObserverErrorType = ObsError;

    fn new() -> Self {
        Observer {
            resource_paths: Vec::new(),
        }
    }

    fn observe<T: AsRef<str>>(&mut self, resource: T) -> Result<(), ObsError> {
        let res_path = Path::from(resource);
        if self.already_observing(res_path.clone()) {
            Err(ObsError::AlreadyObserving)
        } else {
            self.resource_paths.push(res_path);
            Ok(())
        }
    }

    fn forget<T: AsRef<str>>(&mut self, resource: T) -> Result<(), ObsError> {
        let res_path = Path::from(resource);
        if self.already_observing(res_path.clone()) {
            let index = self.resource_paths.iter().position(|ref r| **r == res_path.clone()).unwrap();
            self.resource_paths.remove(index);
            Ok(())
        } else {
            Err(ObsError::NotObserving)
        }
    }
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
