#![allow(unknown_lints)] // for clippy
#![allow(needless_pass_by_value)] // params are passed by value

use backend::*;
use guards::*;
use rocket::State;
use rocket_contrib::{Json, Value as JsonValue};
use rocket_contrib::Template;
use serde_json;

static PGK_NAME: &'static str = "ifconfig.rs";
static PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
static BASE_URL: &'static str = "http://ifconfig.rs";

pub fn index_json(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>)
    -> Option<Json<JsonValue>> {
    let ifconfig_param = IfconfigParam {
        remote: &req_info.remote,
        user_agent_header: &req_info.user_agent,
        user_agent_parser: &user_agent_parser,
        geoip_city_db: &geoip_city_db,
        geoip_asn_db: &geoip_asn_db
    };
    let ifconfig = get_ifconfig(&ifconfig_param);

    // 'Json(ifconfig)' requires a lifetime in the return value, which we cannot supply. Therefore, we serialize manually
    match serde_json::to_value(ifconfig) {
        Ok(json) => Some(Json(json)),
        Err(_) => None
    }
}

pub fn index_html(req_info: RequesterInfo, user_agent_parser: State<UserAgentParser>, geoip_city_db: State<GeoIpCityDb>, geoip_asn_db: State<GeoIpAsnDb>) -> Template {
    let ifconfig_param = IfconfigParam {
        remote: &req_info.remote,
        user_agent_header: &req_info.user_agent,
        user_agent_parser: &user_agent_parser,
        geoip_city_db: &geoip_city_db,
        geoip_asn_db: &geoip_asn_db
    };
    let ifconfig = get_ifconfig(&ifconfig_param);

    #[derive(Serialize)]
    struct Context<'a> {
        ifconfig: Ifconfig<'a>,
        pkg_name: &'a str,
        version: &'a str,
        base_url: &'a str,
        uri: &'a str,
    }

    let context = Context {
        ifconfig,
        pkg_name: PGK_NAME,
        version: PKG_VERSION,
        base_url: BASE_URL,
        uri: req_info.uri,
    };
    Template::render("index", &context)
}

pub fn ip_json(req_info: RequesterInfo, user_agent_parser: State<UserAgentParser>, geoip_city_db: State<GeoIpCityDb>, geoip_asn_db: State<GeoIpAsnDb>) -> Option<Json<JsonValue>> {
    let ifconfig_param = IfconfigParam {
        remote: &req_info.remote,
        user_agent_header: &req_info.user_agent,
        user_agent_parser: &user_agent_parser,
        geoip_city_db: &geoip_city_db,
        geoip_asn_db: &geoip_asn_db
    };
    let ifconfig = get_ifconfig(&ifconfig_param);

    // 'Json(ifconfig)' requires a lifetime in the return value, which we cannot supply. Therefore, we serialize manually
    match serde_json::to_value(ifconfig.ip) {
        Ok(json) => Some(Json(json)),
        Err(_) => None
    }
}

pub fn tcp_json(req_info: RequesterInfo, user_agent_parser: State<UserAgentParser>, geoip_city_db: State<GeoIpCityDb>, geoip_asn_db: State<GeoIpAsnDb>) -> Option<Json<JsonValue>> {
    let ifconfig_param = IfconfigParam {
        remote: &req_info.remote,
        user_agent_header: &req_info.user_agent,
        user_agent_parser: &user_agent_parser,
        geoip_city_db: &geoip_city_db,
        geoip_asn_db: &geoip_asn_db
    };
    let ifconfig = get_ifconfig(&ifconfig_param);

    // 'Json(ifconfig)' requires a lifetime in the return value, which we cannot supply. Therefore, we serialize manually
    match serde_json::to_value(ifconfig.tcp) {
        Ok(json) => Some(Json(json)),
        Err(_) => None
    }
}

pub fn host_json(req_info: RequesterInfo, user_agent_parser: State<UserAgentParser>, geoip_city_db: State<GeoIpCityDb>, geoip_asn_db: State<GeoIpAsnDb>) -> Option<Json<JsonValue>> {
    let ifconfig_param = IfconfigParam {
        remote: &req_info.remote,
        user_agent_header: &req_info.user_agent,
        user_agent_parser: &user_agent_parser,
        geoip_city_db: &geoip_city_db,
        geoip_asn_db: &geoip_asn_db
    };
    let ifconfig = get_ifconfig(&ifconfig_param);

    // 'Json(ifconfig)' requires a lifetime in the return value, which we cannot supply. Therefore, we serialize manually
    match serde_json::to_value(ifconfig.host) {
        Ok(json) => Some(Json(json)),
        Err(_) => None
    }
}

pub fn location_json(req_info: RequesterInfo, user_agent_parser: State<UserAgentParser>, geoip_city_db: State<GeoIpCityDb>, geoip_asn_db: State<GeoIpAsnDb>) -> Option<Json<JsonValue>> {
    let ifconfig_param = IfconfigParam {
        remote: &req_info.remote,
        user_agent_header: &req_info.user_agent,
        user_agent_parser: &user_agent_parser,
        geoip_city_db: &geoip_city_db,
        geoip_asn_db: &geoip_asn_db
    };
    let ifconfig = get_ifconfig(&ifconfig_param);

    // 'Json(ifconfig)' requires a lifetime in the return value, which we cannot supply. Therefore, we serialize manually
    match serde_json::to_value(ifconfig.location) {
        Ok(json) => Some(Json(json)),
        Err(_) => None
    }
}

pub fn isp_json(req_info: RequesterInfo, user_agent_parser: State<UserAgentParser>, geoip_city_db: State<GeoIpCityDb>, geoip_asn_db: State<GeoIpAsnDb>) -> Option<Json<JsonValue>> {
    let ifconfig_param = IfconfigParam {
        remote: &req_info.remote,
        user_agent_header: &req_info.user_agent,
        user_agent_parser: &user_agent_parser,
        geoip_city_db: &geoip_city_db,
        geoip_asn_db: &geoip_asn_db
    };
    let ifconfig = get_ifconfig(&ifconfig_param);

    // 'Json(ifconfig)' requires a lifetime in the return value, which we cannot supply. Therefore, we serialize manually
    match serde_json::to_value(ifconfig.isp) {
        Ok(json) => Some(Json(json)),
        Err(_) => None
    }
}

pub fn user_agent_json(req_info: RequesterInfo, user_agent_parser: State<UserAgentParser>, geoip_city_db: State<GeoIpCityDb>, geoip_asn_db: State<GeoIpAsnDb>) -> Option<Json<JsonValue>> {
    let ifconfig_param = IfconfigParam {
        remote: &req_info.remote,
        user_agent_header: &req_info.user_agent,
        user_agent_parser: &user_agent_parser,
        geoip_city_db: &geoip_city_db,
        geoip_asn_db: &geoip_asn_db
    };
    let ifconfig = get_ifconfig(&ifconfig_param);

    // 'Json(ifconfig)' requires a lifetime in the return value, which we cannot supply. Therefore, we serialize manually
    match serde_json::to_value(ifconfig.user_agent) {
        Ok(json) => Some(Json(json)),
        Err(_) => None
    }
}

