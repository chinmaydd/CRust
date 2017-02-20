extern crate git2;

use self::git2::{Repository, Revspec, Remote, Error};
use self::git2::build::{RepoBuilder};

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
        // FIXME: HACKY AF
        if path_str.contains("www") || path_str.contains("https") {
            Path::WebResource(path_str.to_owned())
        } else {
            Path::Local(path_str.to_owned())
        }
    }
}

/// Generic Observer trait which specifies functions which an Observer should implement
pub trait GenericObserver {
    // An error type for reporting commonly occuring errors
    type ObserverErrorType;
    // Ideally should be a unit of time(?)
    // type frequency;
    // Dispatcher to be attached to this observer
    // type Dispatcher;
    // NOTE: We are assuming absolutely no interaction between the observer and the 
    // runner. This is a fair assumption since the observer will only report the results to
    // the dispatcher and nothing else.

    fn new() -> Self;
    fn observe<T: AsRef<str>>(&mut self, resource: T) -> Result<(), Self::ObserverErrorType>;
    fn forget<T: AsRef<str>>(&mut self, resource: T) -> Result<(), Self::ObserverErrorType>;
    // fn configure_frequency(&mut self, freq: Self::frequency) -> Result<(), Self::ObserverErrorType>;

    // Main function
    // fn run(&mut self) -> Result<(), Self::ObserverErrorType>;

    // Dispatcher interaction
    // fn attach_dispatcher(&mut self, dispatcher_instance: Self::Dispatcher) -> Result<(), Self::ObserverErrorType>;    
}

/// Observer struct.
pub struct Observer {
    resource_paths: Vec<Path>, 
}

/// Error type for reporting Observer errors.
#[derive(Debug, PartialEq)]
pub enum ObsError {
    AlreadyObserving,
    NotObserving,
    GitError,
}

/// Implementing utility functions for implementing GenericObserver trait on our Observer
impl Observer {
    fn already_observing(&self, resource: Path) -> bool {
        if self.resource_paths.contains(&resource) {
            true
        } else {
            false
        }
    }
    
    fn is_git_repository(&self, resource: Path) -> bool {
        match resource {
            Path::WebResource(path) => {
                let mut r_builder = RepoBuilder::new();
                let repo = r_builder.clone(&path, "./tmp".as_ref());
                repo.is_ok()
            },
            Path::Local(path) => {
                match Repository::open(path) {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
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
        } else if self.is_git_repository(res_path.clone()) {
            self.resource_paths.push(res_path);
            Ok(())
        } else {
            Err(ObsError::GitError)
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

    #[test]
    fn check_git_error() {
        let mut a: Observer = Observer::new();
        let b = a.observe("/home/chinmay_dd/deragger.txt".to_owned());
        assert_eq!(b, Err(ObsError::GitError))
    }

    #[test]
    fn check_ok() {
        let mut a: Observer = Observer::new();
        let b = a.observe("/home/chinmay_dd/Projects/Code".to_owned());
        assert_eq!(b, Ok(()))
    }

    #[test]
    fn check_already_observing() {
        let mut a: Observer = Observer::new();
        a.observe("/home/chinmay_dd/Projects/Code".to_owned());
        a.observe("/home/chinmay_dd/Projects/sandpile".to_owned());
        let d = a.observe("/home/chinmay_dd/Projects/Code".to_owned());
        assert_eq!(d, Err(ObsError::AlreadyObserving));
        assert_eq!(a.resource_paths.len(), 2)
    }

    // #[test]
    // Working for now, no need to test again and again
    fn check_correct_web_path() {
        let mut a: Observer = Observer::new();
        let b = a.observe("https://github.com/chinmaydd/Code.git".to_owned());
        assert_eq!(b, Ok(()))
    } 

    #[test]
    fn check_incorrect_web_path() {
        let mut a: Observer = Observer::new();
        let b = a.observe("www.google.com".to_owned());
        assert_eq!(b, Err(ObsError::GitError))
    }
}
