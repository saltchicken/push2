use embedded_graphics_core::{
    Pixel,
    geometry::Size,
    pixelcolor::{Bgr565, IntoStorage},
    prelude::*,
};
use rusb::{Context, Device, DeviceDescriptor, DeviceHandle, UsbContext};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use thiserror::Error;

pub struct Push2Display {
    pub(crate) frame_buffer: Box<[u16]>,
    tx_frame: mpsc::SyncSender<Box<[u16]>>,
}

#[derive(Error, Debug)]
pub enum Push2DisplayError {
    #[error("Ableton Push2 Not found")]
    Push2NotFound,
    #[error(transparent)]
    USBError(#[from] rusb::Error),
    #[error("Failed to parse BMP image")]
    BmpParseError,
    #[error("USB transfer thread has disconnected")]
    TransferThreadDisconnected,
}

pub const DISPLAY_WIDTH: usize = 960;
pub const DISPLAY_HEIGHT: usize = 160;

const PUSH2_BULK_EP_OUT: u8 = 0x01;
const BYTES_PER_LINE: usize = 2048; // 960 * 2 + 128 filler
const PUSH_2_VENDOR_ID: u16 = 0x2982;
const PUSH_2_PRODUCT_ID: u16 = 0x1967;

/// Frame header required by the Push 2 to signify the start of a new frame transfer.
const HEADER: [u8; 16] = [
    0xff, 0xcc, 0xaa, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

/// XOR Mask applied to frame buffer pixels to prevent the display controller 
/// from accidentally interpreting raw pixel data as a USB control command.
const MASK: [u8; 4] = [0xe7, 0xf3, 0xe7, 0xff];

impl Push2Display {
    /// Opens the Push2 display and starts the background USB transfer thread.
    pub fn new() -> Result<Push2Display, Push2DisplayError> {
        let mut context = Context::new()?;
        let (_, _, handle) = open_device(&mut context, PUSH_2_VENDOR_ID, PUSH_2_PRODUCT_ID)
            .ok_or(Push2DisplayError::Push2NotFound)?;

        handle.claim_interface(0)?;

        let (tx_frame, rx_frame) = mpsc::sync_channel::<Box<[u16]>>(2);

        // Spawn a background thread to handle USB I/O.
        // It acts as a keep-alive loop: the Push 2 screen blanks if it doesn't receive data for ~2s.
        thread::spawn(move || {
            let mut transfer_buffer = vec![0u8; BYTES_PER_LINE * DISPLAY_HEIGHT];
            let usb_timeout = Duration::from_millis(500);
            let frame_interval = Duration::from_millis(16); // ~60 FPS
            let mut has_frame = false;

            loop {
                // Wait for a new frame, but timeout if the app is idle
                match rx_frame.recv_timeout(frame_interval) {
                    Ok(frame) => {
                        has_frame = true;
                        for r in 0..DISPLAY_HEIGHT {
                            for c in 0..DISPLAY_WIDTH {
                                let i = r * DISPLAY_WIDTH + c;
                                let b: [u8; 2] = u16::to_le_bytes(frame[i]);
                                let di = r * BYTES_PER_LINE + c * 2;

                                transfer_buffer[di] = b[0] ^ MASK[di % 4];
                                transfer_buffer[di + 1] = b[1] ^ MASK[(di + 1) % 4];
                            }
                        }
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => {
                        // App is idle (no new flush). We just fall through and resend the existing buffer
                    }
                    Err(mpsc::RecvTimeoutError::Disconnected) => {
                        // The main app shut down
                        break;
                    }
                }

                if has_frame {
                    // Write the frame header followed by the masked frame data
                    let _ = handle.write_bulk(PUSH2_BULK_EP_OUT, &HEADER, usb_timeout);
                    let _ = handle.write_bulk(PUSH2_BULK_EP_OUT, &transfer_buffer, usb_timeout);
                }
            }
        });

        let frame_buffer = vec![0; DISPLAY_WIDTH * DISPLAY_HEIGHT].into_boxed_slice();

        Ok(Push2Display {
            frame_buffer,
            tx_frame,
        })
    }

    /// Sends the current frame buffer to the USB transfer thread.
    pub fn flush(&mut self) -> Result<(), Push2DisplayError> {
        self.tx_frame
            .send(self.frame_buffer.clone())
            .map_err(|_| Push2DisplayError::TransferThreadDisconnected)?;
        Ok(())
    }
}

impl DrawTarget for Push2Display {
    type Color = Bgr565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels.into_iter() {
            if let Ok((x @ 0..=959, y @ 0..=159)) = point.try_into() {
                let index: u32 = x + y * 960;
                self.frame_buffer[index as usize] = color.into_storage();
            }
        }
        Ok(())
    }
}

impl OriginDimensions for Push2Display {
    fn size(&self) -> Size {
        Size::new(DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32)
    }
}

fn open_device<T: UsbContext>(
    context: &mut T,
    vid: u16,
    pid: u16,
) -> Option<(Device<T>, DeviceDescriptor, DeviceHandle<T>)> {
    let devices = context.devices().ok()?;
    for device in devices.iter() {
        if let Ok(device_desc) = device.device_descriptor() {
            if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
                if let Ok(handle) = device.open() {
                    return Some((device, device_desc, handle));
                }
            }
        }
    }
    None
}
