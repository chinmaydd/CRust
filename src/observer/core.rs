use git2::{Repository, Revspec, Remote, Error};
use git2::build::{RepoBuilder};

use dispatcher::core::GenericDispatcher;

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
pub trait GenericObserver<L> where L: GenericDispatcher {
    // An error type for reporting commonly occuring errors
    type ObserverErrorType;
    // Result type for the Observer
    type ObserverResultType;
    // NOTE: We are assuming absolutely no interaction between the observer and the 
    // runner. This is a fair assumption since the observer will only report the results to
    // the dispatcher and nothing else.
    
    fn new() -> Self;
    fn observe<T: AsRef<str>>(&mut self, resource: T) -> Result<(), Self::ObserverErrorType>;
    fn forget<T: AsRef<str>>(&mut self, resource: T) -> Result<(), Self::ObserverErrorType>;
    fn configure_frequency(&mut self, freq: i32) -> Result<(), Self::ObserverErrorType>;

    // Main function
    fn run(&mut self) -> Result<(), Self::ObserverErrorType>;

    // Dispatcher interaction
    fn attach_dispatcher(&mut self, dispatcher_instance: L) -> Result<(), Self::ObserverErrorType>;    
}

/// Observer struct.
pub struct Observer {
    resource_paths: Vec<Path>,
    frequency: i32,
}

/// Error type for reporting Observer errors.
#[derive(Debug, PartialEq)]
pub enum ObsError {
    AlreadyObserving,
    NotObserving,
    GitError,
    FrequencyError,
}

pub enum ObsCommand {
    Dispatch,
}

// TODO: Have a listener module for the entire architecture which will allow us to add new
// repositories dynamically!
//
// Should all the three modules listen for JSON formatted messages? Will that make things easier?
// Maybe have an interactive console as well? That would be sick!
// Need a design!

/// Result type for Observer
#[derive(Debug)]
pub struct ObsResult {
    // What operation does the Dispatcher have to perform?
    cmd: ObsCommand,
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
impl<L> GenericObserver<L> for Observer where L: GenericDispatcher {
    type ObserverErrorType = ObsError;
    type ObserverResultType = ObsResult;

    // Assuming default frequency is 5 seconds.
    fn new() -> Self {
        Observer {
            resource_paths: Vec::new(),
            frequency: 5,
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

    fn configure_frequency(&mut self, f: i32) -> Result<(), ObsError> {
        if f > 0 {
            self.frequency = f;
            Ok(())
        } else {
            Err(ObsError::FrequencyError)
        }
    }
    
    // Open a set of repos beforehand and then run the loop so that we dont open the repo again and
    // again. 
    fn run(&mut self) -> Result<(), ObsError> {
        info!("Observer module starting up!");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // These tests should contain only those which test the individual operations of an observer.
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn check_git2_dynamic_hash_change() {
        let a = match Repository::open("/home/chinmay_dd/Projects/wtf") {
            Ok(repo) => repo,
            Err(e) => panic!("WTF"),
        };

        loop {
            thread::sleep(Duration::from_secs(5));
            println!("{}", a.revparse_single("HEAD").ok().unwrap().as_commit().unwrap().id());
        }
    }


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
