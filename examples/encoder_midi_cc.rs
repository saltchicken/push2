use embedded_graphics::{pixelcolor::Bgr565, prelude::*};
use log::info;
use push2::{button_map::EncoderName, AppConfig, GuiApi, Push2, Push2Event};
use std::{collections::HashMap, error::Error, thread, time};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let mut push2 = Push2::new(AppConfig::load_or_default()?)?;
    
    // Track the absolute 0-127 values of our 8 encoders, starting centered at 64
    let mut encoder_values: HashMap<EncoderName, i32> = HashMap::new();
    let track_encoders = [
        EncoderName::Track1, EncoderName::Track2, EncoderName::Track3, EncoderName::Track4,
        EncoderName::Track5, EncoderName::Track6, EncoderName::Track7, EncoderName::Track8,
    ];

    for &encoder in &track_encoders {
        encoder_values.insert(encoder, 64);
    }

    // Initial render of the encoders to the Push 2 display
    push2.display.clear(Bgr565::BLACK)?;
    for (i, &encoder) in track_encoders.iter().enumerate() {
        let val = *encoder_values.get(&encoder).unwrap();
        push2.display.draw_encoder_outline(i as u8, Bgr565::WHITE)?;
        push2.display.draw_encoder_bar(i as u8, val, Bgr565::GREEN)?;
    }
    push2.display.flush()?;

    info!("Starting event loop. Twist any of the 8 track encoders to send CC on Channel 2.");

    loop {
        let mut needs_redraw = false;

        while let Some(event) = push2.poll_event() {
            if let Push2Event::EncoderTwisted { name, raw_delta } = event {
                // Only process the 8 track encoders for this example
                if !track_encoders.contains(&name) {
                    continue;
                }

                // Encoders send relative two's complement data.
                // > 64 means it's a negative delta (twisted left).
                let delta = if raw_delta > 64 {
                    -((128 - raw_delta) as i32)
                } else {
                    raw_delta as i32
                };

                let old_value = *encoder_values.get(&name).unwrap_or(&64);
                let new_value = (old_value.saturating_add(delta)).clamp(0, 127);

                if old_value != new_value {
                    encoder_values.insert(name, new_value);
                    needs_redraw = true;

                    // Send the CC message cleanly on Channel 2
                    if let Err(e) = push2.send_encoder_cc(2, name, new_value as u8) {
                        log::error!("Failed to send MIDI: {}", e);
                    } else {
                        info!("Sent CC: Channel=2, Encoder={:?}, Value={}", name, new_value);
                    }
                }
            }
        }

        // Only redraw the screen if an encoder actually changed its value
        if needs_redraw {
            push2.display.clear(Bgr565::BLACK)?;
            for (i, &encoder) in track_encoders.iter().enumerate() {
                let val = *encoder_values.get(&encoder).unwrap();
                push2.display.draw_encoder_outline(i as u8, Bgr565::WHITE)?;
                push2.display.draw_encoder_bar(i as u8, val, Bgr565::GREEN)?;
            }
            push2.display.flush()?;
        }
        
        // Prevent 100% CPU usage
        thread::sleep(time::Duration::from_millis(16));
    }
}
