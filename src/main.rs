#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use rocket_contrib::Template;
use std::net::SocketAddr;


struct RequesterInfo {
    remote: SocketAddr,
    user_agent: Option<String>,
}

impl<'a, 'r> FromRequest<'a, 'r> for RequesterInfo {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let remote = if let Some(remote) = req.remote() {
            remote
        } else {
            return Outcome::Failure((Status::BadRequest, ()))
        };
        let user_agent = req.headers().get_one("User-Agent").map(|u| u.to_string());

        let request_info = RequesterInfo { remote: remote, user_agent: user_agent };
        Outcome::Success(request_info)
    }
}

#[derive(Serialize)]
struct TemplateContext {
    ip: String,
    user_agent: Option<String>,
}

#[get("/")]
fn index(req_info: RequesterInfo) -> Template {
    let ip = format!("{}", req_info.remote.ip());
    let context = TemplateContext { ip: ip, user_agent: req_info.user_agent };
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}

