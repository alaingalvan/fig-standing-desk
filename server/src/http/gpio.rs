use std::thread::sleep;
use std::time::Duration;
use std::process::Command;

use sysfs_gpio::{Direction, Pin};

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

    dir1.export();
    dir1.set_direction(Direction::Out);
    dir1.set_value(direction);


    dir2.export();
    dir2.set_direction(Direction::Out);
    dir2.set_value(direction);

    println!("Sent GPIO Direction signals.");

    // Since PWM doesn't seem to work, send GPIO 100% signals.
    let move1 = Pin::new(12);

    move1.export();
    move1.set_direction(Direction::Out);
    move1.set_value(1);
    
    let move2 = Pin::new(18);
    move2.export();
    move2.set_direction(Direction::Out);
    move2.set_value(1);

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

    sleep(Duration::from_millis(time as u64));
    println!("Sent direction and PWM signals.");

    dir1.unexport();
    dir2.unexport();
    move1.unexport();
    move2.unexport();
}