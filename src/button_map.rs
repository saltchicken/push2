use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ButtonMapError {
    #[error("Mapping failure: {0}")]
    MappingError(String),
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PadCoord {
    pub x: u8,
    pub y: u8,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControlName {
    TapTempo, Metronome, Delete, Undo, Mute, Solo, Stop, Convert, 
    DoubleLoop, Quantize, Duplicate, New, FixedLength, Automate, Record, Play,
    UpperRow1, UpperRow2, UpperRow3, UpperRow4, UpperRow5, UpperRow6, UpperRow7, UpperRow8,
    LowerRow1, LowerRow2, LowerRow3, LowerRow4, LowerRow5, LowerRow6, LowerRow7, LowerRow8,
    Beat1_32t, Beat1_32, Beat1_16t, Beat1_16, Beat1_8t, Beat1_8, Beat1_4t, Beat1_4,
    Setup, User, AddDevice, AddTrack, Device, Mix, Browse, Clip, Master,
    Up, Down, Left, Right, Repeat, Accent, Scale, Layout, Note, Session,
    OctaveUp, OctaveDown, PageLeft, PageRight, Shift, Select,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncoderName {
    Tempo, Swing, Track1, Track2, Track3, Track4, Track5, Track6, Track7, Track8, Master,
}

/// Statically typed lookup structure containing Push 2 button mappings.
pub struct ButtonMap;

impl ButtonMap {
    pub fn new() -> Result<Self, ButtonMapError> {
        Ok(Self)
    }

    /// Converts a raw MIDI note to a 2D Pad Coordinate `(x, y)` in `O(1)` mathematically.
    pub fn get_note(&self, address: u8) -> Option<PadCoord> {
        // Standard Push 2 grid: Notes 36 (bottom-left) to 99 (top-right)
        if (36..=99).contains(&address) {
            let offset = address - 36;
            let x = offset % 8;
            let y = 7 - (offset / 8);
            Some(PadCoord { x, y })
        } else {
            None
        }
    }

    /// Converts a 2D Pad Coordinate `(x, y)` back to a raw MIDI note.
    pub fn get_note_address(&self, coord: PadCoord) -> Option<u8> {
        if coord.x > 7 || coord.y > 7 {
            None
        } else {
            Some(36 + (7 - coord.y) * 8 + coord.x)
        }
    }

    pub fn get_control(&self, address: u8) -> Option<ControlName> {
        // Map common Push 2 CCs (These match standard defaults, expand as necessary)
        match address {
            3 => Some(ControlName::TapTempo),
            9 => Some(ControlName::Metronome),
            118 => Some(ControlName::Delete),
            119 => Some(ControlName::Undo),
            _ => None,
        }
    }

    pub fn get_control_address(&self, name: ControlName) -> Option<u8> {
        match name {
            ControlName::TapTempo => Some(3),
            ControlName::Metronome => Some(9),
            ControlName::Delete => Some(118),
            ControlName::Undo => Some(119),
            _ => None, 
        }
    }

    pub fn get_encoder(&self, address: u8) -> Option<EncoderName> {
        match address {
            14 => Some(EncoderName::Tempo),
            15 => Some(EncoderName::Swing),
            71 => Some(EncoderName::Track1),
            72 => Some(EncoderName::Track2),
            73 => Some(EncoderName::Track3),
            74 => Some(EncoderName::Track4),
            75 => Some(EncoderName::Track5),
            76 => Some(EncoderName::Track6),
            77 => Some(EncoderName::Track7),
            78 => Some(EncoderName::Track8),
            79 => Some(EncoderName::Master),
            _ => None,
        }
    }

    /// Converts an EncoderName back to a raw MIDI CC address.
    pub fn get_encoder_address(&self, name: EncoderName) -> Option<u8> {
        match name {
            EncoderName::Tempo => Some(14),
            EncoderName::Swing => Some(15),
            EncoderName::Track1 => Some(71),
            EncoderName::Track2 => Some(72),
            EncoderName::Track3 => Some(73),
            EncoderName::Track4 => Some(74),
            EncoderName::Track5 => Some(75),
            EncoderName::Track6 => Some(76),
            EncoderName::Track7 => Some(77),
            EncoderName::Track8 => Some(78),
            EncoderName::Master => Some(79),
        }
    }

    /// Returns a list of supported Control Addresses for initialization resets
    pub fn get_control_addresses(&self) -> impl Iterator<Item = u8> {
        [3, 9, 118, 119].into_iter()
    }
}
