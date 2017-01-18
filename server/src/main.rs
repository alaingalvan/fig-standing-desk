#[macro_use]
extern crate nickel;
extern crate rustc_serialize;
extern crate sysfs_gpio;
extern crate sysfs_pwm;

mod http;

fn main() {

    println!("
Fig Standing Desk Server
Version 1.2.2 (January 18, 2017)
================================");

    let server = http::create_server();
    server.listen("localhost:3007").expect("Failed to launch server!");
    println!("Listening @ port 3007");
}