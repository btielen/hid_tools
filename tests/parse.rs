use hid_tools::hid::Collection;
use hid_tools::parse;
use hid_tools::report_builder::ReportDescriptorBuilder;
use hid_tools::usage_table::generic_desktop::GenericDesktopControlsUsage;
use hid_tools::usage_table::UsagePage;

#[test]
fn parse_equals_build() {
    // Keyboard descriptor report from
    // https://www.usb.org/sites/default/files/hid1_11.pdf - page 69
    let report = ReportDescriptorBuilder::new()
        .item(UsagePage::GenericDesktopControls)
        .item(GenericDesktopControlsUsage::Keyboard)
        .item(Collection::Application)
        .item(UsagePage::Keyboard)
        .usage_minimum::<u16>(224)
        .usage_maximum::<u16>(231)
        .logical_minimum(0)
        .logical_maximum(1)
        .report_size(1)
        .report_count(8)
        .input(0x02)
        .report_count(1)
        .report_size(8)
        .input(0x01)
        .report_count(5)
        .report_size(1)
        .item(UsagePage::LED)
        .usage_minimum::<u16>(1)
        .usage_maximum::<u16>(5)
        .output(0x02)
        .report_count(1)
        .report_size(3)
        .output(0x01)
        .report_count(6)
        .report_size(8)
        .logical_minimum(0)
        .logical_maximum(101)
        .usage_page(UsagePage::Keyboard)
        .usage_minimum::<u16>(0)
        .usage_maximum::<u16>(101)
        .input(0)
        .end_collection()
        .build();

    let raw: Vec<u8> = vec![
        0x05, 0x01, 0x09, 0x06, 0xA1, 0x01, 0x05, 0x07, 0x19, 0xE0, 0x29, 0xE7, 0x15, 0x00, 0x25,
        0x01, 0x75, 0x01, 0x95, 0x08, 0x81, 0x02, 0x95, 0x01, 0x75, 0x08, 0x81, 0x01, 0x95, 0x05,
        0x75, 0x01, 0x05, 0x08, 0x19, 0x01, 0x29, 0x05, 0x91, 0x02, 0x95, 0x01, 0x75, 0x03, 0x91,
        0x01, 0x95, 0x06, 0x75, 0x08, 0x15, 0x00, 0x25, 0x65, 0x05, 0x07, 0x19, 0x00, 0x29, 0x65,
        0x81, 0x00, 0xC0,
    ];

    let parsed = parse::report_descriptor(&raw);
    assert_eq!(parsed, Ok(report))
}
