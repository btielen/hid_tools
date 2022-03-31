use crate::device::{Device, DeviceList};

pub fn list_devices() -> std::io::Result<DeviceList> {
    let mut devices = Vec::new();

    for entry in std::fs::read_dir("/sys/class/hidraw")? {
        let entry = entry?;
        let mut path = entry.path();

        if path.is_dir() {
            path.push("device/report_descriptor");

            devices.push(Device::new(path));
        }
    }

    Ok(DeviceList { devices })
}
