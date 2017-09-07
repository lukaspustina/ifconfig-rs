#![feature(plugin)]
#![plugin(rocket_codegen)]

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
pub mod routes;

use backend::*;
use fairings::*;
use routes::*;

use rocket::Rocket;
use rocket_contrib::Template;

static PROJECT_NAME: &'static str = env!("CARGO_PKG_NAME");
static PROJECT_VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Serialize)]
pub struct ProjectInfo {
    name: String,
    version: String,
    base_url: String,
}

pub fn rocket() -> Rocket {
    let mut rocket = rocket::ignite()
        .catch(errors![not_found])
        .mount(
            "/",
            routes![
                index_plain_cli,
                index_json,
                index_plain,
                index_html,
                index_json_json,
                ip_plain_cli,
                ip_json,
                ip_plain,
                ip_json_json,
                tcp_plain_cli,
                tcp_json,
                tcp_plain,
                tcp_json_json,
                host_plain_cli,
                host_json,
                host_plain,
                host_json_json,
                location_plain_cli,
                location_json,
                location_plain,
                location_json_json,
                isp_plain_cli,
                isp_json,
                isp_plain,
                isp_json_json,
                user_agent_plain_cli,
                user_agent_json,
                user_agent_plain,
                user_agent_json_json
            ],
        )
        .attach(Template::fairing())
        .manage(init_user_agent_parser());

    rocket = match rocket.config().get_str("runtime_environment") {
        Ok("heroku") => rocket.attach(HerokuForwardedFor::default()),
        _ => rocket,
    };


    let project_info = ProjectInfo {
        name: rocket.config().get_str("project_name").unwrap_or(PROJECT_NAME ).to_string(),
        version: PROJECT_VERSION.to_string(),
        base_url: rocket.config().get_str("project_base_url").expect("config setting base URL").to_string(),
    };
    rocket = rocket.manage(project_info);

    rocket = match rocket
        .config()
        .get_str("geoip_city_db")
        .map(|s| s.to_string())
        {
            Ok(db) => rocket.manage(init_geoip_city_db(&db)),
            _ => rocket,
        };

    rocket = match rocket
        .config()
        .get_str("geoip_asn_db")
        .map(|s| s.to_string())
        {
            Ok(db) => rocket.manage(init_geoip_asn_db(&db)),
            _ => rocket,
        };

    rocket
}

fn init_user_agent_parser() -> UserAgentParser {
    UserAgentParser::new()
}

fn init_geoip_city_db(db: &str) -> GeoIpCityDb {
    GeoIpCityDb::new(db).expect("Failed to load GeoIP City DB")
}

fn init_geoip_asn_db(db: &str) -> GeoIpAsnDb {
    GeoIpAsnDb::new(db).expect("Failed to load GeoIP ASN DB")
}
