#![allow(unknown_lints)] // for clippy
#![allow(needless_pass_by_value)] // params are passed by value

use crate::ProjectInfo;
use crate::backend::*;
use crate::guards::*;
use rocket::State;
use rocket_dyn_templates::Template;

pub fn root_html(
    project_info: &State<ProjectInfo>,
    req_info: RequesterInfo,
    user_agent_parser: &State<UserAgentParser>,
    geoip_city_db: &State<GeoIpCityDb>,
    geoip_asn_db: &State<GeoIpAsnDb>,
) -> Template {
    let ifconfig_param = IfconfigParam {
        remote: &req_info.remote,
        user_agent_header: &req_info.user_agent,
        user_agent_parser: &user_agent_parser,
        geoip_city_db: &geoip_city_db,
        geoip_asn_db: &geoip_asn_db,
    };
    let ifconfig = get_ifconfig(&ifconfig_param);

    #[derive(Serialize)]
    struct Context<'a> {
        ifconfig: Ifconfig<'a>,
        project: &'a ProjectInfo,
        uri: &'a str,
    }

    let context = Context {
        ifconfig,
        project: &project_info,
        uri: req_info.uri.as_ref(),
    };
    Template::render("index", &context)
}

macro_rules! handler {
    ($name:ident, $ifconfig:ident, $json:block, $ty:ty, $plain:block) => {
        pub mod $name {
            use crate::backend::*;
            use crate::guards::*;
            use rocket::State;
            use rocket::serde::json::{Json};
            use serde_json::{Value as JsonValue};

            fn to_json($ifconfig: Ifconfig) -> $ty {
                $json
            }

            pub fn json(
                req_info: RequesterInfo,
                user_agent_parser: &State<UserAgentParser>,
                geoip_city_db: &State<GeoIpCityDb>,
                geoip_asn_db: &State<GeoIpAsnDb>,
            ) -> Option<Json<JsonValue>> {
                let ifconfig_param = IfconfigParam {
                    remote: &req_info.remote,
                    user_agent_header: &req_info.user_agent,
                    user_agent_parser: &user_agent_parser,
                    geoip_city_db: &geoip_city_db,
                    geoip_asn_db: &geoip_asn_db,
                };
                let ifconfig = get_ifconfig(&ifconfig_param);

                let value = to_json(ifconfig);

                // 'Json(ifconfig)' requires a lifetime in the return value, which we cannot supply. Therefore, we serialize manually
                match serde_json::to_value(value) {
                    Ok(json) => Some(Json(json)),
                    Err(_) => None,
                }
            }

            fn to_plain($ifconfig: Ifconfig) -> String {
                $plain
            }

            pub fn plain(
                req_info: RequesterInfo,
                user_agent_parser: &State<UserAgentParser>,
                geoip_city_db: &State<GeoIpCityDb>,
                geoip_asn_db: &State<GeoIpAsnDb>,
            ) -> Option<String> {
                let ifconfig_param = IfconfigParam {
                    remote: &req_info.remote,
                    user_agent_header: &req_info.user_agent,
                    user_agent_parser: &user_agent_parser,
                    geoip_city_db: &geoip_city_db,
                    geoip_asn_db: &geoip_asn_db,
                };
                let ifconfig = get_ifconfig(&ifconfig_param);

                let value = to_plain(ifconfig);

                Some(value)
            }
        }
    }
}

handler!(root, ifconfig, {ifconfig}, Ifconfig, {format!("{}\n", ifconfig.ip.addr)});

handler!(ip, ifconfig, {ifconfig.ip}, Ip, {format!("{}\n", ifconfig.ip.addr)});

handler!(tcp, ifconfig, {ifconfig.tcp}, Tcp, {format!("{}\n", ifconfig.tcp.port)});

handler!(host, ifconfig, {ifconfig.host}, Option<Host>, {format!("{}\n", ifconfig.host.map(|h| h.name).unwrap_or_else(|| "unknown".to_string()))});

handler!(isp, ifconfig, {ifconfig.isp}, Option<Isp>, {format!("{}\n",
    ifconfig.isp.and_then(|isp| isp.name).unwrap_or_else(|| "unknown".to_string())
)});

handler!(location, ifconfig, {ifconfig.location}, Option<Location>, {
    let unknown = "unknown".to_string();
    format!("{}, {}\n",
        ifconfig.location.as_ref().and_then(|l| l.city.as_ref()).unwrap_or_else(|| &unknown),
        ifconfig.location.as_ref().and_then(|l| l.country.as_ref()).unwrap_or_else(|| &unknown)
    )
});

handler!(user_agent, ifconfig, {ifconfig.user_agent}, Option<UserAgent>, {format!("{}\n",
    ifconfig.user_agent.map(|ua| format!("{}, {}, {}, {}", ua.name, ua.version, ua.os, ua.os_version)).unwrap()
)});
