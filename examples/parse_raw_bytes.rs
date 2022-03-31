use hid_tools::parse;
use nom::Finish;

fn main() {
    let bytes = [
        0x05, 0x01, // Usage Page (Generic Desktop)
        0x09, 0x06, // Usage (Keyboard)
        0xa1, 0x01, // Collection (Application)
        0x85, 0x01, // .Report ID (1)
        0x05, 0x07, // .Usage Page (Keyboard)
        0x19, 0xe0, // .Usage Minimum (224)
        0x29, 0xe7, // .Usage Maximum (231)
        0x15, 0x00, // .Logical Minimum (0)
        0x25, 0x01, // .Logical Maximum (1)
        0x75, 0x01, // .Report Size (1)
        0x95, 0x08, // .Report Count (8)
        0x81, 0x02, // .Input (Data,Var,Abs)
        0x19, 0x01, // .Usage Minimum (1)
        0x29, 0x97, // .Usage Maximum (151)
        0x15, 0x00, // .Logical Minimum (0)
        0x25, 0x01, // .Logical Maximum (1)
        0x75, 0x01, // .Report Size (1)
        0x95, 0x98, // .Report Count (152)
        0x81, 0x02, // .Input (Data,Var,Abs)
        0xc0, // End Collection
    ];

    let parsed = parse::report_descriptor(&bytes).unwrap();
    println!("{}", parsed);
}
