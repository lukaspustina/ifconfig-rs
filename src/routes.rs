#![allow(unknown_lints)] // for clippy
#![allow(needless_pass_by_value)] // params are passed by value

use super::ProjectInfo;
use backend::*;
use guards::*;
use handlers;
use rocket_contrib::json::Json;
use rocket::{Request, State};
use rocket::response::NamedFile;
use rocket_contrib::templates::Template;
use serde_json::{Value as JsonValue};
use std::path::{Path, PathBuf};

#[get("/", rank = 1)]
pub(crate) fn root_plain_cli(
    req_info: RequesterInfo,
    _cli_req: CliClientRequest,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<String> {
    handlers::root::plain(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/", format = "text/plain", rank = 2)]
pub(crate) fn root_plain(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<String> {
    handlers::root::plain(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/", format = "application/json", rank = 3)]
pub(crate) fn root_json(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<Json<JsonValue>> {
    handlers::root::json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/", rank = 4)]
pub(crate) fn root_html(
    project_info: State<ProjectInfo>,
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Template {
    handlers::root_html(project_info, req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[get("/json")]
pub(crate) fn root_json_json(
    req_info: RequesterInfo,
    user_agent_parser: State<UserAgentParser>,
    geoip_city_db: State<GeoIpCityDb>,
    geoip_asn_db: State<GeoIpAsnDb>,
) -> Option<Json<JsonValue>> {
    handlers::root::json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
}

#[catch(404)]
pub(crate) fn not_found(_: &Request) -> String {
    "not implemented".to_string()
}

macro_rules! route {
    ($name:ident, $route:tt, $route_json:tt) => {
        pub mod $name {
            use backend::*;
            use guards::*;
            use handlers;
            use rocket::State;
            use rocket_contrib::json::Json;
            use serde_json::{Value as JsonValue};

            #[get($route, rank = 1)]
            pub(crate) fn plain_cli(
                req_info: RequesterInfo,
                _cli_req: CliClientRequest,
                user_agent_parser: State<UserAgentParser>,
                geoip_city_db: State<GeoIpCityDb>,
                geoip_asn_db: State<GeoIpAsnDb>,
            ) -> Option<String> {
                handlers::$name::plain(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
            }

            #[get($route, format = "application/json", rank = 2)]
            pub(crate) fn json(
                req_info: RequesterInfo,
                user_agent_parser: State<UserAgentParser>,
                geoip_city_db: State<GeoIpCityDb>,
                geoip_asn_db: State<GeoIpAsnDb>,
            ) -> Option<Json<JsonValue>> {
                handlers::$name::json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
            }

            #[get($route, rank = 3)]
            pub(crate) fn plain(
                req_info: RequesterInfo,
                user_agent_parser: State<UserAgentParser>,
                geoip_city_db: State<GeoIpCityDb>,
                geoip_asn_db: State<GeoIpAsnDb>,
            ) -> Option<String> {
                handlers::$name::plain(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
            }

            #[get($route_json)]
            pub(crate) fn json_json(
                req_info: RequesterInfo,
                user_agent_parser: State<UserAgentParser>,
                geoip_city_db: State<GeoIpCityDb>,
                geoip_asn_db: State<GeoIpAsnDb>,
            ) -> Option<Json<JsonValue>> {
                handlers::$name::json(req_info, user_agent_parser, geoip_city_db, geoip_asn_db)
            }
        }
    }
}

route!(root, "/", "/json");

route!(ip, "/ip", "/ip/json");

route!(tcp, "/tcp", "/tcp/json");

route!(host, "/host", "/host/json");

route!(isp, "/isp", "/isp/json");

route!(location, "/location", "/location/json");

route!(user_agent, "/user_agent", "/user_agent/json");

#[get("/<file..>", rank = 5)]
pub(crate) fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("htdocs/").join(file)).ok()
}
