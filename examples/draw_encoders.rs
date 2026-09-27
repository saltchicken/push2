use embedded_graphics::{pixelcolor::Bgr565, prelude::*};
use log::debug;
use push2::{AppConfig, GuiApi, Push2, Push2Event, button_map::EncoderName};
use std::{error::Error, thread, time};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    
    let mut push2 = Push2::new(AppConfig::load_or_default()?)?;
    let mut track_encoder_values: [i32; 8] = [64; 8];
    
    push2.display.clear(Bgr565::BLACK)?;
    for i in 0..8u8 {
        push2.display.draw_encoder_outline(i, Bgr565::WHITE)?;
        push2.display.draw_encoder_bar(i, track_encoder_values[i as usize], Bgr565::GREEN)?;
    }
    push2.display.flush()?;

    debug!("Starting event loop. Twist any track encoders.");
    loop {
        let mut needs_redraw = false;
        
        while let Some(event) = push2.poll_event() {
            if let Push2Event::EncoderTwisted { name, raw_delta } = event {
                let index = match name {
                    EncoderName::Track1 => Some(0),
                    EncoderName::Track2 => Some(1),
                    EncoderName::Track3 => Some(2),
                    EncoderName::Track4 => Some(3),
                    EncoderName::Track5 => Some(4),
                    EncoderName::Track6 => Some(5),
                    EncoderName::Track7 => Some(6),
                    EncoderName::Track8 => Some(7),
                    _ => None,
                };
                if let Some(idx) = index {
                    let delta = if raw_delta > 64 { -((128 - raw_delta) as i32) } else { raw_delta as i32 };
                    let new_value = (track_encoder_values[idx as usize].saturating_add(delta)).clamp(0, 127);
                    if track_encoder_values[idx as usize] != new_value {
                        track_encoder_values[idx as usize] = new_value;
                        needs_redraw = true;
                    }
                }
            }
        }
        
        if needs_redraw {
            push2.display.clear(Bgr565::BLACK)?;
            for i in 0..8u8 {
                push2.display.draw_encoder_outline(i, Bgr565::WHITE)?;
                push2.display.draw_encoder_bar(i, track_encoder_values[i as usize], Bgr565::GREEN)?;
            }
            push2.display.flush()?;
        }
        thread::sleep(time::Duration::from_millis(16));
    }
}
