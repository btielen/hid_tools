use std::fs;
use std::io::Read;
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;
use nix::ioctl_read;

//https://github.com/torvalds/linux/blob/master/include/uapi/linux/hidraw.h
ioctl_read!(hid_read_descr_size, b'H', 0x01, u32);
ioctl_read!(hid_read_descriptor, b'H', 0x02, ReportDescriptor);
ioctl_read!(hid_read_device_info, b'H', 0x03, DeviceInfo);

#[derive(Debug)]
pub struct ReportDescriptor {
    size: u32,
    raw: [u8; 4096]
}

impl ReportDescriptor {
    pub fn data(&self) -> &[u8] {
        &self.raw[0..self.size as usize]
    }
}

#[derive(Debug)]
pub struct DeviceInfo {
    bus_type: u32,
    pub vendor_id: i16,
    pub product_id: i16
}

fn main() {

    let mut options = fs::OpenOptions::new();
    options.read(true);

    let mut file = options.open(PathBuf::from("/dev/hidraw4")).unwrap();
    let mut desc_size: u32 = 3;

    // Get Report Descriptor size from HIDRAW API
    unsafe { hid_read_descr_size(file.as_raw_fd(), &mut desc_size);};

    let mut report = ReportDescriptor {
        size: desc_size,
        raw: [0; 4096]
    };

    let mut device_info = DeviceInfo {
        bus_type: 0,
        vendor_id: 0,
        product_id: 0
    };

    // Get information from device
    unsafe {
        hid_read_descriptor(file.as_raw_fd(), &mut report);
        hid_read_device_info(file.as_raw_fd(), &mut device_info);
    };


    dbg!(desc_size);
    dbg!(&report);

    println!("Vendor id: {:#04x} Product id: {:#04x}", device_info.vendor_id, device_info.product_id);

    let parsed = hid_tools::parse::report_descriptor(report.data()).unwrap();

    println!("{}", parsed);

    let mut data: [u8; 4096] = [0; 4096];
    loop {

        match file.read(&mut data) {
            Ok(size) => {
                println!("Event Report: {:?}", &data[0..size])
            }
            Err(_) => {
                println!("Could not read data");
                break;
            }
        }
    }
}