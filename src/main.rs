#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod api;
use rocket::fairing::AdHoc;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                api::version,
                api::config,
                api::summary,
                api::devs,
                api::devdetails,
                api::stats,
                api::coin,
                api::usbstats,
                api::lcd,
                api::notify,
                api::privileged,
                api::restart,
                api::check,
                api::debug,
                api::hotplug,
                api::lockstats,
                api::zero,
            ],
        )
        .mount(
            "/pools",
            routes![
                api::pools,
                api::switchpool,
                api::enablepool,
                api::disablepool,
                api::addpool,
                api::removepool,
                api::poolquota,
            ],
        )
        .mount(
            "/pga",
            routes![
                api::pga,
                api::pgacount,
                api::pgaenable,
                api::pgadisable,
                api::pgaidentify,
                api::pgaset,
            ],
        )
        .mount(
            "/asc",
            routes![
                api::asc,
                api::asccount,
                api::ascenable,
                api::ascdisable,
                api::ascidentify,
                api::ascset,
            ],
        )
        .register(catchers![
            api::not_found,
            api::internal_server_error,
            api::unprocessable_entity
        ])
        .attach(AdHoc::on_attach("CgminerAddr", |rocket| {
            let default_addr = "127.0.0.1:4028".to_owned();
            let mut timeout = 60;
            if let Ok(cgminer_timeout) = rocket.config().get_int("cgminer_timeout") {
                timeout = cgminer_timeout
            }
            if let Ok(cgminer_addr) = rocket.config().get_string("cgminer_addr") {
                Ok(rocket.manage(api::CgminerConfig {
                    addr: cgminer_addr,
                    timeout: timeout,
                }))
            } else {
                Ok(rocket.manage(api::CgminerConfig {
                    addr: default_addr,
                    timeout: timeout,
                }))
            }
        }))
        .launch();
}
