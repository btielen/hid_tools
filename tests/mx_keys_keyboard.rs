use hid_tools::report::{expected_input_reports, parse_raw_input_report};
use hid_tools::report_descriptor::parse;

#[test]
fn main() {
    let report_descriptor: Vec<u8> = vec![
        0x05, 0x01, 0x09, 0x06, 0xa1, 0x01, 0x85, 0x01, 0x05, 0x07, 0x19, 0xe0, 0x29, 0xe7, 0x15,
        0x00, 0x25, 0x01, 0x75, 0x01, 0x95, 0x08, 0x81, 0x02, 0x95, 0x05, 0x05, 0x08, 0x19, 0x01,
        0x29, 0x05, 0x91, 0x02, 0x95, 0x01, 0x75, 0x03, 0x91, 0x01, 0x95, 0x06, 0x75, 0x08, 0x15,
        0x00, 0x26, 0xa4, 0x00, 0x05, 0x07, 0x19, 0x00, 0x2a, 0xa4, 0x00, 0x81, 0x00, 0xc0, 0x05,
        0x01, 0x09, 0x02, 0xa1, 0x01, 0x85, 0x02, 0x09, 0x01, 0xa1, 0x00, 0x95, 0x10, 0x75, 0x01,
        0x15, 0x00, 0x25, 0x01, 0x05, 0x09, 0x19, 0x01, 0x29, 0x10, 0x81, 0x02, 0x05, 0x01, 0x16,
        0x01, 0xf8, 0x26, 0xff, 0x07, 0x75, 0x0c, 0x95, 0x02, 0x09, 0x30, 0x09, 0x31, 0x81, 0x06,
        0x15, 0x81, 0x25, 0x7f, 0x75, 0x08, 0x95, 0x01, 0x09, 0x38, 0x81, 0x06, 0x95, 0x01, 0x05,
        0x0c, 0x0a, 0x38, 0x02, 0x81, 0x06, 0xc0, 0xc0, 0x05, 0x0c, 0x09, 0x01, 0xa1, 0x01, 0x85,
        0x03, 0x75, 0x10, 0x95, 0x02, 0x15, 0x01, 0x26, 0x8c, 0x02, 0x19, 0x01, 0x2a, 0x8c, 0x02,
        0x81, 0x60, 0xc0, 0x06, 0x43, 0xff, 0x0a, 0x02, 0x02, 0xa1, 0x01, 0x85, 0x11, 0x75, 0x08,
        0x95, 0x13, 0x15, 0x00, 0x26, 0xff, 0x00, 0x09, 0x02, 0x81, 0x00, 0x09, 0x02, 0x91, 0x00,
        0xc0,
    ];
    let report_descriptor = parse::report_descriptor(&report_descriptor).unwrap();

    // Test display report descriptor
    assert_eq!(
        format!("{}", report_descriptor),
        "[05, 01]            Usage Page (Generic Desktop Controls)
[09, 06]            Usage (Keyboard)
[a1, 01]            Collection (Application)
[85, 01]                Report ID (1)
[05, 07]                Usage Page (Keyboard)
[19, e0]                Usage Minimum (Keyboard Left Control)
[29, e7]                Usage Maximum (Keyboard Right GUI)
[15, 00]                Logical Minimum (0)
[25, 01]                Logical Maximum (1)
[75, 01]                Report Size (1)
[95, 08]                Report Count (8)
[81, 02]                Input (Data, Var, Abs)
[95, 05]                Report Count (5)
[05, 08]                Usage Page (LED)
[19, 01]                Usage Minimum (1)
[29, 05]                Usage Maximum (5)
[91, 02]                Output (Data, Var, Abs)
[95, 01]                Report Count (1)
[75, 03]                Report Size (3)
[91, 01]                Output (Const, Arr, Abs)
[95, 06]                Report Count (6)
[75, 08]                Report Size (8)
[15, 00]                Logical Minimum (0)
[26, a4, 00]            Logical Maximum (164)
[05, 07]                Usage Page (Keyboard)
[19, 00]                Usage Minimum (Reserved (0))
[2a, a4, 00]            Usage Maximum (Keyboard Ex Sel)
[81, 00]                Input (Data, Arr, Abs)
[c0]                End Collection
[05, 01]            Usage Page (Generic Desktop Controls)
[09, 02]            Usage (Mouse)
[a1, 01]            Collection (Application)
[85, 02]                Report ID (2)
[09, 01]                Usage (Pointer)
[a1, 00]                Collection (Physical)
[95, 10]                    Report Count (16)
[75, 01]                    Report Size (1)
[15, 00]                    Logical Minimum (0)
[25, 01]                    Logical Maximum (1)
[05, 09]                    Usage Page (Button)
[19, 01]                    Usage Minimum (1)
[29, 10]                    Usage Maximum (16)
[81, 02]                    Input (Data, Var, Abs)
[05, 01]                    Usage Page (Generic Desktop Controls)
[16, 01, f8]                Logical Minimum (-2047)
[26, ff, 07]                Logical Maximum (2047)
[75, 0c]                    Report Size (12)
[95, 02]                    Report Count (2)
[09, 30]                    Usage (X)
[09, 31]                    Usage (Y)
[81, 06]                    Input (Data, Var, Rel)
[15, 81]                    Logical Minimum (-127)
[25, 7f]                    Logical Maximum (127)
[75, 08]                    Report Size (8)
[95, 01]                    Report Count (1)
[09, 38]                    Usage (Wheel)
[81, 06]                    Input (Data, Var, Rel)
[95, 01]                    Report Count (1)
[05, 0c]                    Usage Page (Consumer)
[0a, 38, 02]                Usage (AC Pan)
[81, 06]                    Input (Data, Var, Rel)
[c0]                    End Collection
[c0]                End Collection
[05, 0c]            Usage Page (Consumer)
[09, 01]            Usage (Consumer Control)
[a1, 01]            Collection (Application)
[85, 03]                Report ID (3)
[75, 10]                Report Size (16)
[95, 02]                Report Count (2)
[15, 01]                Logical Minimum (1)
[26, 8c, 02]            Logical Maximum (652)
[19, 01]                Usage Minimum (Consumer Control)
[2a, 8c, 02]            Usage Maximum (AC Send)
[81, 60]                Input (Data, Arr, Abs)
[c0]                End Collection
[06, 43, ff]        Usage Page (Vendor Defined (0xff43))
[0a, 02, 02]        Usage (514)
[a1, 01]            Collection (Application)
[85, 11]                Report ID (17)
[75, 08]                Report Size (8)
[95, 13]                Report Count (19)
[15, 00]                Logical Minimum (0)
[26, ff, 00]            Logical Maximum (255)
[09, 02]                Usage (2)
[81, 00]                Input (Data, Arr, Abs)
[09, 02]                Usage (2)
[91, 00]                Output (Data, Arr, Abs)
[c0]                End Collection
"
    );

    let expected = expected_input_reports(&report_descriptor).unwrap();

    // Test 4 & 2 buttons pressed
    let report: Vec<u8> = vec![1, 0, 33, 31, 0, 0, 0, 0];
    let parsed_report = parse_raw_input_report(&report, &expected).unwrap();

    assert_eq!(
        format!("{}", parsed_report),
        "ReportId(1)
Keyboard - Keyboard Left Control(0)
Keyboard - Keyboard Left Shift(0)
Keyboard - Keyboard Left Alt(0)
Keyboard - Keyboard Left GUI(0)
Keyboard - Keyboard Right Control(0)
Keyboard - Keyboard Right Shift(0)
Keyboard - Keyboard Right Alt(0)
Keyboard - Keyboard Right GUI(0)
Keyboard - Keyboard 4 and $
Keyboard - Keyboard 2 and @
"
    );

    // Test Volume up pressed
    let report: Vec<u8> = vec![3, 233, 0, 0, 0];
    let parsed_report = parse_raw_input_report(&report, &expected).unwrap();

    assert_eq!(
        format!("{}", parsed_report),
        "ReportId(3)
Consumer - Volume Increment
"
    );
}
