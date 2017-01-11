extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::env;
use std::thread::sleep;
use std::time::Duration;

pub fn poll(pin_num: u64) -> sysfs_gpio::Result<()> {

    let input = Pin::new(pin_num);
    input.with_exported(|| {
        try!(input.set_direction(Direction::In));
        let mut prev_val: u8 = 255;
        loop {
            let val = try!(input.get_value());
            if val != prev_val {
                println!("Pin State: {}", if val == 0 { "Low" } else { "High" });
                prev_val = val;
            }
            sleep(Duration::from_millis(10));
        }
    })
}