use std::thread::sleep;
use std::time::Duration;

extern crate sysfs_gpio;

pub fn send(vector: f32, time: u32) {
    println!("Recived request to move {} direction for {} milliseconds.", vector, time);

    // Convert direction to normalized float and bool direction.
    // ================================

    let length = vector.abs() % 1.0;

    let direction = vector > 0.0;

    println!("Going in dir {} at length {}.", direction, length);

    // Access Physical Memory Location on Linux via memmap

    //

    // Write Direction to GPIO 6 and 17
    // Refer to BCM2835 ARM Peripherals page 89
    // ================================



    // Write to PWM Registers
    // Refer to BCM2835 ARM Peripherals page 138
    // ================================

    // CTL Register
    let ctl_mask: u32 = 0b1000000110000001;

    // DMAC register
    let dmac_mask: u32 = 0x8000;

    // RNG1 register (Pulse Width)
    let rng1_mask: u32 = (32.0 * length) as u32;

    // DTA1 register (Pulses)
    let data1_mask: u32 = 0x1;

    // GPIO12 ALT0

    // GPIO18 ALT5

}