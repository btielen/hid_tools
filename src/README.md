# HID-Tools

First attempt at parsing a raw HID (Human Interface Device) Report Descriptor 
and displaying it in user-friendly way.

## Getting started

Following example prints every connected HID device on a linux-system.

```rust
fn main() {
    for device in hid_tools::linux::list_devices() {
        println!("{}", device.get_report_descriptor())
    }
}
```


### Example Output
```
Usage Page (Generic Desktop Controls) 
Usage (Keyboard)
Collection (1) 
    Report ID (1) 
    Usage Page (Keyboard) 
    Usage Minimum (224) 
    Usage Maximum (231) 
    Logical Minimum (0) 
    Logical Maximum (1) 
    Report Size (1) 
    Report Count (8) 
    Input (2) 
    Usage Minimum (0) 
    Usage Maximum (151) 
    Logical Minimum (0) 
    Logical Maximum (1) 
    Report Size (1) 
    Report Count (152) 
    Input (2) 
End Collection (0) 
```

## Todo

- Test negative numeric payloads (`Logical Minimum (-256)`)
- Display Usage minimum/maximum (eg `USAGE_MINIMUM (Button 1)`)
- Display Collection (eg. `COLLECTION (Physical)`)
- Display Input (`INPUT (Data,Var,Rel)`)
- Examples
- ReportDescriptorBuilder
- Convert more Usage tables (help wanted)


## Resources

- [Device Class Definition for HID 1.11](https://www.usb.org/sites/default/files/hid1_11.pdf)
- [HID Usage tables](https://usb.org/sites/default/files/hut1_3_0.pdf)

