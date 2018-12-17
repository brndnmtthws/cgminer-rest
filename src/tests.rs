use rocket::fairing::AdHoc;
use rocket::local::Client;

#[test]
fn version_fail() {
    let rocket = rocket::ignite()
        .mount("/", routes![super::api::version])
        .register(catchers![super::api::internal_server_error])
        .attach(AdHoc::on_attach("CgminerAddr", |rocket| {
            Ok(rocket.manage(super::api::CgminerConfig {
                addr: "127.0.0.1:4028".to_owned(),
                timeout: 60,
            }))
        }));
    let client = Client::new(rocket).unwrap();
    let mut response = client.get("/version").dispatch();
    assert!(response
        .body_string()
        .unwrap_or_else(|| "invalid".to_string())
        .starts_with("{\"error\":\"IO error: Connection refused"));
}

#[test]
fn version_fail2() {
    let rocket = rocket::ignite()
        .mount("/", routes![super::api::version])
        .register(catchers![super::api::internal_server_error])
        .attach(AdHoc::on_attach("CgminerAddr", |rocket| {
            Ok(rocket.manage(super::api::CgminerConfig {
                addr: "lolol127.0.0.1:4028".to_owned(),
                timeout: 60,
            }))
        }));
    let client = Client::new(rocket).unwrap();
    let mut response = client.get("/version").dispatch();
    assert!(response
        .body_string()
        .unwrap_or_else(|| "invalid".to_string())
        .starts_with("{\"error\":\"IO error: failed to lookup address information: "));
}
