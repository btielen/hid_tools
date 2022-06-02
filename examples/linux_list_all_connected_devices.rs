use hid_tools::report_descriptor::parse;
use std::path::PathBuf;

pub fn list_devices() -> std::io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    for entry in std::fs::read_dir("/sys/class/hidraw")? {
        let entry = entry?;
        let mut path = entry.path();

        if path.is_dir() {
            path.push("device/report_descriptor");

            paths.push(path);
        }
    }

    Ok(paths)
}

fn main() {
    for report_descriptor_path in list_devices().unwrap().iter() {
        let report_bytes = std::fs::read(report_descriptor_path).unwrap();

        println!("{}", parse::report_descriptor(&report_bytes).unwrap())
    }
}
