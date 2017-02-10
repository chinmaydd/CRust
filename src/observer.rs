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

pub struct Observer {
}
