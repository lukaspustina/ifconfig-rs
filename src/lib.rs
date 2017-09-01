extern crate dns_lookup;
extern crate maxminddb;
#[macro_use] extern crate serde_derive;
extern crate woothee;

use maxminddb::geoip2;
use std::net::SocketAddr;

#[derive(Serialize)]
pub struct Host {
    name: String,
}

#[derive(Serialize)]
pub struct Ip<'a> {
    addr: String,
    version: &'a str,
}

#[derive(Serialize)]
pub struct Tcp {
    port: u16,
}

pub struct GeoIpCityDb(pub maxminddb::Reader);

impl GeoIpCityDb {
    pub fn new(db_path: &str) -> Option<Self> {
        maxminddb::Reader::open(db_path).ok().map(|reader| GeoIpCityDb(reader))
    }
}

pub struct GeoIpAsnDb(pub maxminddb::Reader);

impl GeoIpAsnDb {
    pub fn new(db_path: &str) -> Option<Self> {
        maxminddb::Reader::open(db_path).ok().map(|reader| GeoIpAsnDb(reader))
    }
}

#[derive(Serialize)]
pub struct Location {
    pub city: Option<String>,
    pub country: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Serialize)]
pub struct Isp {
    pub name: Option<String>,
}

pub type UserAgentParser = woothee::parser::Parser;
pub type UserAgentParserResult<'a> = woothee::parser::WootheeResult<'a>;

#[derive(Serialize)]
pub struct UserAgent<'a> {
    pub name: &'a str,
    pub category: &'a str,
    pub os: &'a str,
    pub os_version: String,
    pub browser_type: &'a str,
    pub version: String,
    pub vendor: &'a str,
}

impl<'a> From<UserAgentParserResult<'a>> for UserAgent<'a> {
    fn from(ua: UserAgentParserResult<'a>) -> Self {
        UserAgent {
            name: ua.name,
            category: ua.category,
            os: ua.os,
            os_version: ua.os_version,
            browser_type: ua.browser_type,
            version: ua.version,
            vendor: ua.vendor,
        }
    }
}

#[derive(Serialize)]
pub struct Ifconfig<'a> {
    host: Host,
    ip: Ip<'a>,
    tcp: Tcp,
    location: Option<Location>,
    isp: Option<Isp>,
    user_agent: Option<UserAgent<'a>>,
    user_agent_header: Option<&'a str>,
}

pub fn get_ifconfig<'a>(remote: &'a SocketAddr, user_agent_header: &Option<&'a str>, user_agent_parser: &'a UserAgentParser, geoip_city_db: &'a GeoIpCityDb, geoip_asn_db: &'a GeoIpAsnDb) -> Ifconfig<'a> {
    let hostname = dns_lookup::lookup_addr(&remote.ip()).expect("not found");
    let host = Host { name: hostname };

    let ip_addr = format!("{}", remote.ip());
    let ip_version = if remote.is_ipv4() { "4" } else { "6" };
    let ip = Ip { addr: ip_addr, version: ip_version };

    let tcp = Tcp { port: remote.port() };

    let geo_city: Option<geoip2::City> = geoip_city_db.0.lookup(remote.ip()).ok();
    let location = geo_city
        .map(|c| Location {
            city: c.city.and_then( |e| e.names).and_then( |mut h| h.remove("en")),
            country: c.country.and_then( |e| e.names).and_then( |mut h| h.remove("en")),
            latitude: c.location.as_ref().and_then(|l| l.latitude),
            longitude: c.location.as_ref().and_then(|l| l.longitude),
        });

    let geo_isp: Option<geoip2::Isp> = geoip_asn_db.0.lookup(remote.ip()).ok();
    let isp = geo_isp.map(|isp| Isp { name: isp.autonomous_system_organization });

    let user_agent = user_agent_header
        .and_then(|s| user_agent_parser.parse(s))
        .map(|res| res.into());

    Ifconfig { host, ip, tcp, location, isp, user_agent, user_agent_header: *user_agent_header }
}
