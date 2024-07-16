use std::time::Instant;

fn chip_init_pin(
    chip: &mut gpio_cdev::Chip,
    offset: u32,
    lrf: LineRequestFlags,
) -> Result<linux_embedded_hal::CdevPin, linux_embedded_hal::gpio_cdev::Error> {
    let handle = chip.get_line(offset)?.request(lrf, 0, "emdl-demo")?;
    CdevPin::new(handle)
}

pub struct AdvancedDelay(linux_embedded_hal::Delay);
impl AdvancedDelay {
    pub fn new() -> Self {
        Self(linux_embedded_hal::Delay)
    }
}
impl DelayNs for AdvancedDelay {
    fn delay_ns(&mut self, ns: u32) {
        let mut wait = true;
        let ns = ns.saturating_sub(1000);
        let start = Instant::now();
        while wait {
            self.0.delay_ns(0);
            if start.elapsed().as_nanos() as u32 >= ns {
                wait = false;
            }
        }
    }
}

pub mod prelude {
    pub use super::AdvancedDelay;
    pub use embedded_hal::{
        delay::DelayNs,
        digital::{OutputPin, PinState},
    };
    pub use linux_embedded_hal::{
        gpio_cdev::{self, Chip, Line, LineDirection, LineInfo, LineRequestFlags},
        CdevPin,
    };
    pub fn chip_input_pin(
        chip: &mut gpio_cdev::Chip,
        offset: u32,
    ) -> Result<CdevPin, gpio_cdev::Error> {
        super::chip_init_pin(chip, offset, LineRequestFlags::INPUT)?.into_input_pin()
    }
    pub fn chip_output_pin(
        chip: &mut gpio_cdev::Chip,
        offset: u32,
    ) -> Result<CdevPin, gpio_cdev::Error> {
        super::chip_init_pin(chip, offset, LineRequestFlags::OUTPUT)?.into_output_pin(PinState::Low)
    }
}

pub use self::prelude::*;
