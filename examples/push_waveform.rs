use push2::{AppConfig, Push2, Push2Colors, Push2Event, PushColor};

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Bgr565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};
use log::{debug, info};
use std::path::PathBuf;
use std::{error, fs, thread, time};

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

const PAD_COLOR_ON: PushColor = Push2Colors::GREEN_PALE;
const BUTTON_LIGHT_ON: PushColor = Push2Colors::GREEN_PALE;

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();
    
    let mut push2 = Push2::new(AppConfig::load_or_default()?)?;

    let audio_storage_path = get_audio_storage_path()?;
    let bmp_path = audio_storage_path.join("waveform.bmp");

    let bmp_data = fs::read(&bmp_path).map_err(|e| {
        format!("Failed to read 'waveform.bmp' from {}: {}", audio_storage_path.display(), e)
    })?;

    let text_style = MonoTextStyle::new(&FONT_10X20, Bgr565::WHITE);
    let mut position = Point::new(0, 70);
    let mut step = 4;

    loop {
        while let Some(event) = push2.poll_event() {
            match event {
                Push2Event::PadPressed { coord, .. } => {
                    push2.set_pad_color(coord, PAD_COLOR_ON)?;
                }
                Push2Event::PadReleased { coord } => {
                    push2.set_pad_color(coord, PushColor(0))?;
                }
                Push2Event::ButtonPressed { name, .. } => {
                    push2.set_button_light(name, BUTTON_LIGHT_ON)?;
                }
                Push2Event::ButtonReleased { name } => {
                    push2.set_button_light(name, PushColor(0))?;
                }
                _ => {}
            }
        }

        push2.display.clear(Bgr565::BLACK)?;
        push2.draw_bmp_to_display(&bmp_data, Point::zero())?;

        Rectangle::new(Point::zero(), push2.display.size())
            .into_styled(PrimitiveStyle::with_stroke(Bgr565::WHITE, 1))
            .draw(&mut push2.display)?;

        position.x += step;
        if position.x >= push2.display.size().width as i32 || position.x <= 0 {
            step *= -1;
        }

        Text::new("Hello!", position, text_style).draw(&mut push2.display)?;
        push2.display.flush()?;

        thread::sleep(time::Duration::from_millis(1000 / 60));
    }
}
