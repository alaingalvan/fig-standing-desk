#[macro_use]
extern crate nickel;
extern crate rustc_serialize;
extern crate sysfs_gpio;

mod http;

static SERVER_MESSAGE: &'static str = "\
🍐 Fig Standing Desk Server
Version 1.4.3 (April 28, 2017)
================================";

fn main() {

    println!("{}", SERVER_MESSAGE);

    let server = http::create_server();
    
    server
        .listen("localhost:3007")
        .expect("Failed to launch server!");

    println!("Listening @ port 3007, check the /api endpoint!");
}