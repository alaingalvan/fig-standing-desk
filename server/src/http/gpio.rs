use std::thread::sleep;
use std::time::Duration;

pub fn send(vector: f32, time: u32) {
    println!("Recived request to move {} direction for {} milliseconds.", vector, time);

    // Convert direction to normalized float and bool direction.
    // ================================

    let length = vector.abs().rem(1.0);

    let direction = vector > 0.0;

    println!("Going in dir {} at length {}.", direction, length);

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
    let rng1_mask: u32 = 32 * length;

    // DTA1 register (Pulses)
    let data1_mask: u32 = 0x1;

    // GPIO12 ALT0

    // GPIO18 ALT5

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