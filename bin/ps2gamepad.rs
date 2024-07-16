extern crate rpi_demo_emdl;

use emdl_ps2device::prelude::*;
use rpi_demo_emdl::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Firmware: {}!\nCrate: {} v{}!",
        file!(),
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let mut chip = Chip::new("/dev/gpiochip0")?;

    let mut delay = linux_embedded_hal::Delay;
    // gamepad
    let mut gamepad = create_psx_controller(
        chip_input_pin(&mut chip, 26)?,
        chip_output_pin(&mut chip, 13)?,
        chip_output_pin(&mut chip, 6)?,
        chip_output_pin(&mut chip, 5)?,
        AdvancedDelay::new(),
    );

    gamepad.connect();
    {
        let str_gp_type: &str = match gamepad.ctype {
            Ps2DeviceType::Unknown => "Unknown",
            Ps2DeviceType::DualShock1 => "DualShock1",
            Ps2DeviceType::DualShock2 => "DualShock2",
            // Ps2DeviceType::GuitarHero => "GuitarHero",
        };
        let str_gp_state: &str = match gamepad.state {
            Ps2DeviceState::Connected => "connected",
            _ => "connection error",
        };
        let str_gp_mode: &str = match gamepad.is_analog_led {
            true => "Analog",
            _ => "Digital",
        };

        delay.delay_ms(10);
        println!("Gamepad {} {} ({})", str_gp_type, str_gp_state, str_gp_mode);
        // println!("unknown1: {}");
        // print!("unknown1: 0x ").unwrap();
        // aux::write_hex(&gamepad.info.unknown1[..], " ");
        // print!("\nunknown2: 0x ").unwrap();
        // aux::write_hex(&gamepad.info.unknown2[..], " ");
        // print!("\nunknown3: 0x ").unwrap();
        // aux::write_hex(&gamepad.info.unknown3[..], " ");
        // print!("\n");
    }

    println!("Start polling...");

    loop {
        delay.delay_ms(10);
        gamepad.poll();

        if gamepad.is_down(Ps2Button::Select) {
            println!("Select down");
        }
        if gamepad.is_up(Ps2Button::LJoyBtn) {
            println!("LJoyBtn up");
        }
        if gamepad.is_pressed(Ps2Button::RJoyBtn) {
            println!("RJoyBtn pressed");
        }
        if gamepad.is_changed(Ps2Button::Start) {
            println!("Start changed");
        }

        if gamepad.is_down(Ps2Button::Up) {
            println!("Up down");
        }
        if gamepad.is_up(Ps2Button::Right) {
            println!("Right up");
        }
        if gamepad.is_pressed(Ps2Button::Down) {
            println!("Down pressed");
        }
        if gamepad.is_changed(Ps2Button::Left) {
            println!("Left changed");
        }

        if gamepad.is_down(Ps2Button::LTrigger) {
            println!("LTrigger down");
        }
        if gamepad.is_up(Ps2Button::RTrigger) {
            println!("RTrigger up");
        }
        if gamepad.is_pressed(Ps2Button::LButton) || gamepad.is_pressed(Ps2Button::RButton) {
            let a = gamepad.analog_sticks();
            println!(
                "Analog = [Lx: {}, Ly: {}, Rx: {}, Ry: {}]",
                a.lx, a.ly, a.rx, a.ry
            );
        }
        if gamepad.is_changed(Ps2Button::RButton) {
            println!("RButton changed");
        }

        if gamepad.is_down(Ps2Button::Square) {
            println!("Square down");
        }
        if gamepad.is_up(Ps2Button::Cross) {
            println!("Cross up");
        }
        if gamepad.is_pressed(Ps2Button::Circle) {
            println!("Circle pressed");
        }
        if gamepad.is_changed(Ps2Button::Triangle) {
            println!("Triangle changed");
        }
    }
}
