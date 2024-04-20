use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request, Response};
use std::net::IpAddr;
use std::net::SocketAddr;
use std::str::FromStr;

#[derive(Default)]
pub struct HerokuForwardedFor;

#[rocket::async_trait]
impl Fairing for HerokuForwardedFor {
    fn info(&self) -> Info {
        Info {
            name: "Set the request remote to Heroku's X-Forwarded-For",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        let new_remote = if let Some(xfr) = request.headers().get_one("X-Forwarded-For") {
            if let Some(remote) = request.remote() {
                if let Ok(ip) = IpAddr::from_str(xfr) {
                    Some(SocketAddr::new(ip, remote.port()))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        if let Some(remote) = new_remote {
            request.set_remote(remote);
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, _response: &mut Response<'r>) {
        return;
    }
}
