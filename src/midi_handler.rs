use crate::app_config::AppConfig;
use log::{info, warn};
use midir::{
    ConnectError, Ignore, InitError, MidiInput, MidiInputConnection, MidiInputPort, MidiOutput,
    MidiOutputConnection, MidiOutputPort, PortInfoError,
};
use std::io::{self, Write, stdin, stdout};
use std::num::ParseIntError;
use std::sync::mpsc::Sender;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MidiHandlerError {
    #[error("MidiInput initialization failed: {0}")]
    InputInit(#[from] InitError),
    #[error("MidiOutput initialization failed: {0}")]
    OutputInit(InitError),
    #[error("Failed to get port name: {0}")]
    PortName(#[from] PortInfoError),
    #[error("Input connection failed: {0}")]
    InputConnection(#[from] ConnectError<MidiInput>),
    #[error("Output connection failed: {0}")]
    OutputConnection(#[from] ConnectError<MidiOutput>),
    #[error("No MIDI input ports found")]
    NoInputPorts,
    #[error("No MIDI output ports found")]
    NoOutputPorts,
    #[error("Invalid port selection: {0}")]
    InvalidPortSelection(#[from] ParseIntError),
    #[error("STDIO error: {0}")]
    IOError(#[from] io::Error),
    #[error("Invalid input port index")]
    InvalidInputPortIndex,
    #[error("Invalid output port index")]
    InvalidOutputPortIndex,
}

/// Holds the MIDI connections.
/// `_conn_in` is kept to ensure it stays alive (RAII).
/// `conn_out` is public so `main.rs` can send messages.
pub struct MidiHandler {
    pub _conn_in: MidiInputConnection<()>,
    pub conn_out: MidiOutputConnection,
}

impl MidiHandler {
    /// Creates a new MidiHandler, finds and connects to ports.
    pub fn new(config: &AppConfig, tx: Sender<Vec<u8>>) -> Result<Self, MidiHandlerError> {
        // --- Input Connection ---
        let mut midi_in = MidiInput::new("push2")?;
        midi_in.ignore(Ignore::None);

        let in_port = Self::select_input_port(&midi_in, &config.midi_input_port)?;
        let in_port_name = midi_in.port_name(&in_port)?;

        info!("Opening input connection to: {}", in_port_name);
        let _conn_in = midi_in.connect(
            &in_port,
            "push2-input-connection",
            move |_stamp, message, _| {
                tx.send(message.to_vec()).unwrap();
            },
            (),
        )?;

        // --- Output Connection ---
        let midi_out = MidiOutput::new("push2_output").map_err(MidiHandlerError::OutputInit)?;
        let out_port = Self::select_output_port(&midi_out, &config.midi_output_port)?;
        let out_port_name = midi_out.port_name(&out_port)?;

        info!("Opening output connection to: {}", out_port_name);
        let conn_out = midi_out.connect(&out_port, "push2-output-connection")?;

        Ok(MidiHandler { _conn_in, conn_out })
    }

    /// Finds the configured input port, or falls back to manual selection.

fn select_input_port(
        midi_in: &MidiInput,
        config_port_name: &str,
    ) -> Result<MidiInputPort, MidiHandlerError> {
        let in_ports = midi_in.ports();
        
        // 1. Try to find exact port from config
        for port in &in_ports {
            let name = midi_in.port_name(port)?;
            if name == config_port_name {
                info!("Found configured input port: {}", name);
                return Ok(port.clone());
            }
        }

        // 2. Try partial match using the config string (handles OS port suffixes like ' 28:0')
        for port in &in_ports {
            let name = midi_in.port_name(port)?;
            // Bidirectional check: if the OS name contains the config name, or vice versa
            if name.contains(config_port_name) || config_port_name.contains(&name) {
                info!("Auto-detected input port via config substring: {}", name);
                return Ok(port.clone());
            }
        }

        // Configured port not found, fall back to manual logic
        warn!(
            "Configured input port '{}' not found. Falling back to manual selection.",
            config_port_name
        );

        match in_ports.len() {
            0 => Err(MidiHandlerError::NoInputPorts),
            1 => {
                info!(
                    "Choosing the only available input port: {}",
                    midi_in.port_name(&in_ports[0])?
                );
                Ok(in_ports[0].clone())
            }
            _ => {
                println!("\nAvailable input ports:");
                for (i, port) in in_ports.iter().enumerate() {
                    println!("{}: {}", i, midi_in.port_name(port)?);
                }
                print!("Please select port for Ableton Push 2 INPUT: ");
                stdout().flush()?;
                let mut input = String::new();
                stdin().read_line(&mut input)?;
                let port_index: usize = input.trim().parse()?;
                in_ports
                    .get(port_index)
                    .cloned()
                    .ok_or(MidiHandlerError::InvalidInputPortIndex)
            }
        }
    }

    fn select_output_port(
        midi_out: &MidiOutput,
        config_port_name: &str,
    ) -> Result<MidiOutputPort, MidiHandlerError> {
        let out_ports = midi_out.ports();
        
        // 1. Try to find exact port from config
        for port in &out_ports {
            let name = midi_out.port_name(port)?;
            if name == config_port_name {
                info!("Found configured output port: {}", name);
                return Ok(port.clone());
            }
        }

        // 2. Try partial match using the config string
        for port in &out_ports {
            let name = midi_out.port_name(port)?;
            if name.contains(config_port_name) || config_port_name.contains(&name) {
                info!("Auto-detected output port via config substring: {}", name);
                return Ok(port.clone());
            }
        }

        // Configured port not found, fall back to manual logic
        warn!(
            "Configured output port '{}' not found. Falling back to manual selection.",
            config_port_name
        );

        match out_ports.len() {
            0 => Err(MidiHandlerError::NoOutputPorts),
            1 => {
                info!(
                    "Choosing the only available output port: {}",
                    midi_out.port_name(&out_ports[0])?
                );
                Ok(out_ports[0].clone())
            }
            _ => {
                println!("\nAvailable output ports:");
                for (i, port) in out_ports.iter().enumerate() {
                    println!("{}: {}", i, midi_out.port_name(port)?);
                }
                print!("Please select port for Ableton Push 2 OUTPUT: ");
                stdout().flush()?;
                let mut input = String::new();
                stdin().read_line(&mut input)?;
                let port_index: usize = input.trim().parse()?;
                out_ports
                    .get(port_index)
                    .cloned()
                    .ok_or(MidiHandlerError::InvalidOutputPortIndex)
            }
        }
    }
}
