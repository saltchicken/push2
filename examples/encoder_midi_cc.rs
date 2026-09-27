use embedded_graphics::{pixelcolor::Bgr565, prelude::*};
use log::info;
use push2::{button_map::EncoderName, AppConfig, GuiApi, Push2, Push2Event};
use std::{error::Error, thread, time};

// MIDI Control Change status bytes: 
// Channel 1 is 0xB0 (176), Channel 2 is 0xB1 (177).
const CC_STATUS_CH2: u8 = 0xB2;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let mut push2 = Push2::new(AppConfig::load_or_default()?)?;
    
    // Track the absolute 0-127 values of our 8 encoders, starting centered at 64
    let mut encoder_values: [i32; 8] = [64; 8];

    // Initial render of the encoders to the Push 2 display
    push2.display.clear(Bgr565::BLACK)?;
    for i in 0..8u8 {
        push2.display.draw_encoder_outline(i, Bgr565::WHITE)?;
        push2.display.draw_encoder_bar(i, encoder_values[i as usize], Bgr565::GREEN)?;
    }
    push2.display.flush()?;

    info!("Starting event loop. Twist any of the 8 track encoders to send CC on Channel 2.");

    loop {
        let mut needs_redraw = false;

        while let Some(event) = push2.poll_event() {
            if let Push2Event::EncoderTwisted { name, raw_delta } = event {
                // Map the named encoder to an index 0-7
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
                    // Encoders send relative two's complement data.
                    // > 64 means it's a negative delta (twisted left).
                    let delta = if raw_delta > 64 {
                        -((128 - raw_delta) as i32)
                    } else {
                        raw_delta as i32
                    };

                    let old_value = encoder_values[idx as usize];
                    let new_value = (old_value.saturating_add(delta)).clamp(0, 127);

                    if old_value != new_value {
                        encoder_values[idx as usize] = new_value;
                        needs_redraw = true;

                        // Calculate which CC number to send.
                        // For convenience, we map Track 1-8 to CCs 71-78 to match standard Push numbering
                        let cc_number = 71 + idx as u8;
                        let cc_value = new_value as u8;

                        // Send the CC message over MIDI channel 2
                        if let Err(e) = push2.midi_out.send(&[CC_STATUS_CH2, cc_number, cc_value]) {
                            log::error!("Failed to send MIDI: {}", e);
                        } else {
                            info!("Sent CC: Channel=2, CC#={}, Value={}", cc_number, cc_value);
                        }
                    }
                }
            }
        }

        // Only redraw the screen if an encoder actually changed its value
        if needs_redraw {
            push2.display.clear(Bgr565::BLACK)?;
            for i in 0..8u8 {
                push2.display.draw_encoder_outline(i, Bgr565::WHITE)?;
                push2.display.draw_encoder_bar(i, encoder_values[i as usize], Bgr565::GREEN)?;
            }
            push2.display.flush()?;
        }
        
        // Prevent 100% CPU usage
        thread::sleep(time::Duration::from_millis(16));
    }
}
