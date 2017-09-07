#![allow(unknown_lints)] // for clippy

use regex::RegexSet;
use rocket::{Outcome, Request};
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
            return Outcome::Failure((Status::InternalServerError, ()));
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

/// `CliClient` checks if a known CLI client sends a "plain" text request
///
/// At least curl, httpie, wget send "Accept: */*" by default. This makes it difficult to dispatch the request. This
/// request guard tries to guess, if a request is plain text request by a CLI client. The the heuristic goes like this:
/// 1. Is this as known CLI client, cf. `RE_SET`?
/// 2. If yes, is the default Accept header set, i.e., */* set?
/// 3. If yes, then this is a plain text request by a CLI client
/// 4. In any other case, the request is forwarded to higher ranked routes.
pub struct CliClientRequest<'a> {
    pub user_agent_header: &'a str,
}

#[allow(trivial_regex)]
impl<'a, 'r> FromRequest<'a, 'r> for CliClientRequest<'a> {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        lazy_static! {
            static ref RE_SET: RegexSet = RegexSet::new(&[
                r"HTTPie",
                r"HTTP Library",
            ]).unwrap();
        }

        let user_agent_header = req.headers().get_one("User-Agent");
        let accept_header = req.headers().get_one("Accept");

        match (user_agent_header, accept_header) {
            (Some(uah), Some("*/*")) if !RE_SET.matches(uah).matched(0) => Outcome::Success(CliClientRequest { user_agent_header: uah }),
            _ => Outcome::Forward(())
        }
    }
}
