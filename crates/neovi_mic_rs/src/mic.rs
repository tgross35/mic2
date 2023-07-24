use crate::types::Result;
use rusb::{self, GlobalContext};

#[derive(Debug, Clone, PartialEq)]
pub enum UsbDeviceType {
    MicrochipHub,
    FT245R,
    FT245RAccessDenied,
    GPS,
    Audio,
    Unknown,
}

impl Default for UsbDeviceType {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UsbDeviceInfo {
    pub vendor_id: u16,
    pub product_id: u16,
    pub bus_number: u8,
    pub address: u8,
    pub device_type: UsbDeviceType,
    pub serial_number: Option<String>,
}

impl UsbDeviceInfo {
    pub fn from_rusb_device(device: &rusb::Device<GlobalContext>) -> Self {
        let device_desc = device.device_descriptor().unwrap();
        let vendor_id = device_desc.vendor_id();
        let product_id = device_desc.product_id();
        Self {
            vendor_id,
            product_id,
            bus_number: device.bus_number(),
            address: device.address(),
            device_type: Self::usb_device_type_from_vid_pid(&vendor_id, &product_id),
            serial_number: None,
        }
    }

    fn usb_device_type_from_vid_pid(vid: &u16, pid: &u16) -> UsbDeviceType {
        match (vid, pid) {
            (0x0424, 0x2514) => UsbDeviceType::MicrochipHub,
            (0x93C, 0x601) => UsbDeviceType::FT245R,
            (0x8BB, 0x2912) => UsbDeviceType::Audio,
            (0x1546, 0x1A8) => UsbDeviceType::GPS,
            _ => UsbDeviceType::Unknown,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct NeoVIMIC {
    usb_hub: UsbDeviceInfo,
    usb_children: Vec<UsbDeviceInfo>, /*
                                      /// Serial number of the neoVI MIC, MCxxxx
                                      serial_number: String,
                                      /// Serial port identifier for the GPS functionality. Some devices
                                      /// don't have GPS so this is optional.
                                      gps_serial_port: Option<String>,
                                      /// Audio Capture device name attached to the neoVI MIC.
                                      audio_name: String,
                                      /// Index of the FTDI device attached to the neoVI MIC. This is for IO
                                      /// control of the device (button, speaker, and LEDs).
                                      ftdi_index: u32,
                                      */
}

pub fn find_neovi_mics() -> Result<Vec<NeoVIMIC>> {
    let mut usb_hubs = Vec::new();

    // Find all potential neoVI MIC2 USB hubs
    // 0424:2514 Microchip Technology, Inc. (formerly SMSC) USB 2.0 Hub
    for rusb_device in rusb::devices().unwrap().iter() {
        let device = UsbDeviceInfo::from_rusb_device(&rusb_device);
        // Are we the hub? 0424:2514 Microchip Technology, Inc. (formerly SMSC) USB 2.0 Hub
        if device.vendor_id == 0x0424 || device.product_id == 0x2514 {
            usb_hubs.push(device);
        }
    }

    let mut devices = Vec::new();
    // Find all children attached to all the hubs
    for usb_hub in usb_hubs {
        let mut usb_children = Vec::new();
        for device in rusb::devices().unwrap().iter() {
            let parent = device.get_parent();
            if parent.is_none() {
                continue;
            }
            let parent = UsbDeviceInfo::from_rusb_device(&parent.unwrap());
            if parent == usb_hub {
                let mut child: UsbDeviceInfo = UsbDeviceInfo::from_rusb_device(&device);
                // Lets attempt to open the device and get the serial number
                if child.device_type == UsbDeviceType::FT245R {
                    let serial_number = match &device.open() {
                        Ok(handle) => handle
                            .read_serial_number_string_ascii(&device.device_descriptor().unwrap())
                            .unwrap(),
                        Err(e) => {
                            // Probably an access denied error, udev rules correct?
                            format!("{e}").into()
                        },
                    };
                    child = UsbDeviceInfo {
                        serial_number: Some(serial_number.into()),
                        ..child
                    };
                }
                usb_children.push(child);
            }
        }
        devices.push(NeoVIMIC {
            usb_hub,
            usb_children,
        });
    }
    Ok(devices)
}

impl NeoVIMIC {
    pub fn new(_index: u32) -> Result<Self> {
        Ok(Self {
            ..Default::default()
        })
    }

    pub fn from_serial_number(serial_number: impl Into<String>) -> Result<Self> {
        Ok(Self {
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_neovi_mics() {
        let devices = find_neovi_mics().expect("Expected at least one neoVI MIC2!");
        println!("{devices:#X?}")
    }
}
