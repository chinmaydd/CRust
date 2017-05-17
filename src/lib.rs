extern crate git2;
#[macro_use] extern crate log;

pub mod dispatcher;
pub mod observer;
pub mod runner;

#[cfg(test)]
mod tests {
    // Tests here should test integration between Observer <-> Dispatcher <-> Runner
}
