use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};

pub fn send(direction: f32, time: u32) {

    println!("Recived request to move {} direction for {} milliseconds.",
             direction,
             time);

    // Convert direction to normalized float and bool direction.
    let dir = if direction > 0.0 { 1 } else { 0 };
    let length = direction.abs() % 1.001;

    // To protect mechanical relays, avoid bursts that are too short.
    if length < 0.1 || time < 10 {
        return;
    };

    println!("Going in dir {} at length {}.", direction, length);

    let actuator1up = Pin::new(6);
    let actuator1down = Pin::new(13);
    let actuator2up = Pin::new(19);
    let actuator2down = Pin::new(26);

    actuator1up.export();
    actuator2up.export();
    actuator1down.export();
    actuator2down.export();

    actuator1up.set_direction(Direction::Out);
    actuator2up.set_direction(Direction::Out);
    actuator1down.set_direction(Direction::Out);
    actuator2down.set_direction(Direction::Out);

    actuator1up.set_value(0);
    actuator1down.set_value(0);
    actuator2up.set_value(0);
    actuator2down.set_value(0);

    match dir {
        0 => {
            actuator1up.set_value(1);
            actuator2up.set_value(1);
        }
        _ => {
            actuator1down.set_value(1);
            actuator2down.set_value(1);
        }
    }

    sleep(Duration::from_millis(time as u64));

    actuator1up.set_value(0);
    actuator1down.set_value(0);
    actuator2up.set_value(0);
    actuator2down.set_value(0);

    actuator1up.unexport();
    actuator2up.unexport();
    actuator1down.unexport();
    actuator2down.unexport();

    println!("Sent Relay Signals.");

}

