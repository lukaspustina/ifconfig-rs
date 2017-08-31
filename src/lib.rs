extern crate dns_lookup;
#[macro_use] extern crate serde_derive;
extern crate woothee;

use std::net::SocketAddr;

pub type UserAgentParser = woothee::parser::Parser;
pub type UserAgentParserResult<'a> = woothee::parser::WootheeResult<'a>;

#[derive(Serialize)]
pub struct Ip<'a> {
    addr: String,
    version: &'a str,
}

#[derive(Serialize)]
pub struct Host {
    name: String,
}


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
    ip: Ip<'a>,
    host: Host,
    user_agent: Option<UserAgent<'a>>,
}

pub fn get_ifconfig<'a>(remote: &'a SocketAddr, user_agent: &Option<&'a str>, user_agent_parser: &'a UserAgentParser) -> Ifconfig<'a> {
    let ip_addr = format!("{}", remote.ip());
    let ip_version = if remote.is_ipv4() { "4" } else { "6" };
    let ip = Ip { addr: ip_addr, version: ip_version };

    let hostname = dns_lookup::lookup_addr(&remote.ip()).expect("not found");
    let host = Host { name: hostname };

    let user_agent = user_agent
        .and_then(|s| user_agent_parser.parse(s))
        .map(|res| res.into());

    Ifconfig { ip, host, user_agent }
}

