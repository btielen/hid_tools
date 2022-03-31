use std::io;
use std::path::PathBuf;
use std::slice::Iter;

pub struct Device {
    report_descriptor_path: PathBuf,
}

pub struct DeviceList {
    pub devices: Vec<Device>,
}

impl Device {
    pub fn new(report_descriptor_path: PathBuf) -> Self {
        Device {
            report_descriptor_path,
        }
    }

    pub fn report_descriptor_bytes(&self) -> io::Result<Vec<u8>> {
        std::fs::read(&self.report_descriptor_path)
    }
}

impl DeviceList {
    pub fn iter(&self) -> Iter<'_, Device> {
        self.devices.iter()
    }
}
