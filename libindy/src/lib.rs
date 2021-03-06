extern crate base64;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

extern crate openssl;

// Note that to use macroses from util inside of other modules it must me loaded first!
#[allow(dead_code)] /* FIXME */
#[macro_use]
mod utils;

pub mod api;
mod commands;
#[allow(dead_code)] /* FIXME */
#[allow(unused_variables)] /* FIXME */
mod errors;
#[allow(dead_code)] /* FIXME */
#[allow(unused_variables)] /* FIXME */
mod services;

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn dummy() {
        assert!(true, "Dummy check!");
    }
}
