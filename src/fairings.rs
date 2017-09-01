use rocket::{Data, Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use std::str::FromStr;
use std::net::IpAddr;
use std::net::SocketAddr;

#[derive(Default)]
pub struct HerokuForwardedFor;

impl Fairing for HerokuForwardedFor {
    fn info(&self) -> Info {
        Info {
            name: "Set the request remote to Heroku's X-Forwarded-For",
            kind: Kind::Request | Kind::Response,
        }
    }

    fn on_request(&self, request: &mut Request, _: &Data) {
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

    fn on_response(&self, _: &Request, _: &mut Response) {
        return;
    }
}
