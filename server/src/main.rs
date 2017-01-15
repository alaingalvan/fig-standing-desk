#[macro_use]
extern crate nickel;
extern crate rustc_serialize;

mod http;

fn main() {

    println!("
Fig Standing Desk Server
Version 1.0 (January 7, 2017)
=============================
    ");

    let server = http::create_server();
    server.listen("localhost:3007").expect("Failed to launch server!");
    println!("Listening @ port 3007");
}