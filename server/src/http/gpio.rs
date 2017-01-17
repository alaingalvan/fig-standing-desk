use std::thread::sleep;
use std::time::Duration;
use std::process::Command;

use sysfs_gpio::{Direction, Pin};
use sysfs_pwm::{Pwm, Result};

pub fn send(vector: f32, time: u32) {

    let dir1 = Pin::new(6);
    let dir2 = Pin::new(17);


    println!("Recived request to move {} direction for {} milliseconds.",
             vector,
             time);

    // Convert direction to normalized float and bool direction.
    let direction = if vector > 0.0 { 1 } else { 0 };
    let length = vector.abs() % 1.0;

    if length < 0.1 || time < 10 {
        return;
    };

    println!("Going in dir {} at length {}.", direction, length);

    // Perhaps this should be conccurrent?
    match dir1.export() {
        Ok(()) => println!("Gpio {} exported!", dir1.get_pin()),
        Err(err) => println!("Gpio {} could not be exported: {}", dir1.get_pin(), err),
    }

    match dir1.set_direction(Direction::Out) {
        Ok(()) => println!("Gpio {} direction set!", dir1.get_pin()),
        Err(err) => println!("Gpio {} could not set direction: {}", dir1.get_pin(), err),
    }

    match dir1.set_value(direction) {
        Ok(()) => println!("Gpio {} value set!", dir1.get_pin()),
        Err(err) => println!("Gpio {} could not set value: {}", dir1.get_pin(), err),
    }

    // 400 hz means 400 times a second

    dir2.export();
    dir2.set_direction(Direction::Out);
    dir2.set_value(direction);

    println!("Sent GPIO Direction signals.");


    let pwm = Pwm::new(0, 0).unwrap(); // number depends on chip, etc.
    pwm.with_exported(|| {
            pwm.enable(true).unwrap();
            pwm.set_period_ns(2500000).unwrap();
            loop {
                pwm_increase_to_max(&pwm, time / 2, 50).unwrap();
                pwm_decrease_to_minimum(&pwm, time / 2, 50).unwrap();
            }
        })
        .unwrap();
    // Since PWM doesn't seem to work, send GPIO 100% signals.
    // let move1 = Pin::new(12);
    //
    // move1.export();
    // move1.set_direction(Direction::Out);
    // move1.set_value(1);
    //
    // let move2 = Pin::new(18);
    // move2.export();
    // move2.set_direction(Direction::Out);
    // move2.set_value(1);
    //
    // let exportpwm0 = Command::new("echo 0 > /sys/class/pwm/pwmchip0/export")
    // .output()
    // .expect("failed to execute process");
    // sleep(Duration::from_millis(10));
    // let set_period = Command::new("echo 10000000 > /sys/class/pwm/pwmchip0/pwm0/period")
    // .output()
    // .expect("failed to execute process");
    // sleep(Duration::from_millis(10));
    // let set_duty = Command::new("echo 8000000 > /sys/class/pwm/pwmchip0/pwm0/duty_cycle")
    // .output()
    // .expect("failed to execute process");
    // sleep(Duration::from_millis(10));
    // let set_enable = Command::new("echo 1 > /sys/class/pwm/pwmchip0/pwm0/enable")
    // .output()
    // .expect("failed to execute process");
    //
    // sleep(Duration::from_millis(time as u64));
    // println!("Sent direction and PWM signals.");
    //
    //
    // move1.set_value(0);
    // move2.set_value(0);

    // move1.unexport();
    // move2.unexport();

    dir1.unexport();
    dir2.unexport();
}


fn pwm_increase_to_max(pwm: &Pwm, duration_ms: u32, update_period_ms: u32) -> Result<()> {
    let step: f32 = duration_ms as f32 / update_period_ms as f32;
    let mut duty_cycle = 0.0;
    let period_ns: u32 = try!(pwm.get_period_ns());
    while duty_cycle < 1.0 {
        try!(pwm.set_duty_cycle_ns((duty_cycle * period_ns as f32) as u32));
        duty_cycle += step;
    }
    pwm.set_duty_cycle_ns(period_ns)
}

fn pwm_decrease_to_minimum(pwm: &Pwm, duration_ms: u32, update_period_ms: u32) -> Result<()> {
    let step: f32 = duration_ms as f32 / update_period_ms as f32;
    let mut duty_cycle = 1.0;
    let period_ns: u32 = try!(pwm.get_period_ns());
    while duty_cycle > 0.0 {
        try!(pwm.set_duty_cycle_ns((duty_cycle * period_ns as f32) as u32));
        duty_cycle -= step;
    }
    pwm.set_duty_cycle_ns(0)
}
