// --- Module Declarations ---
pub mod app_config;
pub mod button_map;
pub mod colors;
pub mod display;
pub mod gui;
pub mod midi_handler;
pub mod state;

// --- Public API Re-exports ---
pub use app_config::{AppConfig, ConfigError};
pub use button_map::{ButtonMap, ButtonMapError, ControlName, EncoderName, PadCoord};
pub use colors::{PushColor, self as Push2Colors};
pub use display::{Push2Display, Push2DisplayError};
use embedded_graphics::prelude::Point;
pub use gui::GuiApi;
pub use midi_handler::{MidiHandler, MidiHandlerError};
use midir::{MidiInputConnection, MidiOutputConnection, SendError};
pub use state::Push2State;
use std::sync::mpsc::Receiver;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Push2Error {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    #[cfg(feature = "waveform")]
    #[error("Waveform error: {0}")]
    Waveform(#[from] gui::WaveformError),
    #[error("Button map error: {0}")]
    ButtonMap(#[from] ButtonMapError),
    #[error("Display error: {0}")]
    Display(#[from] Push2DisplayError),
    #[error("MIDI initialization error: {0}")]
    MidiInit(#[from] MidiHandlerError),
    #[error("MIDI send error: {0}")]
    MidiSend(#[from] SendError),
}

// --- MIDI Message Constants ---
pub const NOTE_ON: u8 = 144;
pub const NOTE_OFF: u8 = 128;
pub const CONTROL_CHANGE: u8 = 176;
pub const PITCH_BEND: u8 = 224;

/// High-level events from the Ableton Push 2
#[derive(Debug, Clone, Copy)]
pub enum Push2Event {
    PadPressed { coord: PadCoord, velocity: u8 },
    PadReleased { coord: PadCoord },
    ButtonPressed { name: ControlName, velocity: u8 },
    ButtonReleased { name: ControlName },
    EncoderTwisted { name: EncoderName, raw_delta: u8 },
    SliderMoved { value: u16 },
}

/// Main struct for interfacing with the Ableton Push 2
pub struct Push2 {
    pub display: Push2Display,
    pub midi_out: MidiOutputConnection,
    pub button_map: ButtonMap,
    pub state: Push2State,
    event_rx: Receiver<Vec<u8>>,
    _conn_in: MidiInputConnection<()>,
}

impl Push2 {
    /// Connects to the Push 2 display and MIDI ports explicitly requiring config.
    pub fn new(app_config: AppConfig) -> Result<Self, Push2Error> {
        let (tx, rx) = std::sync::mpsc::channel();
        let midi_handler = MidiHandler::new(&app_config, tx)?;
        let button_map = ButtonMap::new()?;
        let display = Push2Display::new()?;
        let MidiHandler { _conn_in, conn_out } = midi_handler;
        
        let mut push2 = Self {
            display,
            midi_out: conn_out,
            button_map,
            event_rx: rx,
            _conn_in,
            state: Push2State::new(),
        };
        
        push2.reset_all_lights()?;
        Ok(push2)
    }

    fn reset_all_lights(&mut self) -> Result<(), Push2Error> {
        for address in 36..=99 {
            self.midi_out.send(&[NOTE_OFF, address, 0])?;
        }
        for address in self.button_map.get_control_addresses() {
            self.midi_out.send(&[CONTROL_CHANGE, address, 0])?;
        }
        Ok(())
    }

    pub fn set_pad_color(&mut self, coord: PadCoord, color: PushColor) -> Result<(), Push2Error> {
        if let Some(address) = self.button_map.get_note_address(coord) {
            let message = if color.0 == 0 {
                [NOTE_OFF, address, 0]
            } else {
                [NOTE_ON, address, color.0]
            };
            self.midi_out.send(&message)?;
            self.state.pads[coord.y as usize][coord.x as usize].color = color;
        }
        Ok(())
    }

    pub fn set_button_light(&mut self, name: ControlName, light: PushColor) -> Result<(), Push2Error> {
        if let Some(address) = self.button_map.get_control_address(name) {
            let message = if light.0 == 0 {
                [CONTROL_CHANGE, address, 0]
            } else {
                [CONTROL_CHANGE, address, light.0]
            };
            self.midi_out.send(&message)?;
            self.state.buttons.entry(name).or_default().light = light;
        }
        Ok(())
    }

    /// Sends a standard Control Change (CC) message to a specific 1-indexed MIDI channel (1-16).
    pub fn send_cc(&mut self, channel: u8, cc: u8, value: u8) -> Result<(), Push2Error> {
        // Clamp channel to 1-16, then convert to 0-15 for the MIDI status byte
        let ch_index = channel.clamp(1, 16) - 1;
        
        // 0xB0 (176) is the base Control Change status. Adding the channel index targets the right channel.
        let status = CONTROL_CHANGE | ch_index;
        
        self.midi_out.send(&[status, cc, value])?;
        Ok(())
    }

    /// Sends a CC message for a specific encoder on a specific 1-indexed MIDI channel (1-16).
    pub fn send_encoder_cc(&mut self, channel: u8, name: EncoderName, value: u8) -> Result<(), Push2Error> {
        if let Some(cc) = self.button_map.get_encoder_address(name) {
            self.send_cc(channel, cc, value)?;
        }
        Ok(())
    }

    pub fn draw_bmp_to_display(&mut self, bmp_data: &[u8], position: Point) -> Result<(), Push2Error> {
        self.display.draw_bmp(bmp_data, position)?;
        Ok(())
    }

    /// Helper to parse raw MIDI events into a Push2Event
    fn parse_message(&self, message: &[u8]) -> Option<Push2Event> {
        if message.is_empty() { return None; }
        match message[0] {
            NOTE_ON | NOTE_OFF if message.len() >= 3 => {
                let (address, velocity) = (message[1], message[2]);
                if let Some(pad_coord) = self.button_map.get_note(address) {
                    if message[0] == NOTE_ON && velocity > 0 {
                        Some(Push2Event::PadPressed { coord: pad_coord, velocity })
                    } else {
                        Some(Push2Event::PadReleased { coord: pad_coord })
                    }
                } else { None }
            }
            CONTROL_CHANGE if message.len() >= 3 => {
                let (address, velocity) = (message[1], message[2]);
                if let Some(control_name) = self.button_map.get_control(address) {
                    if velocity > 0 {
                        Some(Push2Event::ButtonPressed { name: control_name, velocity })
                    } else {
                        Some(Push2Event::ButtonReleased { name: control_name })
                    }
                } else if let Some(encoder_name) = self.button_map.get_encoder(address) {
                    Some(Push2Event::EncoderTwisted { name: encoder_name, raw_delta: velocity })
                } else { None }
            }
            PITCH_BEND if message.len() >= 3 => {
                let value = ((message[2] as u16) << 7) | (message[1] as u16);
                Some(Push2Event::SliderMoved { value })
            }
            _ => None,
        }
    }

    /// Polls for the next high-level `Push2Event` (non-blocking).
    pub fn poll_event(&mut self) -> Option<Push2Event> {
        while let Ok(message) = self.event_rx.try_recv() {
            if let Some(event) = self.parse_message(&message) {
                self.state.update_from_event(&event);
                return Some(event);
            }
        }
        None
    }

    /// Blocks until the next high-level `Push2Event` is received.
    pub fn wait_event(&mut self) -> Push2Event {
        loop {
            if let Ok(message) = self.event_rx.recv() {
                if let Some(event) = self.parse_message(&message) {
                    self.state.update_from_event(&event);
                    return event;
                }
            } else {
                panic!("MIDI receive channel disconnected");
            }
        }
    }
}
