#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]

extern crate rocket;
pub mod utils;
pub mod api;
use api::deploy;

fn main() {
    rocket::ignite().mount("/api", routes![deploy::create]).launch();
}
