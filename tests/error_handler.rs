extern crate ifconfig_rs;
extern crate rocket;

use rocket::local::blocking::Client;
use rocket::http::Status;

#[test]
fn handle_error_not_found() {
    let client = Client::tracked(ifconfig_rs::rocket()).expect("valid rocket instance");
    let response = client.get("/does_not_exist").dispatch();
    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.into_string(), Some("not implemented".into()));
}
