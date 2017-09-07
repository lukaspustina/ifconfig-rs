#![allow(unknown_lints)] // for clippy
#![allow(needless_pass_by_value)] // params are passed by value

use backend::*;
use guards::*;
use handlers;
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
