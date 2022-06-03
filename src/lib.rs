//! Tools to work with the Human Interface Device (HID) protocol
//!
//! The USB HID class describes devices used with nearly every modern computer. Many
//! predefined functions exist in the USB HID class. These functions allow hardware
//! manufacturers to design a product to USB HID class specifications and expect it
//! to work with any software that also meets these specifications. This crate provides some
//! tools to parse, build and display Report Descriptors and data reports.
//!
//! ## Getting started
//!
//! Following example parses a raw HID Report Descriptor
//!
//! ```rust
//! use hid_tools::report_descriptor::parse;
//!
//! let bytes = [
//!  0x05, 0x01, 0x09, 0x06, 0xa1, 0x01, 0x85, 0x01, 0x05, 0x07,
//!  0x19, 0xe0, 0x29, 0xe7, 0x15, 0x00, 0x25, 0x01, 0x75, 0x01,
//!  0x95, 0x08, 0x81, 0x02, 0x19, 0x01, 0x29, 0x97, 0x15, 0x00,
//!  0x25, 0x01, 0x75, 0x01, 0x95, 0x98, 0x81, 0x02, 0xc0,
//! ];
//!
//! let parsed = parse::report_descriptor(&bytes).unwrap();
//! println!("{}", parsed);
//! ```
//!
//!
//! ### Example Output
//! Following is an example output for a 2-factor authentication usb-stick.
//!
//! ```text
//! [06, d0, f1]        Usage Page (FIDO Alliance)
//! [09, 01]            Usage (U2F Authenticator Device)
//! [a1, 01]            Collection (Application)
//! [09, 20]                Usage (Input Report Data)
//! [15, 00]                Logical Minimum (0)
//! [26, ff, 00]            Logical Maximum (255)
//! [75, 08]                Report Size (8)
//! [95, 40]                Report Count (64)
//! [81, 02]                Input (Data, Var, Abs)
//! [09, 21]                Usage (Output Report Data)
//! [15, 00]                Logical Minimum (0)
//! [26, ff, 00]            Logical Maximum (255)
//! [75, 08]                Report Size (8)
//! [95, 40]                Report Count (64)
//! [91, 02]                Output (Data, Var, Abs)
//! [c0]                End Collection (0)
//! ```
//!
//! ## Report Descriptor
//!
//! The HID Report Descriptor is a hard coded array of bytes that describe the device's data packets. This
//! package provides a builder to create a Report Descriptor.
//!
//! ```rust
//! use hid_tools::report_builder::ReportDescriptorBuilder;
//! use hid_tools::report_descriptor::Collection;
//! use hid_tools::usage_table::{UsagePage};
//! use hid_tools::usage_table::generic_desktop::GenericDesktopControlsUsage;
//!
//! let raw_report = ReportDescriptorBuilder::new()
//!     .usage_page(UsagePage::GenericDesktopControls)
//!     .usage(GenericDesktopControlsUsage::Mouse)
//!     .item(Collection::Application)
//!     // add more items here (see also examples)
//!     .end_collection()
//!     .build()
//!     .bytes();
//!
//! println!("{:02x?}", raw_report)
//! ```
//!
//! ## Report
//!
//! With the parsed or built Report Descriptor we know which data Reports to expect. With
//! this we can parse a Report.
//!
//! ```rust
//! use hid_tools::report_descriptor::parse::report_descriptor;
//! use hid_tools::report::{expected_input_reports, parse_raw_input_report};
//!
//! let keyboard_report_descriptor_bytes: Vec<u8> = vec![
//!     0x05, 0x01, 0x09, 0x06, 0xa1, 0x01, 0x05, 0x08, 0x19, 0x01,0x29, 0x03, 0x15, 0x00, 0x25,
//!     0x01, 0x75, 0x01, 0x95, 0x03,0x91, 0x02, 0x95, 0x05, 0x91, 0x01, 0x05, 0x07, 0x19, 0xe0,0x29,
//!     0xe7, 0x95, 0x08, 0x81, 0x02, 0x75, 0x08, 0x95, 0x01,0x81, 0x01, 0x19, 0x00, 0x29, 0x91,
//!     0x26, 0xff, 0x00, 0x95, 0x06, 0x81, 0x00, 0xc0];
//!
//! // Left shift, a and b pressed on keyboard
//! let event_report: Vec<u8> = vec![0x02, 0, 0x04, 0x05, 0, 0, 0, 0];
//! let report_descriptor = report_descriptor(&keyboard_report_descriptor_bytes).unwrap();
//! let expected_reports = expected_input_reports(&report_descriptor).unwrap();
//! let parsed_report = parse_raw_input_report(&event_report, &expected_reports).unwrap();
//!
//! println!("Event: {:?} \n\n{}", event_report, parsed_report);
//! ```
//!
//! will print
//!
//! ```text
//! Event: [2, 0, 4, 5, 0, 0, 0, 0]
//!
//! Keyboard - Keyboard Left Control(0)
//! Keyboard - Keyboard Left Shift(1)
//! Keyboard - Keyboard Left Alt(0)
//! Keyboard - Keyboard Left GUI(0)
//! Keyboard - Keyboard Right Control(0)
//! Keyboard - Keyboard Right Shift(0)
//! Keyboard - Keyboard Right Alt(0)
//! Keyboard - Keyboard Right GUI(0)
//! Constant(0)
//! Keyboard - Keyboard a and A
//! Keyboard - Keyboard b and B
//! ```
//!
//! See also the `parse_raw_report_keyboard` example.
//!
//! ## Needs some work
//!
//! - Logical minimum/maximum
//! - Convert more Usage tables (help wanted)
//!
//!
//! ## Resources
//!
//! - [Device Class Definition for HID 1.11](https://www.usb.org/sites/default/files/hid1_11.pdf)
//! - [HID Usage tables](https://usb.org/sites/default/files/hut1_3_0.pdf)

#![warn(missing_docs)]

/// User-friendly display of reports, report descriptors and data
mod display;

/// Errors
mod error;

/// Create a list of expected reports from a Report Descriptor and parse event reports
pub mod report;

/// A builder for ReportDescriptor
pub mod report_builder;

/// Parse raw bytes of a Report Descriptor
pub mod report_descriptor;

/// Usages and tables for user-friendly displaying
pub mod usage_table;
