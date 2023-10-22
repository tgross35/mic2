use std::sync::{Arc, Mutex};

use neovi_mic_rs::io;
use neovi_mic_rs::mic;
use pyo3::prelude::*;

macro_rules! define_basic_py_object {
    ($name:ident, $inner_name:ty) => {
        #[pyclass]
        #[derive(Debug)]
        #[repr(transparent)]
        pub struct $name(pub Arc<Mutex<$inner_name>>);

        // Arc is only Send if T is Send so lets mark it as safe here
        unsafe impl Send for $name {}

        #[pymethods]
        impl $name {
            #[new]
            fn py_new() -> Self {
                Self::new()
            }
        }
    };
}

macro_rules! define_basic_py_object_no_new {
    ($name:ident, $inner_name:ty) => {
        #[pyclass]
        #[derive(Debug, Default, Clone)]
        #[repr(transparent)]
        pub struct $name(pub Arc<Mutex<$inner_name>>);

        // Arc is only Send if T is Send so lets mark it as safe here
        unsafe impl Send for $name {}
    };
}

define_basic_py_object_no_new!(NeoVIMIC, mic::NeoVIMIC);
#[pymethods]
impl NeoVIMIC {
    #[new]
    fn py_new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn __str__(&self) -> String {
        let serial = self.0.lock().unwrap().get_serial_number();
        format!("NeoVI MIC2 {serial}").to_string()
    }

    fn __repr__(&self) -> String {
        let description = self.__str__();
        format!("<NeoVI MIC2 {description}").to_string()
    }

    fn get_serial_number(&self) -> PyResult<String> {
        Ok(self.0.lock().unwrap().get_serial_number())
    }

    fn has_gps(&self) -> PyResult<bool> {
        Ok(self.0.lock().unwrap().has_gps())
    }

    fn get_ftdi_device(&self) -> PyResult<UsbDeviceInfo> {
        Ok(UsbDeviceInfo::from(
            self.0.lock().unwrap().get_ftdi_device().unwrap(),
        ))
    }

    fn get_io_device(&self) -> PyResult<IODevice> {
        Ok(IODevice::from(
            self.0.lock().unwrap().get_io_device().unwrap(),
        ))
    }
}

impl NeoVIMIC {
    /* TODO
    fn new() -> Self {
        Self {
            0: Arc::new(Mutex::new(mic::NeoVIMIC { ..Default::default() })),
        }
    }
     */
    pub fn from(neovi_mic: mic::NeoVIMIC) -> Self {
        Self {
            0: Arc::new(Mutex::new(neovi_mic)),
        }
    }
}

define_basic_py_object_no_new!(UsbDeviceInfo, mic::UsbDeviceInfo);
#[pymethods]
impl UsbDeviceInfo {
    #[new]
    fn py_new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn __str__(&self) -> String {
        let serial = match &self.0.lock().unwrap().serial_number {
            Some(s) => s.clone(),
            None => "None".to_string(),
        };
        format!("NeoVI MIC2 {serial}").to_string()
    }

    fn __repr__(&self) -> String {
        let description = self.__str__();
        format!("<NeoVI MIC2 {description}").to_string()
    }

    #[getter]
    fn vendor_id(&self) -> PyResult<u16> {
        Ok(self.0.lock().unwrap().vendor_id)
    }

    #[getter]
    fn product_id(&self) -> PyResult<u16> {
        Ok(self.0.lock().unwrap().product_id)
    }

    #[getter]
    fn bus_number(&self) -> PyResult<u8> {
        Ok(self.0.lock().unwrap().bus_number)
    }

    #[getter]
    fn address(&self) -> PyResult<u8> {
        Ok(self.0.lock().unwrap().address)
    }

    /* TODO
    #[getter]
    fn device_type(&self) -> PyResult<u32> {
        Ok(self.0.lock().unwrap().device_type.into())
    }
    */
}

impl UsbDeviceInfo {
    /* TODO
    fn new() -> Self {
        Self {
            0: Arc::new(Mutex::new(mic::UsbDeviceInfo { ..Default::default() })),
        }
    }
     */
    pub fn from(usb_device_info: &mic::UsbDeviceInfo) -> Self {
        Self {
            0: Arc::new(Mutex::new(usb_device_info.to_owned())),
        }
    }
}

#[pyclass(name = "IODeviceBitMode")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PyIODeviceBitMode {
    None = 0x0,
    Buzzer = 0x1,
    Button = 0x2,
    GPSLed = 0x4,
    CBUS3 = 0x8,
    BuzzerMask = 0x10,
    ButtonMask = 0x20,
    GPSLedMask = 0x40,
    CBUS3Mask = 0x80,

    DefaultMask = 0x50,
}

impl TryFrom<io::IODeviceBitMode> for PyIODeviceBitMode {
    type Error = &'static str;

    fn try_from(value: io::IODeviceBitMode) -> Result<Self, Self::Error> {
        let value = match value {
            io::IODeviceBitMode::None => Ok(PyIODeviceBitMode::None),
            io::IODeviceBitMode::Buzzer => Ok(PyIODeviceBitMode::Buzzer),
            io::IODeviceBitMode::Button => Ok(PyIODeviceBitMode::Button),
            io::IODeviceBitMode::GPSLed => Ok(PyIODeviceBitMode::GPSLed),
            io::IODeviceBitMode::CBUS3 => Ok(PyIODeviceBitMode::CBUS3),
            io::IODeviceBitMode::BuzzerMask => Ok(PyIODeviceBitMode::BuzzerMask),
            io::IODeviceBitMode::ButtonMask => Ok(PyIODeviceBitMode::ButtonMask),
            io::IODeviceBitMode::DefaultMask => Ok(PyIODeviceBitMode::DefaultMask),
            _ => Err("Invalid IODeviceBitMode!"),
        };
        value
    }
}

impl TryFrom<u8> for PyIODeviceBitMode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let bit_mode = io::IODeviceBitMode::from_bits(value).unwrap();
        bit_mode.try_into()
    }
}

define_basic_py_object_no_new!(IODevice, io::IODevice);
#[pymethods]
impl IODevice {
    #[new]
    fn py_new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn __str__(&self) -> String {
        let serial = match &self.0.lock().unwrap().get_usb_device_info().serial_number {
            Some(s) => s.clone(),
            None => "None".to_string(),
        };
        format!("NeoVI MIC2 {serial}").to_string()
    }

    fn __repr__(&self) -> String {
        let description = self.__str__();
        format!("<NeoVI MIC2 {description}").to_string()
    }

    /// Check if the device is open.
    fn is_open(&self) -> PyResult<bool> {
        Ok(self.0.lock().unwrap().is_open())
    }

    /// Open the device.
    fn open(&self) -> PyResult<()> {
        Ok(self.0.lock().unwrap().open().unwrap())
    }

    /// Open the device.
    fn close(&self) -> PyResult<()> {
        Ok(self.0.lock().unwrap().close().unwrap())
    }

    /// Enable/disable bitbang modes.
    /// bitmask	Bitmask to configure lines. HIGH/ON value configures a line as output.
    /// mode	Bitbang mode: use the values defined in ftdi_mpsse_mode
    ///
    /// CBUS0 = Buzzer
    /// CBUS1 = Button
    /// CBUS2 = GPS LED
    /// CBUS3 = N/C
    ///
    fn set_bitmode_raw(&self, bitmask: u8) -> PyResult<()> {
        Ok(self.0.lock().unwrap().set_bitmode_raw(bitmask).unwrap())
    }

    fn set_bitmode(&self, bitmask: PyIODeviceBitMode) -> PyResult<()> {
        Ok(self
            .0
            .lock()
            .unwrap()
            .set_bitmode(io::IODeviceBitMode::from_bits(bitmask as u8).unwrap())
            .unwrap())
    }

    /// Directly read pin state, circumventing the read buffer. Useful for bitbang mode.
    ///
    /// CBUS0 = Buzzer
    /// CBUS1 = Button
    /// CBUS2 = GPS LED
    /// CBUS3 = N/C
    ///
    fn read_pins_raw(&self) -> PyResult<u8> {
        Ok(self.0.lock().unwrap().read_pins_raw().unwrap())
    }

    fn read_pins(&self) -> PyResult<PyIODeviceBitMode> {
        Ok(
            PyIODeviceBitMode::try_from(self.0.lock().unwrap().read_pins().unwrap().bits())
                .unwrap(),
        )
    }

    fn get_usb_device_info(&self) -> PyResult<UsbDeviceInfo> {
        Ok(UsbDeviceInfo::from(
            self.0.lock().unwrap().get_usb_device_info(),
        ))
    }
}

impl IODevice {
    /* TODO
    fn new() -> Self {
        Self {
            0: Arc::new(Mutex::new(mic::UsbDeviceInfo { ..Default::default() })),
        }
    }
     */
    pub fn from(io_device: io::IODevice) -> Self {
        Self {
            0: Arc::new(Mutex::new(io_device)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_io_device_bit_mode() {
        assert_eq!(
            PyIODeviceBitMode::None as u8,
            io::IODeviceBitMode::None.bits() as u8
        );
        assert_eq!(
            PyIODeviceBitMode::Buzzer as u8,
            io::IODeviceBitMode::Buzzer.bits() as u8
        );
        assert_eq!(
            PyIODeviceBitMode::Button as u8,
            io::IODeviceBitMode::Button.bits() as u8
        );
        assert_eq!(
            PyIODeviceBitMode::GPSLed as u8,
            io::IODeviceBitMode::GPSLed.bits() as u8
        );
        assert_eq!(
            PyIODeviceBitMode::CBUS3 as u8,
            io::IODeviceBitMode::CBUS3.bits() as u8
        );
        assert_eq!(
            PyIODeviceBitMode::BuzzerMask as u8,
            io::IODeviceBitMode::BuzzerMask.bits() as u8
        );
        assert_eq!(
            PyIODeviceBitMode::ButtonMask as u8,
            io::IODeviceBitMode::ButtonMask.bits() as u8
        );
        assert_eq!(
            PyIODeviceBitMode::GPSLedMask as u8,
            io::IODeviceBitMode::GPSLedMask.bits() as u8
        );
        assert_eq!(
            PyIODeviceBitMode::CBUS3Mask as u8,
            io::IODeviceBitMode::CBUS3Mask.bits() as u8
        );
        assert_eq!(
            PyIODeviceBitMode::DefaultMask as u8,
            io::IODeviceBitMode::DefaultMask.bits() as u8
        );
    }
}