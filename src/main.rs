#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate ifconfig_rs;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use ifconfig_rs::*;
use rocket::http::Status;
use rocket_contrib::{Json, Value as JsonValue};
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use rocket_contrib::Template;
use std::net::SocketAddr;

static VERSION: &'static str = env!("CARGO_PKG_VERSION");
static BASE_URL: &'static str = "http://ifconfig.rs";

pub struct RequesterInfo<'a> {
    remote: SocketAddr,
    user_agent: Option<&'a str>,
    uri: &'a str,
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

        let request_info = RequesterInfo {
            remote: remote,
            user_agent: user_agent,
            uri: req.uri().as_str(),
        };
        Outcome::Success(request_info)
    }
}

fn init_user_agent_parser() -> UserAgentParser { UserAgentParser::new() }

#[get("/", format = "application/json", rank=1)]
#[allow(unknown_lints)] // for clippy
#[allow(needless_pass_by_value)] // params are passed by value
fn index_json(req_info: RequesterInfo, user_agent_parser: State<UserAgentParser>) -> Option<Json<JsonValue>> {
    let ifconfig = get_ifconfig(&req_info.remote, &req_info.user_agent, &user_agent_parser);

    // 'Json(ifconfig)' requires a lifetime in the return value, which we cannot supply. Therefore, we serialize manually
    match serde_json::to_value(ifconfig){
        Ok(json) => Some(Json(json)),
        Err(_) => None
    }
}

#[get("/", rank=2)]
#[allow(unknown_lints)] // for clippy
#[allow(needless_pass_by_value)] // params are passed by value
fn index_html(req_info: RequesterInfo, user_agent_parser: State<UserAgentParser>) -> Template {
    let ifconfig = get_ifconfig(&req_info.remote, &req_info.user_agent, &user_agent_parser);

    #[derive(Serialize)]
    struct Context<'a> {
        ifconfig: Ifconfig<'a>,
        version: &'a str,
        base_url: &'a str,
        uri: &'a str,
    }

    let context = Context{
        ifconfig,
        version: VERSION,
        base_url: BASE_URL,
        uri: req_info.uri
    };
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index_html, index_json])
        .attach(Template::fairing())
        .manage(init_user_agent_parser())
        .launch();
}
