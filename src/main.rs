#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate woothee;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use rocket_contrib::Template;
use std::net::SocketAddr;


struct RequesterInfo<'a> {
    remote: SocketAddr,
    user_agent: Option<&'a str>,
}

impl<'a, 'r> FromRequest<'a, 'r> for RequesterInfo<'a> {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let remote = if let Some(remote) = req.remote() {
            remote
        } else {
            return Outcome::Failure((Status::BadRequest, ()))
        };
        let user_agent = req.headers().get_one("User-Agent");

        let request_info = RequesterInfo { remote: remote, user_agent: user_agent };
        Outcome::Success(request_info)
    }
}

type UserAgentParser = woothee::parser::Parser;
type UserAgentParserResult<'a> = woothee::parser::WootheeResult<'a>;

fn init_user_agent_parser() -> UserAgentParser { UserAgentParser::new() }

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
struct Ip<'a> {
    addr: String,
    version: &'a str,
}

#[derive(Serialize)]
struct IndexContext<'a> {
    ip: &'a Ip<'a>,
    user_agent: Option<UserAgent<'a>>,
}

#[get("/")]
fn index(req_info: RequesterInfo, user_agent_parser: State<UserAgentParser>) -> Template {
    let ip_addr = format!("{}", req_info.remote.ip());
    let ip_version = if req_info.remote.is_ipv4() { "4" } else { "6" };
    let ip = Ip { addr: ip_addr, version: ip_version };
    let user_agent = req_info.user_agent
        .and_then(|s| user_agent_parser.parse(&s))
        .map(|res| res.into());

    let context = IndexContext { ip: &ip, user_agent: user_agent };
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .manage(init_user_agent_parser())
        .launch();
}
