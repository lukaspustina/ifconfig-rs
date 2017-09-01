use rocket::{Request, Outcome};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use std::net::SocketAddr;

pub struct RequesterInfo<'a> {
    pub remote: SocketAddr,
    pub user_agent: Option<&'a str>,
    pub uri: &'a str,
}

impl<'a, 'r> FromRequest<'a, 'r> for RequesterInfo<'a> {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let remote = if let Some(remote) = req.remote() {
            remote
        } else {
            return Outcome::Failure((Status::BadRequest, ()));
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