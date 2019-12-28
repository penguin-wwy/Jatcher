#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate simple_logging;
#[macro_use]
extern crate log;

pub mod logger;
pub mod config;
pub mod runtime;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
