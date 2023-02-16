//! # imclasregan server
//! Backend webserver for imclasregan - an image annotation tool.
//!
//! This server servers web pages and responds to commands (recieved via POST).
//! [Command]s are normally to fetch/store data from/into the associated SQLite database.

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

/// [Command] handler function. A command is serialised in json format and
/// POSTed to the server. This handler responds to the given command with a
/// serialised [Reply](reply::Reply).
///
/// The response ellicited depends on the value of the [Command] enum:
///  - [Command::GetImage] => [get_image]
///  - [Command::GetClassifications] => [get_classes]
///  - [Command::GetRegression] => [get_regression]
///  - [Command::StoreClassificationResult] => [store_classification]
///  - [Command::StoreRegressionResult] => [store_regression]
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

/// Main function of the server. Sets up the bind point for incoming connection
/// based on the environment variables:
///  - IMCLASREGAN_IP sets the IP address
///  - IMCLASREGAN_PORT sets the port
///
/// The server is a typical file server, responding to GET requests with the
/// contents of the requested file. In addition, some POST requests are handled
/// by [command_handler()].
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
