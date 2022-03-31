use hid_tools::linux;
use hid_tools::parse;

fn main() {
    for device in linux::list_devices().unwrap().iter() {
        let report_bytes = device.report_descriptor_bytes().unwrap();

        println!("{}", parse::report_descriptor(&report_bytes).unwrap())
    }
}
