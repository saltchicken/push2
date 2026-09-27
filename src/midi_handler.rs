use crate::app_config::AppConfig;
use log::{info, warn};
use midir::{
    ConnectError, Ignore, InitError, MidiInput, MidiInputConnection, MidiInputPort, MidiOutput,
    MidiOutputConnection, MidiOutputPort, PortInfoError,
};
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
    #[error("Port not found: {0}")]
    PortNotFound(String),
}

/// Holds the MIDI connections.
pub struct MidiHandler {
    pub _conn_in: MidiInputConnection<()>,
    pub conn_out: MidiOutputConnection,
}

impl MidiHandler {
    /// Creates a new MidiHandler, finds and connects to ports.
    pub fn new(config: &AppConfig, tx: Sender<Vec<u8>>) -> Result<Self, MidiHandlerError> {
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

        let midi_out = MidiOutput::new("push2_output").map_err(MidiHandlerError::OutputInit)?;
        let out_port = Self::select_output_port(&midi_out, &config.midi_output_port)?;
        let out_port_name = midi_out.port_name(&out_port)?;

        info!("Opening output connection to: {}", out_port_name);
        let conn_out = midi_out.connect(&out_port, "push2-output-connection")?;

        Ok(MidiHandler { _conn_in, conn_out })
    }

    fn select_input_port(
        midi_in: &MidiInput,
        config_port_name: &str,
    ) -> Result<MidiInputPort, MidiHandlerError> {
        let in_ports = midi_in.ports();
        
        for port in &in_ports {
            let name = midi_in.port_name(port)?;
            if name == config_port_name || name.contains(config_port_name) || config_port_name.contains(&name) {
                info!("Found input port: {}", name);
                return Ok(port.clone());
            }
        }

        warn!("Configured input port '{}' not found.", config_port_name);
        if in_ports.len() == 1 {
            return Ok(in_ports[0].clone());
        }
        
        Err(MidiHandlerError::PortNotFound(format!("Input port '{}' not found", config_port_name)))
    }

    fn select_output_port(
        midi_out: &MidiOutput,
        config_port_name: &str,
    ) -> Result<MidiOutputPort, MidiHandlerError> {
        let out_ports = midi_out.ports();
        
        for port in &out_ports {
            let name = midi_out.port_name(port)?;
            if name == config_port_name || name.contains(config_port_name) || config_port_name.contains(&name) {
                info!("Found output port: {}", name);
                return Ok(port.clone());
            }
        }

        warn!("Configured output port '{}' not found.", config_port_name);
        if out_ports.len() == 1 {
            return Ok(out_ports[0].clone());
        }

        Err(MidiHandlerError::PortNotFound(format!("Output port '{}' not found", config_port_name)))
    }
}
