use regex::RegexSet;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest};
use rocket::Request;
use std::net::SocketAddr;

pub struct RequesterInfo<'a> {
    pub remote: SocketAddr,
    pub user_agent: Option<&'a str>,
    pub uri: String,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for RequesterInfo<'a> {
    type Error = ();

    async fn from_request(req: &'a Request<'_>) -> request::Outcome<Self, Self::Error> {
        let remote = if let Some(remote) = req.remote() {
            remote
        } else {
            return Outcome::Error((Status::InternalServerError, ()));
        };
        let user_agent = req.headers().get_one("User-Agent");

        let request_info = RequesterInfo {
            remote,
            user_agent,
            uri: req.uri().to_string(),
        };
        Outcome::Success(request_info)
    }
}

/// `CliClient` checks if a known CLI client sends a "plain" text request
///
/// At least curl, httpie, wget send "Accept: */*" by default. This makes it difficult to dispatch the request. This
/// request guard tries to guess, if a request is plain text request by a CLI client. The the heuristic goes like this:
/// 1. Is this as known CLI client, cf. `KNOWN_CLI_CLIENTS`?
/// 2. If yes, is the default Accept header set, i.e., */* set?
/// 3. If yes, then this is a plain text request by a CLI client
/// 4. In any other case, the request is forwarded to higher ranked routes.
pub struct CliClientRequest<'a> {
    pub user_agent_header: &'a str,
}

lazy_static! {
    static ref KNOWN_CLI_CLIENTS: RegexSet = RegexSet::new([r"curl", r"HTTPie", r"HTTP Library", r"Wget",]).unwrap();
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for CliClientRequest<'a> {
    type Error = ();

    async fn from_request(req: &'a Request<'_>) -> request::Outcome<Self, Self::Error> {
        let user_agent_header = req.headers().get_one("User-Agent");
        let accept_header = req.headers().get_one("Accept");

        match (user_agent_header, accept_header) {
            (Some(uah), Some("*/*")) if KNOWN_CLI_CLIENTS.is_match(uah) => {
                Outcome::Success(CliClientRequest { user_agent_header: uah })
            }
            _ => Outcome::Forward(Status::Ok),
        }
    }
}
