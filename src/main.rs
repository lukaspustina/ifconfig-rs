#![feature(plugin)]
#![plugin(rocket_codegen)]
#[allow(unknown_lints)] // for clippy
#[allow(needless_pass_by_value)] // params are passed by value

extern crate ifconfig_rs;
extern crate rocket;
extern crate rocket_contrib;

use ifconfig_rs::backend::*;
use ifconfig_rs::fairings::*;
use ifconfig_rs::guards::*;
use ifconfig_rs::handlers;
use rocket_contrib::{Json, Value as JsonValue};
use rocket::{Request, State};
use rocket::response::Redirect;
use rocket_contrib::Template;

#[get("/", format = "application/json", rank = 1)]
fn index_json(req_info: RequesterInfo, user_agent_parser: State<UserAgentParser>, geoip_city_db: State<GeoIpCityDb>, geoip_asn_db: State<GeoIpAsnDb>) -> Option<Json<JsonValue>> {
    handlers::index_json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/", rank = 2)]
fn index_html(req_info: RequesterInfo, user_agent_parser: State<UserAgentParser>, geoip_city_db: State<GeoIpCityDb>, geoip_asn_db: State<GeoIpAsnDb>) -> Template {
    handlers::index_html(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/ip", format = "application/json", rank = 1)]
fn ip_json(req_info: RequesterInfo, user_agent_parser: State<UserAgentParser>, geoip_city_db: State<GeoIpCityDb>, geoip_asn_db: State<GeoIpAsnDb>) -> Option<Json<JsonValue>> {
    handlers::ip_json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/tcp", format = "application/json", rank = 1)]
fn tcp_json(req_info: RequesterInfo, user_agent_parser: State<UserAgentParser>, geoip_city_db: State<GeoIpCityDb>, geoip_asn_db: State<GeoIpAsnDb>) -> Option<Json<JsonValue>> {
    handlers::tcp_json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}


#[get("/host", format = "application/json", rank = 1)]
fn host_json(req_info: RequesterInfo, user_agent_parser: State<UserAgentParser>, geoip_city_db: State<GeoIpCityDb>, geoip_asn_db: State<GeoIpAsnDb>) -> Option<Json<JsonValue>> {
    handlers::host_json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/location", format = "application/json", rank = 1)]
fn location_json(req_info: RequesterInfo, user_agent_parser: State<UserAgentParser>, geoip_city_db: State<GeoIpCityDb>, geoip_asn_db: State<GeoIpAsnDb>) -> Option<Json<JsonValue>> {
    handlers::location_json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/isp", format = "application/json", rank = 1)]
fn isp_json(req_info: RequesterInfo, user_agent_parser: State<UserAgentParser>, geoip_city_db: State<GeoIpCityDb>, geoip_asn_db: State<GeoIpAsnDb>) -> Option<Json<JsonValue>> {
    handlers::isp_json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/user_agent", format = "application/json", rank = 1)]
fn user_agent_json(req_info: RequesterInfo, user_agent_parser: State<UserAgentParser>, geoip_city_db: State<GeoIpCityDb>, geoip_asn_db: State<GeoIpAsnDb>) -> Option<Json<JsonValue>> {
    handlers::user_agent_json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[error(404)]
fn not_found(_: &Request) -> Redirect {
    Redirect::to("/")
}

fn init_user_agent_parser() -> UserAgentParser { UserAgentParser::new() }

fn init_geoip_city_db(db: &str) -> GeoIpCityDb { GeoIpCityDb::new(db).expect("Failed to load GeoIP City DB") }

fn init_geoip_asn_db(db: &str) -> GeoIpAsnDb { GeoIpAsnDb::new(db).expect("Failed to load GeoIP ASN DB") }

fn main() {
    let mut rocket = rocket::ignite()
        .catch(errors![not_found])
        .mount("/", routes![index_html, index_json, ip_json, tcp_json, host_json, location_json, isp_json, user_agent_json])
        .attach(Template::fairing())
        .manage(init_user_agent_parser());

    rocket = match rocket.config().get_str("runtime_environment") {
        Ok("heroku") => rocket.attach(HerokuForwardedFor::default()),
        _ => rocket,
    };

    rocket = match rocket.config().get_str("geoip_city_db").map(|s| s.to_string()) {
        Ok(db) => rocket.manage(init_geoip_city_db(&db)),
        _ => rocket,
    };

    rocket = match rocket.config().get_str("geoip_asn_db").map(|s| s.to_string()) {
        Ok(db) => rocket.manage(init_geoip_asn_db(&db)),
        _ => rocket,
    };

    rocket.launch();
}
