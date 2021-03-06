mod expected;
mod input;
mod parse;
/// Parsed Report data
pub mod parsed;

use crate::report::expected::{ExpectedReport, ExpectedReports};
use crate::report::input::{GlobalItemTracker, Input, LocalItemTracker};
use crate::report::parse::{report_id, val};
use crate::report::parsed::{Field, ParsedReport};
use crate::report_descriptor::{GlobalType, ItemType, LocalType, MainType, ReportDescriptor};
use thiserror::Error;

/// Errors for working with an Report
#[derive(Error, Debug, PartialEq)]
pub enum InputError {
    /// Input was empty
    #[error("Input items is empty")]
    Empty,

    /// Item type was not expected
    #[error("Item type is invalid")]
    InvalidItemType,

    /// Payload was not valid
    #[error("Payload is invalid")]
    InvalidPayload,

    /// A global item was expected but not set
    #[error("Global item `{0}` not set")]
    GlobalItemNotSet(GlobalType),

    /// A local item was expected but not set
    #[error("Local item `{0}` not set")]
    LocalItemNotSet(LocalType),

    /// Report ID was expected
    #[error("Report ID expected")]
    ReportIdExpected,

    /// Report ID was not found
    #[error("Report ID not found in received reports")]
    UnknownReportId,

    /// An array item was expected
    #[error("An array item was expected")]
    ArrayItemExpected,

    /// An variable item was expected
    #[error("An variable item was expected")]
    VariableItemExpected,

    /// Can not take bits from input
    #[error("Can not take bits from input")]
    CannotTakeBits,
}

/// Create a list of expected reports from the Report Descriptor
///
/// # Example
/// ```
/// use hid_tools::report::expected_input_reports;
/// use hid_tools::report_builder::ReportDescriptorBuilder;
/// use hid_tools::usage_table::UsagePage;
/// use hid_tools::usage_table::generic_desktop::GenericDesktopControlsUsage;
///
/// let report_descriptor = ReportDescriptorBuilder::new()
///     .usage_page(UsagePage::GenericDesktopControls)
///     .usage(GenericDesktopControlsUsage::X)
///     .report_size(8)
///     .report_count(1)
///     .input(0x06) // Data, Variable, Relative
///     .build();
///
/// let expected = expected_input_reports(&report_descriptor);
/// ```
pub fn expected_input_reports(
    report_descriptor: &ReportDescriptor,
) -> Result<ExpectedReports, InputError> {
    let mut global_items = GlobalItemTracker::default();
    let mut local_items = LocalItemTracker::default();
    let mut inputs: Vec<Input> = Vec::new();
    let mut has_report_id = false;
    let mut reports: Vec<ExpectedReport> = Vec::new();

    for item in report_descriptor.items() {
        match item.kind {
            ItemType::Main(MainType::Input) => {
                let input = Input::try_from((item, global_items.clone(), local_items.clone()))?;
                inputs.push(input);
                local_items = LocalItemTracker::default();
            }
            ItemType::Main(MainType::EndCollection) => {
                if !inputs.is_empty() {
                    reports.push(ExpectedReport::try_from(inputs.to_vec())?);
                    inputs = Vec::new();
                }
            }
            ItemType::Global(GlobalType::UsagePage) => {
                global_items.set_usage_page(item)?;
            }
            ItemType::Global(GlobalType::ReportSize) => {
                global_items.set_report_size(item)?;
            }
            ItemType::Global(GlobalType::ReportID) => {
                has_report_id = true;
                global_items.set_report_id(item)?;
            }
            ItemType::Global(GlobalType::ReportCount) => {
                global_items.set_report_count(item)?;
            }
            ItemType::Global(GlobalType::LogicalMinimum) => {
                global_items.set_logical_minimum(item)?;
            }
            ItemType::Global(GlobalType::LogicalMaximum) => {
                global_items.set_logical_maximum(item)?;
            }
            ItemType::Local(LocalType::Usage) => {
                local_items.add_usage(item)?;
            }
            ItemType::Local(LocalType::UsageMinimum) => {
                local_items.set_usage_minimum(item)?;
            }
            ItemType::Local(LocalType::UsageMaximum) => {
                local_items.set_usage_maximum(item)?;
            }
            _ => {}
        }
    }

    if !inputs.is_empty() {
        reports.push(ExpectedReport::try_from(inputs)?);
    }

    Ok(ExpectedReports::from((has_report_id, reports)))
}

/// Parse raw input report
///
/// # Example
/// ```
/// use hid_tools::report_descriptor::parse::report_descriptor;
/// use hid_tools::report::{expected_input_reports, parse_raw_input_report};
///
/// let keyboard_report_descriptor_bytes: Vec<u8> = vec![0x05, 0x01, 0x09, 0x06, 0xa1, 0x01, 0x05, 0x08, 0x19, 0x01,0x29, 0x03, 0x15, 0x00, 0x25,
///     0x01, 0x75, 0x01, 0x95, 0x03,0x91, 0x02, 0x95, 0x05, 0x91, 0x01, 0x05, 0x07, 0x19, 0xe0,0x29,
///     0xe7, 0x95, 0x08, 0x81, 0x02, 0x75, 0x08, 0x95, 0x01,0x81, 0x01, 0x19, 0x00, 0x29, 0x91,
///     0x26, 0xff, 0x00, 0x95, 0x06, 0x81, 0x00, 0xc0];
///
/// // Left shift, a and b pressed on keyboard
/// let event_report: Vec<u8> = vec![0x02, 0, 0x04, 0x05, 0, 0, 0, 0];
/// let report_descriptor = report_descriptor(&keyboard_report_descriptor_bytes).unwrap();
/// let expected_reports = expected_input_reports(&report_descriptor).unwrap();
/// let parsed_report = parse_raw_input_report(&event_report, &expected_reports).unwrap();
///
/// println!("Event: {:?} \n\n{}", event_report, parsed_report);
/// ```
pub fn parse_raw_input_report(
    report: &[u8],
    expected_reports: &ExpectedReports,
) -> Result<ParsedReport, InputError> {
    // get Report ID if expected
    let report_id = match expected_reports.has_report_id {
        true => {
            let report_id = report_id(report).map_err(|_| InputError::ReportIdExpected)?;
            Some(report_id)
        }
        false => None,
    };

    let expected_report = expected_reports
        .find_report(report_id)
        .ok_or(InputError::UnknownReportId)?;

    let mut parsed_fields: Vec<Field> = Vec::new();

    for expected_field in &expected_report.fields {
        let data = val(
            report,
            expected_field.index() as u32,
            expected_field.size() as u32,
        )
        .map_err(|_| InputError::CannotTakeBits)?;

        parsed_fields.push(Field::try_from((expected_field, data))?);
    }

    Ok(ParsedReport {
        report_id,
        fields: parsed_fields,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report::expected::{ExpectedField, ExpectedFieldItem};
    use crate::report::parsed::{Field, VarItem};
    use crate::report_builder::ReportDescriptorBuilder;
    use crate::report_descriptor::{Collection, DataFieldOptions, Mutability, Structure, Value};
    use crate::usage_table::generic_desktop::GenericDesktopControlsUsage;
    use crate::usage_table::keyboard::KeyboardUsage;
    use crate::usage_table::{Usage, UsagePage};

    #[test]
    fn simple_var_input() {
        // Stripped mouse report
        let report = ReportDescriptorBuilder::new()
            .usage_page(UsagePage::GenericDesktopControls)
            .usage(GenericDesktopControlsUsage::X)
            .report_size(8)
            .report_count(1)
            .input(0x06) // Data, Variable, Relative
            .build();

        let result = expected_input_reports(&report);

        assert_eq!(
            result,
            Ok(ExpectedReports {
                has_report_id: false,
                reports: vec![ExpectedReport {
                    report_id: None,
                    size: 8,
                    fields: vec![ExpectedField::Variable(ExpectedFieldItem {
                        usage_page: UsagePage::GenericDesktopControls,
                        usage: Usage::GenericDesktopControls(GenericDesktopControlsUsage::X),
                        index_in_raw: 0,
                        size_bits: 8,
                        options: DataFieldOptions::from((
                            Mutability::Data,
                            Structure::Variable,
                            Value::Relative
                        ))
                    })]
                }]
            })
        );
    }

    #[test]
    fn simple_constant_input() {
        let report = ReportDescriptorBuilder::new()
            .usage_page(UsagePage::GenericDesktopControls)
            .usage(GenericDesktopControlsUsage::Keyboard)
            .report_size(5)
            .report_count(1)
            .input(0x01) // Constant
            .build();

        let result = expected_input_reports(&report);

        assert_eq!(
            result,
            Ok(ExpectedReports {
                has_report_id: false,
                reports: vec![ExpectedReport {
                    report_id: None,
                    size: 5,
                    fields: vec![ExpectedField::Constant(ExpectedFieldItem {
                        usage_page: UsagePage::GenericDesktopControls,
                        usage: Usage::GenericDesktopControls(GenericDesktopControlsUsage::Keyboard),
                        index_in_raw: 0,
                        size_bits: 5,
                        options: DataFieldOptions::from((
                            Mutability::Constant,
                            Structure::Array,
                            Value::Absolute
                        ))
                    })]
                }]
            })
        );
    }

    #[test]
    fn simple_array_input() {
        // Part from a keyboard report descriptor
        let report = ReportDescriptorBuilder::new()
            .usage_page(UsagePage::Keyboard)
            .usage_minimum::<u16>(0x00)
            .usage_maximum::<u16>(0x91)
            .report_count(2)
            .report_size(8)
            .input(0x00) // Data, Array, Abs
            .build();

        let result = expected_input_reports(&report);
        assert_eq!(
            result,
            Ok(ExpectedReports {
                has_report_id: false,
                reports: vec![ExpectedReport {
                    report_id: None,
                    size: 16,
                    fields: vec![
                        ExpectedField::ArrayItem(ExpectedFieldItem {
                            usage_page: UsagePage::Keyboard,
                            usage: Usage::Undefined,
                            index_in_raw: 0,
                            size_bits: 8,
                            options: DataFieldOptions::from((
                                Mutability::Data,
                                Structure::Array,
                                Value::Absolute
                            ))
                        }),
                        ExpectedField::ArrayItem(ExpectedFieldItem {
                            usage_page: UsagePage::Keyboard,
                            usage: Usage::Undefined,
                            index_in_raw: 8,
                            size_bits: 8,
                            options: DataFieldOptions::from((
                                Mutability::Data,
                                Structure::Array,
                                Value::Absolute
                            ))
                        })
                    ]
                }]
            })
        );
    }

    #[test]
    fn keyboard_multiple_variable() {
        // taken from real world keyboard
        let report = ReportDescriptorBuilder::new()
            .usage_page(UsagePage::GenericDesktopControls)
            .usage(Usage::GenericDesktopControls(
                GenericDesktopControlsUsage::Keyboard,
            ))
            .collection(Collection::Application)
            .usage_page(UsagePage::Keyboard)
            .usage_minimum(KeyboardUsage::KeyboardLeftControl)
            .usage_maximum(KeyboardUsage::KeyboardRightGUI)
            .report_count(8)
            .report_size(1)
            .input(0x02) // Data, Var, Abs
            .end_collection()
            .build();

        let result = expected_input_reports(&report).unwrap();

        let expected_first_field = ExpectedField::Variable(ExpectedFieldItem {
            usage_page: UsagePage::Keyboard,
            usage: Usage::Keyboard(KeyboardUsage::KeyboardLeftControl),
            index_in_raw: 0,
            size_bits: 1,
            options: DataFieldOptions::from((
                Mutability::Data,
                Structure::Variable,
                Value::Absolute,
            )),
        });

        let expected_last_field = ExpectedField::Variable(ExpectedFieldItem {
            usage_page: UsagePage::Keyboard,
            usage: Usage::Keyboard(KeyboardUsage::KeyboardRightGUI),
            index_in_raw: 7,
            size_bits: 1,
            options: DataFieldOptions::from((
                Mutability::Data,
                Structure::Variable,
                Value::Absolute,
            )),
        });

        assert_eq!(result.reports[0].size, 8);
        assert_eq!(result.reports[0].fields[0], expected_first_field);
        assert_eq!(result.reports[0].fields[7], expected_last_field);
    }

    #[test]
    fn mouse_multiple_var_and_constant_in_one_byte() {
        // Taken from a real world mouse report descriptor
        let report = ReportDescriptorBuilder::new()
            .usage_page(UsagePage::GenericDesktopControls)
            .usage(Usage::GenericDesktopControls(
                GenericDesktopControlsUsage::Mouse,
            ))
            .collection(Collection::Application)
            .usage_page(UsagePage::Button)
            .usage_minimum::<u16>(1)
            .usage_maximum::<u16>(5)
            .report_count(5)
            .report_size(1)
            .input(0x02) // Data, Var, Abs
            .report_count(1)
            .report_size(3)
            .input(0x01) // Constant
            .end_collection()
            .build();

        let result = expected_input_reports(&report).unwrap();

        let expected_first_field = ExpectedField::Variable(ExpectedFieldItem {
            usage_page: UsagePage::Button,
            usage: Usage::Button(1),
            index_in_raw: 0,
            size_bits: 1,
            options: DataFieldOptions::from((
                Mutability::Data,
                Structure::Variable,
                Value::Absolute,
            )),
        });

        let expected_last_field = ExpectedField::Constant(ExpectedFieldItem {
            usage_page: UsagePage::Button,
            usage: Usage::Undefined,
            index_in_raw: 5,
            size_bits: 3,
            options: DataFieldOptions::from((
                Mutability::Constant,
                Structure::Array,
                Value::Absolute,
            )),
        });

        assert_eq!(result.reports[0].size, 8);
        assert_eq!(result.reports[0].fields[0], expected_first_field);
        assert_eq!(result.reports[0].fields[5], expected_last_field);
    }

    #[test]
    fn parse_simple_report() {
        let expected_reports = ExpectedReports::from((
            false,
            vec![ExpectedReport {
                report_id: None,
                size: 1,
                fields: vec![ExpectedField::Variable(ExpectedFieldItem {
                    usage_page: UsagePage::Button,
                    usage: Usage::Button(1),
                    index_in_raw: 0,
                    size_bits: 1,
                    options: Default::default(),
                })],
            }],
        ));

        let expected_result = ParsedReport {
            report_id: None,
            fields: vec![Field::Variable(VarItem {
                usage_page: UsagePage::Button,
                usage: Usage::Button(1),
                value: 1,
                options: Default::default(),
            })],
        };

        let result = parse_raw_input_report(&vec![1], &expected_reports);
        assert_eq!(result, Ok(expected_result));
    }
}
