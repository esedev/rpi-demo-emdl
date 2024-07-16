extern crate rpi_demo_emdl;

use rpi_demo_emdl::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Firmware: {}!\nCrate: {} v{}!",
        file!(),
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let chips = linux_embedded_hal::gpio_cdev::chips()?;
    for result_chip in chips {
        match result_chip {
            Ok(chip) => {
                for line in chip.lines() {
                    if let Err(err) = reset_pin(&line) {
                        eprintln!(
                            "chip {} line #{} fail: {}",
                            chip.path().display(),
                            line.offset(),
                            err
                        );
                    }
                }
            }
            Err(e) => eprintln!("Chip fail: {e}"),
        }
    }

    Ok(())
}

fn reset_pin(line: &Line) -> Result<(), gpio_cdev::errors::Error> {
    let info = line.info()?;
    if !info.name().map(|e| e.starts_with("GPIO")).unwrap_or(false) {
        return Ok(());
    }
    let handle = line.request(LineRequestFlags::INPUT, 0, "gpio-reset")?;
    let pin = CdevPin::new(handle)?;
    let pin = pin.into_input_pin()?;
    drop(pin);
    Ok(())
}
