pub mod usb;
pub mod utils;

use std::sync::{Arc, Mutex};

use pyo3::prelude::*;

use usb::PyUsbDeviceInfo;
use neovi_mic_rs::mic::{find_neovi_mics, NeoVIMIC};

use crate::utils::create_python_object;

#[pyfunction]
fn find() -> PyResult<Vec<PyNeoVIMIC>> {
    let devices = find_neovi_mics()
        .unwrap()
        .into_iter()
        .map(|x| PyNeoVIMIC::from(x))
        .collect::<Vec<PyNeoVIMIC>>();
    Ok(devices)
}


create_python_object!(PyNeoVIMIC, "NeoVIMIC", NeoVIMIC);
#[pymethods]
impl PyNeoVIMIC {
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

    fn io_open(&self) -> PyResult<()> {
        Ok(self.0.lock().unwrap().io_open().unwrap())
    }

    fn io_close(&self) -> PyResult<()> {
        Ok(self.0.lock().unwrap().io_close().unwrap())
    }

    fn io_is_open(&self) -> PyResult<bool> {
        Ok(self.0.lock().unwrap().io_is_open().unwrap())
    }

    fn io_buzzer_enable(&self, enabled: bool) -> PyResult<()> {
        Ok(self.0.lock().unwrap().io_buzzer_enable(enabled).unwrap())
    }

    fn io_buzzer_is_enabled(&self) -> PyResult<bool> {
        Ok(self.0.lock().unwrap().io_buzzer_is_enabled().unwrap())
    }

    fn io_gpsled_enable(&self, enabled: bool) -> PyResult<()> {
        Ok(self.0.lock().unwrap().io_gpsled_enable(enabled).unwrap())
    }

    fn io_gpsled_is_enabled(&self) -> PyResult<bool> {
        Ok(self.0.lock().unwrap().io_gpsled_is_enabled().unwrap())
    }

    fn io_button_is_pressed(&self) -> PyResult<bool> {
        Ok(self.0.lock().unwrap().io_button_is_pressed().unwrap())
    }
}

impl PyNeoVIMIC {
    pub fn from(neovi_mic: NeoVIMIC) -> Self {
        Self {
            0: Arc::new(Mutex::new(neovi_mic)),
        }
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn neovi_mic(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyNeoVIMIC>()?;
    m.add_class::<PyUsbDeviceInfo>()?;
    m.add_function(wrap_pyfunction!(find, m)?)?;
    Ok(())
}
