use push2::{button_map::PadCoord, AppConfig, Push2, Push2Event, PushColor};
use log::{debug, info};
use std::{error};

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();

    let config = AppConfig::load_or_default()?;
    let mut push2 = Push2::new(config)?;

    info!("Setting all 64 pads to colors 0-63...");
    for y in 0..8 {
        for x in 0..8 {
            let color_index = (y * 8 + x) as u8;
            let coord = PadCoord { x, y };
            push2.set_pad_color(coord, PushColor(color_index))?;
        }
    }

    info!("All pads set. The device will remain lit.");
    info!("Press any pad to log its (x, y) coordinates and color index.");

    loop {
        // Wait_event blocks so we don't have to spin the CPU!
        let event = push2.wait_event();
        match event {
            Push2Event::PadPressed { coord, .. } => {
                let color_index = (coord.y * 8 + coord.x) as u8;
                info!("Pad ({}, {}) PRESSED. Color index: {}", coord.x, coord.y, color_index);
            }
            _ => debug!("Received event: {:?}", event),
        }
    }
}
