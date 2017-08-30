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
}

impl<'a, 'r> FromRequest<'a, 'r> for RequesterInfo {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match req.remote() {
            Some(socket) => Outcome::Success(RequesterInfo{ remote: socket.clone() } ),
            None => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

#[derive(Serialize)]
struct TemplateContext {
    ip: String,
}

#[get("/")]
fn index(req_info: RequesterInfo) -> Template {
    let ip = format!("{}", req_info.remote.ip());
    let context = TemplateContext { ip: ip };
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}

