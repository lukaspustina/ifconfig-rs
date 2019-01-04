extern crate ifconfig_rs;
extern crate rocket;

use rocket::local::Client;
use rocket::http::{Accept, ContentType, Status};
use rocket::http::hyper::header::UserAgent;

#[test]
fn handle_root_plain_cli() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/")
        .remote("192.168.0.101:8000".parse().unwrap())
        .header(Accept::Any)
        .header(UserAgent("curl/7.54.0".into()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("192.168.0.101\n".into()));
}

#[test]
fn handle_root_plain() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/")
        .remote("192.168.0.101:8000".parse().unwrap())
        .header(Accept::Plain)
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("192.168.0.101\n".into()));
}

#[test]
fn handle_root_json() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/")
        .remote("192.168.0.101:8000".parse().unwrap())
        .header(Accept::JSON)
        .header(UserAgent("Some browser that will ultimately win the war.".into()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let expected = r#"{"host":{"name":"192.168.0.101"},"ip":{"addr":"192.168.0.101","version":"4"},"isp":null,"location":null,"tcp":{"port":8000},"user_agent":null,"user_agent_header":"Some browser that will ultimately win the war."}"#;
    assert_eq!(response.body_string(), Some(expected.into()));
}

#[test]
fn handle_root_html() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/")
        .remote("192.168.0.101:8000".parse().unwrap())
        .header(Accept::HTML)
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    assert!(response.body_string().unwrap().contains("html"));
}

#[test]
fn handle_root_json_json() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/json")
        .remote("192.168.0.101:8000".parse().unwrap())
        .header(UserAgent("Some browser that will ultimately win the war.".into()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let expected = r#"{"host":{"name":"192.168.0.101"},"ip":{"addr":"192.168.0.101","version":"4"},"isp":null,"location":null,"tcp":{"port":8000},"user_agent":null,"user_agent_header":"Some browser that will ultimately win the war."}"#;
    assert_eq!(response.body_string(), Some(expected.into()));
}

#[test]
fn handle_ip_plain_cli() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/ip")
        .remote("192.168.0.101:8000".parse().unwrap())
        .header(Accept::Any)
        .header(UserAgent("curl/7.54.0".into()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("192.168.0.101\n".into()));
}

#[test]
fn handle_ip_plain() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/ip")
        .remote("192.168.0.101:8000".parse().unwrap())
        .header(Accept::Plain)
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("192.168.0.101\n".into()));
}

#[test]
fn handle_ip_json() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/ip")
        .remote("192.168.0.101:8000".parse().unwrap())
        .header(Accept::JSON)
        .header(UserAgent("Some browser that will ultimately win the war.".into()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let expected = r#"{"addr":"192.168.0.101","version":"4"}"#;
    assert_eq!(response.body_string(), Some(expected.into()));
}

#[test]
fn handle_ip_json_json() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/ip/json")
        .remote("192.168.0.101:8000".parse().unwrap())
        .header(UserAgent("Some browser that will ultimately win the war.".into()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let expected = r#"{"addr":"192.168.0.101","version":"4"}"#;
    assert_eq!(response.body_string(), Some(expected.into()));
}

#[test]
fn handle_tcp_plain_cli() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/tcp")
        .remote("192.168.0.101:8000".parse().unwrap())
        .header(Accept::Any)
        .header(UserAgent("curl/7.54.0".into()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("8000\n".into()));
}

#[test]
fn handle_tcp_plain() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/tcp")
        .remote("192.168.0.101:8000".parse().unwrap())
        .header(Accept::Plain)
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("8000\n".into()));
}

#[test]
fn handle_tcp_json() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/tcp")
        .remote("192.168.0.101:8000".parse().unwrap())
        .header(Accept::JSON)
        .header(UserAgent("Some browser that will ultimately win the war.".into()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let expected = r#"{"port":8000}"#;
    assert_eq!(response.body_string(), Some(expected.into()));
}

#[test]
fn handle_tcp_json_json() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/tcp/json")
        .remote("192.168.0.101:8000".parse().unwrap())
        .header(UserAgent("Some browser that will ultimately win the war.".into()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let expected = r#"{"port":8000}"#;
    assert_eq!(response.body_string(), Some(expected.into()));
}

#[test]
fn handle_host_plain_cli_curl() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/host")
        .remote("8.8.8.8:8000".parse().unwrap())
        .header(Accept::Any)
        .header(UserAgent("curl/7.54.0".into()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("google-public-dns-a.google.com\n".into()));
}

#[test]
fn handle_host_plain_cli_httpie() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/host")
        .remote("8.8.8.8:8000".parse().unwrap())
        .header(Accept::Any)
        .header(UserAgent("HTTPie/0.9.9".into()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("google-public-dns-a.google.com\n".into()));
}


#[test]
fn handle_host_plain_cli_wget() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/host")
        .remote("8.8.8.8:8000".parse().unwrap())
        .header(Accept::Any)
        .header(UserAgent("Wget/1.19.5 (darwin17.5.0)".into()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("google-public-dns-a.google.com\n".into()));
}

#[test]
fn handle_host_plain() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/host")
        .remote("8.8.8.8:8000".parse().unwrap())
        .header(Accept::Plain)
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("google-public-dns-a.google.com\n".into()));
}

#[test]
fn handle_host_json() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/host")
        .remote("8.8.8.8:8000".parse().unwrap())
        .header(Accept::JSON)
        .header(UserAgent("Some browser that will ultimately win the war.".into()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let expected = r#"{"name":"google-public-dns-a.google.com"}"#;
    assert_eq!(response.body_string(), Some(expected.into()));
}

#[test]
fn handle_host_json_json() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/host/json")
        .remote("8.8.8.8:8000".parse().unwrap())
        .header(UserAgent("Some browser that will ultimately win the war.".into()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let expected = r#"{"name":"google-public-dns-a.google.com"}"#;
    assert_eq!(response.body_string(), Some(expected.into()));
}

#[test]
fn handle_isp_plain_cli() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/isp")
        .remote("8.8.8.8:8000".parse().unwrap())
        .header(Accept::Any)
        .header(UserAgent("curl/7.54.0".into()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("Google LLC\n".into()));
}

#[test]
fn handle_isp_plain() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/isp")
        .remote("8.8.8.8:8000".parse().unwrap())
        .header(Accept::Plain)
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("Google LLC\n".into()));
}

#[test]
fn handle_isp_json() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/isp")
        .remote("8.8.8.8:8000".parse().unwrap())
        .header(Accept::JSON)
        .header(UserAgent("Some browser that will ultimately win the war.".into()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let expected = r#"{"name":"Google LLC"}"#;
    assert_eq!(response.body_string(), Some(expected.into()));
}

#[test]
fn handle_isp_json_json() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/isp/json")
        .remote("8.8.8.8:8000".parse().unwrap())
        .header(UserAgent("Some browser that will ultimately win the war.".into()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let expected = r#"{"name":"Google LLC"}"#;
    assert_eq!(response.body_string(), Some(expected.into()));
}

#[test]
fn handle_location_plain_cli() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/location")
        .remote("93.184.216.34:8000".parse().unwrap())
        .header(Accept::Any)
        .header(UserAgent("curl/7.54.0".into()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("Norwell, United States\n".into()));
}

#[test]
fn handle_location_plain() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/location")
        .remote("93.184.216.34:8000".parse().unwrap())
        .header(Accept::Plain)
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("Norwell, United States\n".into()));
}

#[test]
fn handle_location_json() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/location")
        .remote("93.184.216.34:8000".parse().unwrap())
        .header(Accept::JSON)
        .header(UserAgent("Some browser that will ultimately win the war.".into()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let expected = r#"{"city":"Norwell","country":"United States","latitude":42.1596,"longitude":-70.8217}"#;
    assert_eq!(response.body_string(), Some(expected.into()));
}

#[test]
fn handle_location_json_json() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/location/json")
        .remote("93.184.216.34:8000".parse().unwrap())
        .header(UserAgent("Some browser that will ultimately win the war.".into()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let expected = r#"{"city":"Norwell","country":"United States","latitude":42.1596,"longitude":-70.8217}"#;
    assert_eq!(response.body_string(), Some(expected.into()));
}

#[test]
fn handle_user_agent_plain_cli() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/user_agent")
        .remote("192.168.0.101:8000".parse().unwrap())
        .header(Accept::Any)
        .header(UserAgent("Wget/1.19.5 (darwin17.5.0)".to_string()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("HTTP Library, wget, UNKNOWN, UNKNOWN\n".into()));
}

#[test]
fn handle_user_agent_plain() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/user_agent")
        .remote("192.168.0.101:8000".parse().unwrap())
        .header(Accept::Plain)
        .header(UserAgent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_6) AppleWebKit/603.3.8 (KHTML, like Gecko) Version/10.1.2 Safari/603.3.8".to_string()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("Safari, 10.1.2, Mac OSX, 10.12.6\n".into()));
}

#[test]
fn handle_user_agent_json() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/user_agent")
        .remote("192.168.0.101:8000".parse().unwrap())
        .header(Accept::JSON)
        .header(UserAgent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_6) AppleWebKit/603.3.8 (KHTML, like Gecko) Version/10.1.2 Safari/603.3.8".to_string()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let expected = r#"{"browser_type":"browser","category":"pc","name":"Safari","os":"Mac OSX","os_version":"10.12.6","vendor":"Apple","version":"10.1.2"}"#;
    assert_eq!(response.body_string(), Some(expected.into()));
}

#[test]
fn handle_user_agent_json_json() {
    let client = Client::new(ifconfig_rs::rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/user_agent/json")
        .remote("192.168.0.101:8000".parse().unwrap())
        .header(UserAgent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_6) AppleWebKit/603.3.8 (KHTML, like Gecko) Version/10.1.2 Safari/603.3.8".to_string()))
        .dispatch();
    eprintln!("{:?}", response);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let expected = r#"{"browser_type":"browser","category":"pc","name":"Safari","os":"Mac OSX","os_version":"10.12.6","vendor":"Apple","version":"10.1.2"}"#;
    assert_eq!(response.body_string(), Some(expected.into()));
}
