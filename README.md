# HID-Tools

First attempt at parsing a raw HID (Human Interface Device) Report Descriptor 
and displaying it in user-friendly way.

## Getting started

Following example prints every connected HID device on a linux-system.

```rust
use hid_tools::linux;
use hid_tools::parse;

fn main() {
    for device in linux::list_devices().unwrap().iter() {
        let report_bytes = device.report_descriptor_bytes().unwrap();

        println!("{}", parse::report_descriptor(&report_bytes).unwrap())
    }
}
```


### Example Output
Following is an example output for a 2-factor authentication usb-stick.
```
[06, d0, f1]        Usage Page (FIDO Alliance) 
[09, 01]            Usage (U2F Authenticator Device)
[a1, 01]            Collection (Application) 
[09, 20]                Usage (Input Report Data)
[15, 00]                Logical Minimum (0) 
[26, ff, 00]            Logical Maximum (255) 
[75, 08]                Report Size (8) 
[95, 40]                Report Count (64) 
[81, 02]                Input (2) 
[09, 21]                Usage (Output Report Data)
[15, 00]                Logical Minimum (0) 
[26, ff, 00]            Logical Maximum (255) 
[75, 08]                Report Size (8) 
[95, 40]                Report Count (64) 
[91, 02]                Output (2) 
[c0]                End Collection (0) 
```

### Build a raw HID Descriptor Report

```rust
use hid_tools::report_builder::ReportDescriptorBuilder;
use hid_tools::hid::Collection;
use hid_tools::usage_table::{UsagePage};
use hid_tools::usage_table::generic_desktop::GenericDesktopControlsUsage;

fn main() {
    let raw_report = ReportDescriptorBuilder::new()
        .usage_page(UsagePage::GenericDesktopControls)
        .usage(GenericDesktopControlsUsage::Mouse)
        .item(Collection::Application)
        // add more items here (see also examples)
        .end_collection()
        .build()
        .bytes();

    println!("{:02x?}", raw_report)
}
```

## Todo

- Display Input, Output, Feature (eg `INPUT (Data,Var,Rel)`)
- Convert more Usage tables (help wanted)


## Resources

- [Device Class Definition for HID 1.11](https://www.usb.org/sites/default/files/hid1_11.pdf)
- [HID Usage tables](https://usb.org/sites/default/files/hut1_3_0.pdf)

