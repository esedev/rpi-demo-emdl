extern crate rpi_demo_emdl;

use rpi_demo_emdl::prelude::*;
use std::{error::Error, fmt};

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
                println!(
                    "Chip information:\n\tpath: {}\n\tname: {}\n\tlabel: {}\n\tlines: {}",
                    chip.path().display(),
                    chip.name(),
                    chip.label(),
                    chip.num_lines()
                );
                for line in chip.lines() {
                    match line.info() {
                        Ok(info) => {
                            let value = if info.name().unwrap_or_default().starts_with("GPIO") {
                                match line.request(LineRequestFlags::INPUT, 0, "read-value") {
                                    Ok(handle) => match handle.get_value() {
                                        Ok(value) => value.to_string(),
                                        Err(e) => format!("Err({e})"),
                                    },
                                    Err(e) => format!("Err({e})"),
                                }
                            } else {
                                "?".to_string()
                            };
                            println!(
                                "\tline #{:02}: {{ {} }} value: {value}",
                                line.offset(),
                                LineInfoFormater(&info)
                            )
                        }
                        Err(e) => eprintln!("\tline #{}: {}", line.offset(), e),
                    }
                }
            }
            Err(e) => eprintln!("Chip fail: {e}"),
        }
    }

    Ok(())
}

/// LineInfoFormater
struct LineInfoFormater<'s>(&'s LineInfo);
impl<'s> fmt::Display for LineInfoFormater<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = 0u32;
        if self.0.is_kernel() {
            flags |= 1 << 0;
        }
        if self.0.is_used() {
            flags |= 1 << 1;
        }
        if self.0.is_active_low() {
            flags |= 1 << 2;
        }
        if self.0.is_open_drain() {
            flags |= 1 << 3;
        }
        if self.0.is_open_source() {
            flags |= 1 << 4;
        }
        write!(
            f,
            "name: {}, consumer: {}, dir: {}, flags: {:05b}",
            self.0.name().unwrap_or("None"),
            self.0.consumer().unwrap_or("None"),
            match self.0.direction() {
                LineDirection::In => "In",
                LineDirection::Out => "Out",
            },
            flags
        )?;
        Ok(())
    }
}
