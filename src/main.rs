#![feature(plugin)]
#![plugin(rocket_codegen)]
#![allow(unknown_lints)] // for clippy
#![allow(needless_pass_by_value)] // params are passed by value

extern crate ifconfig_rs;
extern crate rocket;
extern crate rocket_contrib;

use ifconfig_rs::backend::*;
use ifconfig_rs::fairings::*;
use ifconfig_rs::guards::*;
use ifconfig_rs::handlers;
use rocket_contrib::{Json, Value as JsonValue};
use rocket::{Request, State};
use rocket_contrib::Template;

#[get("/", rank = 1)]
fn index_plain_cli(
    req_info: RequesterInfo,
    _cli_req: CliClientRequest,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<String> {
    handlers::index_plain(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/", format = "application/json", rank = 2)]
fn index_json(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<Json<JsonValue>> {
    handlers::index_json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/", format = "text/plain" , rank = 2)]
fn index_plain(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<String> {
    handlers::index_plain(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/", rank = 3)]
fn index_html(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Template {
    handlers::index_html(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/json")]
fn index_json_json(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<Json<JsonValue>> {
    handlers::index_json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/ip", rank = 1)]
fn ip_plain_cli(
    req_info: RequesterInfo,
    _cli_req: CliClientRequest,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<String> {
    handlers::ip_plain(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/ip", format = "application/json", rank = 2)]
fn ip_json(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<Json<JsonValue>> {
    handlers::ip_json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/ip", rank = 3)]
fn ip_plain(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<String> {
    handlers::ip_plain(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/ip/json")]
fn ip_json_json(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<Json<JsonValue>> {
    handlers::ip_json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/tcp", rank = 1)]
fn tcp_plain_cli(
    req_info: RequesterInfo,
    _cli_req: CliClientRequest,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<String> {
    handlers::tcp_plain(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/tcp", format = "application/json", rank = 2)]
fn tcp_json(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<Json<JsonValue>> {
    handlers::tcp_json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/tcp", rank = 3)]
fn tcp_plain(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<String> {
    handlers::tcp_plain(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/tcp/json")]
fn tcp_json_json(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<Json<JsonValue>> {
    handlers::tcp_json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/host", rank = 1)]
fn host_plain_cli(
    req_info: RequesterInfo,
    _cli_req: CliClientRequest,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<String> {
    handlers::host_plain(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/host", format = "application/json", rank = 2)]
fn host_json(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<Json<JsonValue>> {
    handlers::host_json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/host", rank = 3)]
fn host_plain(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<String> {
    handlers::host_plain(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/host/json")]
fn host_json_json(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<Json<JsonValue>> {
    handlers::host_json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/location", rank = 1)]
fn location_plain_cli(
    req_info: RequesterInfo,
    _cli_req: CliClientRequest,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<String> {
    handlers::location_plain(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/location", format = "application/json", rank = 2)]
fn location_json(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<Json<JsonValue>> {
    handlers::location_json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/location", rank = 3)]
fn location_plain(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<String> {
    handlers::location_plain(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/location/json")]
fn location_json_json(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<Json<JsonValue>> {
    handlers::location_json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/isp", rank = 1)]
fn isp_plain_cli(
    req_info: RequesterInfo,
    _cli_req: CliClientRequest,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<String> {
    handlers::isp_plain(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/isp", format = "application/json", rank = 2)]
fn isp_json(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<Json<JsonValue>> {
    handlers::isp_json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/isp", rank = 3)]
fn isp_plain(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<String> {
    handlers::isp_plain(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/isp/json")]
fn isp_json_json(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<Json<JsonValue>> {
    handlers::isp_json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/user_agent", rank = 1)]
fn user_agent_plain_cli(
    req_info: RequesterInfo,
    _cli_req: CliClientRequest,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<String> {
    handlers::user_agent_plain(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/user_agent", format = "application/json", rank = 2)]
fn user_agent_json(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<Json<JsonValue>> {
    handlers::user_agent_json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}


#[get("/user_agent", rank = 3)]
fn user_agent_plain(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<String> {
    handlers::user_agent_plain(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/user_agent/json")]
fn user_agent_json_json(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<Json<JsonValue>> {
    handlers::user_agent_json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[error(404)]
fn not_found(_: &Request) -> String {
    "not implemented".to_string()
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

fn main() {
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

    rocket.launch();
}
