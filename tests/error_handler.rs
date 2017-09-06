extern crate ifconfig_rs;
extern crate rocket;

use ifconfig_rs::rocket;
use rocket::local::Client;
use rocket::http::Status;

#[test]
fn handle_error_not_found() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/does_not_exist").dispatch();
    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.body_string(), Some("not implemented".into()));
}