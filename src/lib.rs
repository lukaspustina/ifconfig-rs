extern crate dns_lookup;
#[macro_use]
extern crate lazy_static;
extern crate maxminddb;
extern crate regex;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate woothee;

pub mod backend;
pub mod fairings;
pub mod guards;
pub mod handlers;
