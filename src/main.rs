#[macro_use]
extern crate rocket;

mod command;
mod database;
mod reply;

use std::env;
use std::net::Ipv4Addr;

use rocket::fs::FileServer;
use rocket::response::content;
use rocket::serde::json::Json;

use crate::command::Command;
use crate::database::{
    get_classes, get_image, get_regression, store_classification, store_regression,
};

#[post("/site", data = "<cmd>")]
async fn command_handler(cmd: Json<Command<'_>>) -> content::RawJson<String> {
    let reply = match cmd.0 {
        Command::GetImage => get_image(),
        Command::GetClassifications => get_classes(),
        Command::GetRegression { kind } => get_regression(kind),
        Command::StoreClassificationResult { cid, iid, sid, tt } => {
            store_classification(cid, iid, sid, tt)
        }
        Command::StoreRegressionResult {
            rid,
            lid,
            mid,
            sid,
            tt,
        } => store_regression(rid, lid, mid, sid, tt),
    };
    let json = serde_json::to_string(&reply).unwrap();
    content::RawJson(json)
}

#[launch]
fn rocket() -> _ {
    let ip: Ipv4Addr = match env::var("IMCLASREGAN_IP") {
        Ok(ip) => ip.as_str().parse().unwrap(),
        Err(_) => "127.0.0.1".parse().unwrap(),
    };
    let port: u16 = match env::var("IMCLASREGAN_PORT") {
        Ok(port) => port.as_str().parse().unwrap(),
        Err(_) => "8008".parse().unwrap(),
    };
    let config = rocket::Config {
        port: port,
        address: ip.into(),
        ..rocket::Config::debug_default()
    };

    rocket::custom(&config)
        .mount("/", FileServer::from("site/"))
        .mount("/", routes![command_handler])
}
