extern crate dns_lookup;
extern crate maxminddb;
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