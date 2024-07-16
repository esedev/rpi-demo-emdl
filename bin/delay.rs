extern crate rpi_demo_emdl;

use rpi_demo_emdl::prelude::*;
use std::{
    error::Error,
    fmt,
    time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Firmware: {}!\nCrate: {} v{}!",
        file!(),
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    // let mut delay = linux_embedded_hal::Delay;
    let mut delay = AdvancedDelay::new();

    loop {
        let start = Instant::now();
        delay.delay_ms(0);
        let tm = start.elapsed();
        println!("delay_ms(0) duration: {}", OutDuration(tm));

        let start = Instant::now();
        delay.delay_us(2);
        let tm = start.elapsed();
        println!("delay_us(2) duration: {}", OutDuration(tm));

        let start = Instant::now();
        delay.delay_ns(0);
        delay.delay_ns(0);
        let tm = start.elapsed();
        println!("2 x delay_ns(0) duration: {}", OutDuration(tm));

        // led.set_low().unwrap();
        let start = Instant::now();
        delay.delay_ms(500);
        let tm = start.elapsed();
        println!("delay_ms(500) duration: {}", OutDuration(tm));

        // led.set_high().unwrap();
        let start = Instant::now();
        delay.delay_ms(1000);
        let tm = start.elapsed();
        println!("delay_ms(1000) duration: {}", OutDuration(tm));
    }

    // Ok(())
}

///
/// Output display wraper for [`Duration`]
///
struct OutDuration(Duration);
impl fmt::Display for OutDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut t = self.0.as_nanos();
        let n = t % 1000;
        t /= 1000;
        let u = t % 1000;
        t /= 1000;
        let m = t % 1000;
        let s = t / 1000;
        write!(f, "{:1}.{:03}m{:03}μ{:03}n", s, m, u, n)
    }
}
