use sysfs_gpio;
use sysfs_gpio::{Direction, Pin};

use std::thread::sleep;
use std::time::Duration;

pub fn send(dir: f32, time: u32) {
    println!("Recived request to move {} direction for {} milliseconds.", dir, time);
    // Select pins from pin map
    //let (pwm1, dir1, pwm2, dir2) = (Pin::new(18), Pin::new(17), Pin::new(14), Pin::new(15));
    //let mut v = vec![pwm1, dir1, pwm2, dir2];

    //pwm1.set_direction(Direction::Out);

  
}

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