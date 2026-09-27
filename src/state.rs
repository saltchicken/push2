use crate::{ControlName, PushColor};
use std::collections::HashMap;

/// Holds the state of a single 8x8 grid pad
#[derive(Debug, Clone, Copy)]
pub struct PadState {
    /// The last recorded velocity (0 = released)
    pub velocity: u8,
    /// The currently set color
    pub color: PushColor,
}

impl Default for PadState {
    fn default() -> Self {
        Self { velocity: 0, color: crate::colors::BLACK }
    }
}

/// Holds the state of a single control button
#[derive(Debug, Clone, Copy)]
pub struct ButtonState {
    /// The last recorded velocity (0 = released)
    pub velocity: u8,
    /// The currently set brightness/color
    pub light: PushColor,
}

impl Default for ButtonState {
    fn default() -> Self {
        Self { velocity: 0, light: crate::colors::BLACK }
    }
}

#[derive(Debug)]
pub struct Push2State {
    pub pads: [[PadState; 8]; 8],
    pub buttons: HashMap<ControlName, ButtonState>,
    pub slider: u16,
}

impl Push2State {
    pub fn new() -> Self {
        Self {
            pads: [[PadState::default(); 8]; 8],
            buttons: HashMap::new(),
            slider: 0,
        }
    }

    pub fn update_from_event(&mut self, event: &crate::Push2Event) {
        match event {
            crate::Push2Event::PadPressed { coord, velocity } => {
                let pad = &mut self.pads[coord.y as usize][coord.x as usize];
                pad.velocity = *velocity;
            }
            crate::Push2Event::PadReleased { coord } => {
                let pad = &mut self.pads[coord.y as usize][coord.x as usize];
                pad.velocity = 0;
            }
            crate::Push2Event::ButtonPressed { name, velocity } => {
                let button = self.buttons.entry(*name).or_default();
                button.velocity = *velocity;
            }
            crate::Push2Event::ButtonReleased { name } => {
                let button = self.buttons.entry(*name).or_default();
                button.velocity = 0;
            }
            crate::Push2Event::SliderMoved { value } => {
                self.slider = *value;
            }
            _ => {}
        }
    }
}

impl Default for Push2State {
    fn default() -> Self {
        Self::new()
    }
}
