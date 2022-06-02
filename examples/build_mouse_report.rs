use hid_tools::report_builder::ReportDescriptorBuilder;
use hid_tools::report_descriptor::Collection;
use hid_tools::usage_table::generic_desktop::GenericDesktopControlsUsage;
use hid_tools::usage_table::UsagePage;

fn main() {
    let bytes = ReportDescriptorBuilder::new()
        .item(UsagePage::GenericDesktopControls)
        .item(GenericDesktopControlsUsage::Mouse)
        .item(Collection::Application)
        .item(GenericDesktopControlsUsage::Pointer)
        .item(Collection::Physical)
        .item(UsagePage::Button)
        .usage_minimum::<u16>(1)
        .usage_maximum::<u16>(3)
        .logical_minimum(0)
        .logical_maximum(3)
        .report_count(3)
        .report_size(1)
        .input(0x02) // Input (Data, Variable, Absolute)
        .report_count(1)
        .report_size(5)
        .input(0x01) // Input (Constant)
        .item(UsagePage::GenericDesktopControls)
        .item(GenericDesktopControlsUsage::X)
        .item(GenericDesktopControlsUsage::Y)
        .logical_minimum(-127)
        .logical_maximum(127)
        .report_size(8)
        .report_count(2)
        .input(0x02)
        .end_collection()
        .end_collection()
        .build() // Input (Data, Variable, Absolute)
        .bytes();

    println!("{:02x?}", bytes)
}
