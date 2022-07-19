use std::fmt::{Display, Formatter};
/**

This examples reads raw report & report descriptor data from the linux kernel using the
HID-RAW API. The `nix` crate bridges this example and the kernel.

## Setup permissions

The following line was placed in a .rules file (I named mine 99-hidraw-permissions.rules) located under /etc/udev/rules.d
KERNEL=="hidraw*", SUBSYSTEM=="hidraw", MODE="0664", GROUP="plugdev"

Restart your computer afterwards

*/
use hid_tools::report::{expected_input_reports, parse_raw_input_report};
use nix::ioctl_read;
use std::fs;
use std::io::Read;
use std::os::unix::io::AsRawFd;
use std::path::Path;

// Change the path for your device
const HID_DEVICE_PATH: &str = "/dev/hidraw3";

// Create functions to read data from kernel
// For declarations of the SPI_IOC_MAGIC and SPI_IOC_TYPE_MODE, see
// https://github.com/torvalds/linux/blob/master/include/uapi/linux/hidraw.h
ioctl_read!(hid_read_report_descriptor_size, b'H', 0x01, u32);
ioctl_read!(hid_read_report_descriptor, b'H', 0x02, ReportDescriptor);
ioctl_read!(hid_read_device_info, b'H', 0x03, DeviceInfo);

#[derive(Debug)]
pub struct ReportDescriptor {
    size: u32,
    raw: [u8; 4096],
}

impl ReportDescriptor {
    fn new(size: u32, raw: [u8; 4096]) -> Self {
        ReportDescriptor {
            size,
            raw
        }
    }
}

impl ReportDescriptor {
    pub fn data(&self) -> &[u8] {
        &self.raw[0..self.size as usize]
    }
}

#[derive(Default)]
pub struct DeviceInfo {
    pub bus_type: u32,
    pub vendor_id: i16,
    pub product_id: i16,
}

impl Display for DeviceInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vendor id: {:#04x} Product id: {:#04x} Bus type: {}", self.vendor_id, self.product_id, self.bus_type)
    }
}

fn main() {
    let mut options = fs::OpenOptions::new();
    options.read(true);

    let mut file = options.open(Path::new(HID_DEVICE_PATH)).unwrap();
    let mut report_descriptor_size: u32 = 0;

    // Get Report Descriptor size from HID-RAW API
    unsafe {
        hid_read_report_descriptor_size(file.as_raw_fd(), &mut report_descriptor_size).unwrap();
    };

    let mut report = ReportDescriptor::new(report_descriptor_size, [0; 4096]);
    let mut device_info = DeviceInfo::default();

    // Get report descriptor and device info from HID-RAW API
    unsafe {
        hid_read_report_descriptor(file.as_raw_fd(), &mut report).unwrap();
        hid_read_device_info(file.as_raw_fd(), &mut device_info).unwrap();
    };

    println!("{}", device_info);

    let parsed = hid_tools::report_descriptor::parse::report_descriptor(report.data()).unwrap();
    println!("{}", parsed);

    let expected_reports = expected_input_reports(&parsed).unwrap();

    let mut data: [u8; 4096] = [0; 4096];
    loop {
        match file.read(&mut data) {
            Ok(size) => {
                println!("Event Report: {:?}", &data[0..size]);
                let parsed = parse_raw_input_report(&data[0..size], &expected_reports).unwrap();
                println!("{}", parsed);
            }
            Err(_) => {
                println!("Could not read data");
                break;
            }
        }
    }
}
