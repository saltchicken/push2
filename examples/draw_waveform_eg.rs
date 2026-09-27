use embedded_graphics::{pixelcolor::Bgr565, prelude::*};
use log::{debug, info};
use push2::{AppConfig, GuiApi, Push2, Push2Colors, Push2Event, gui, PushColor};
use std::{error::Error, path::PathBuf, thread, time};

const BACKGROUND_COLOR: Bgr565 = Bgr565::BLACK;
const WAVEFORM_COLOR: Bgr565 = Bgr565::GREEN;

pub fn get_audio_storage_path() -> std::io::Result<PathBuf> {
    match dirs::audio_dir() {
        Some(mut path) => {
            path.push("soundboard-recordings");
            std::fs::create_dir_all(&path)?;
            Ok(path)
        }
        None => Err(std::io::Error::other("Could not find audio directory")),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let mut push2 = Push2::new(AppConfig::load_or_default()?)?;
    let image_width = push2.display.size().width;

    let audio_storage_path = get_audio_storage_path()?;
    let input_wav_path = audio_storage_path.join("test.wav");
    
    let peaks = gui::load_waveform_peaks(&input_wav_path, image_width)?;
    push2.display.clear(BACKGROUND_COLOR)?;
    push2.display.draw_waveform_peaks(&peaks, WAVEFORM_COLOR)?;
    push2.display.flush()?;

    info!("Render complete. Starting event loop...");
    loop {
        let event = push2.wait_event();
        match event {
            Push2Event::PadPressed { coord, .. } => {
                push2.set_pad_color(coord, Push2Colors::GREEN_PALE)?;
            }
            Push2Event::PadReleased { coord } => {
                push2.set_pad_color(coord, PushColor(0))?;
            }
            _ => {}
        }
    }
}
