use crate::report_descriptor::DataFieldOptions;
use crate::usage_table::{Usage, UsagePage};

/// From an HID Report Descriptor we can derive a list of expected reports. ExpectedReports
/// describes these reports.
#[derive(Debug, PartialEq)]
pub struct ExpectedReports {
    /// Indicate if all expected report have a Report ID
    pub(super) has_report_id: bool,

    /// All expected reports
    pub(super) reports: Vec<ExpectedReport>,
}

#[derive(Debug, PartialEq)]
pub struct ExpectedReport {
    pub(super) report_id: Option<u8>,
    pub(super) size: usize,
    pub(super) fields: Vec<ExpectedField>,
}

#[derive(Debug, PartialEq)]
pub enum ExpectedField {
    ReportId(u8),
    Constant(ExpectedFieldItem),
    Variable(ExpectedFieldItem),
    ArrayItem(ExpectedFieldItem),
}

#[derive(Debug, PartialEq)]
pub struct ExpectedFieldItem {
    pub(super) usage_page: UsagePage, // From Report Descriptor
    pub(super) usage: Usage,          // From Report Descriptor
    pub(super) index_in_raw: usize,
    pub(super) size_bits: usize, // Report Size
    pub(super) options: DataFieldOptions,
}

impl ExpectedReports {
    /// Find a expected report by report id
    pub fn find_report(&self, report_id: Option<u8>) -> Option<&ExpectedReport> {
        self.reports.iter().find(|&r| r.report_id == report_id)
    }
}

impl From<(bool, Vec<ExpectedReport>)> for ExpectedReports {
    fn from(value: (bool, Vec<ExpectedReport>)) -> Self {
        ExpectedReports {
            has_report_id: value.0,
            reports: value.1,
        }
    }
}

impl ExpectedField {
    /// Return the size of the item
    pub(super) fn size(&self) -> usize {
        match self {
            ExpectedField::ReportId(_) => 8,
            ExpectedField::Constant(item) => item.size_bits,
            ExpectedField::Variable(item) => item.size_bits,
            ExpectedField::ArrayItem(item) => item.size_bits,
        }
    }

    /// Get the index in the raw report
    pub(super) fn index(&self) -> usize {
        match self {
            ExpectedField::ReportId(_) => 0,
            ExpectedField::Constant(item) => item.index_in_raw,
            ExpectedField::Variable(item) => item.index_in_raw,
            ExpectedField::ArrayItem(item) => item.index_in_raw,
        }
    }
}

impl<'a> TryFrom<Vec<super::input::Input<'a>>> for ExpectedReport {
    type Error = super::InputError;

    fn try_from(value: Vec<super::input::Input<'a>>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(super::InputError::Empty);
        }

        let mut expected_fields: Vec<ExpectedField> = Vec::new();
        let mut size: usize = 0;
        let report_id = value.last().unwrap().report_id();

        // Add report_id
        if let Some(report_id) = report_id {
            size = 8;
            expected_fields.push(ExpectedField::ReportId(report_id));
        }

        // Add other fields
        for input in value {
            let mut expected = super::input::expected_fields(input, size)?;
            let total_size_of_expected: usize = expected.iter().map(|f| f.size()).sum();
            size += total_size_of_expected;

            expected_fields.append(&mut expected)
        }

        Ok(ExpectedReport {
            report_id,
            size,
            fields: expected_fields,
        })
    }
}
