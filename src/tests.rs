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
    assert_eq!(
        response.body_string(),
        Some("{\"error\":\"IO error: Connection refused (os error 61)\"}".into())
    );
}
