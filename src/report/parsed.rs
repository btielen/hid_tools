use crate::report::expected::ExpectedField;
use crate::report::InputError;
use crate::report_descriptor::DataFieldOptions;
use crate::usage_table::{Usage, UsagePage};

/// A parsed data report
#[derive(Debug, PartialEq)]
pub struct ParsedReport {
    pub(super) report_id: Option<u8>,
    pub(crate) fields: Vec<Field>,
}

/// A parsed data field
#[derive(Debug, PartialEq)]
pub enum Field {
    /// The parsed report id
    ReportId(u8),

    /// A constant read-only value
    Constant(i64),

    /// A variable value
    Variable(VarItem),

    /// An array value (non-zero)
    ArrayValue(ArrayValueItem),

    /// An array value that is zero
    ArrayZeroValue(ArrayZeroItem),
}

/// A parsed variable value in a report
#[derive(Debug, PartialEq)]
pub struct VarItem {
    pub(crate) usage_page: UsagePage, // From Report Descriptor
    pub(crate) usage: Usage,          // From Report Descriptor
    pub(crate) value: i64,            // From RawReport
    pub(super) options: DataFieldOptions,
}

/// A parsed array value in a report
#[derive(Debug, PartialEq)]
pub struct ArrayValueItem {
    pub(crate) usage_page: UsagePage,
    pub(crate) usage: Usage,
    options: DataFieldOptions,
}

/// A parsed array value in a report that has a value of zero
#[derive(Debug, PartialEq)]
pub struct ArrayZeroItem {
    pub(crate) usage_page: UsagePage,
    options: DataFieldOptions,
}

impl TryFrom<(&ExpectedField, i64)> for Field {
    type Error = InputError;

    fn try_from(value: (&ExpectedField, i64)) -> Result<Self, Self::Error> {
        let field = match (value.0, value.1) {
            (ExpectedField::ReportId(id), _) => Field::ReportId(*id),
            (ExpectedField::Constant(_), c) => Field::Constant(c),
            (ExpectedField::Variable(_), val) => {
                Field::Variable(VarItem::try_from((value.0, val))?)
            }
            (ExpectedField::ArrayItem(_), 0) => {
                Field::ArrayZeroValue(ArrayZeroItem::try_from(value.0)?)
            }
            (ExpectedField::ArrayItem(_), _) => {
                Field::ArrayValue(ArrayValueItem::try_from((value.0, value.1))?)
            }
        };

        Ok(field)
    }
}

impl TryFrom<(&ExpectedField, i64)> for VarItem {
    type Error = InputError;

    fn try_from(value: (&ExpectedField, i64)) -> Result<Self, Self::Error> {
        match value.0 {
            ExpectedField::Variable(item) => Ok(VarItem {
                usage_page: item.usage_page.clone(),
                usage: item.usage.clone(),
                value: value.1,
                options: item.options.clone(),
            }),
            _ => Err(InputError::VariableItemExpected),
        }
    }
}

impl TryFrom<(&ExpectedField, i64)> for ArrayValueItem {
    type Error = InputError;

    fn try_from(value: (&ExpectedField, i64)) -> Result<Self, Self::Error> {
        match value.0 {
            ExpectedField::ArrayItem(item) => {
                let usage_id = u16::try_from(value.1).map_err(|_| InputError::InvalidPayload)?;

                Ok(ArrayValueItem {
                    usage_page: item.usage_page.clone(),
                    usage: Usage::from((&item.usage_page, usage_id)),
                    options: item.options.clone(),
                })
            }
            _ => Err(InputError::ArrayItemExpected),
        }
    }
}

impl TryFrom<&ExpectedField> for ArrayZeroItem {
    type Error = InputError;

    fn try_from(value: &ExpectedField) -> Result<Self, Self::Error> {
        match value {
            ExpectedField::ArrayItem(item) => Ok(ArrayZeroItem {
                usage_page: item.usage_page.clone(),
                options: item.options.clone(),
            }),
            _ => Err(InputError::ArrayItemExpected),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report::expected::{ExpectedField, ExpectedFieldItem};
    use crate::usage_table::generic_desktop::GenericDesktopControlsUsage;
    use crate::usage_table::keyboard::KeyboardUsage;
    use crate::usage_table::{Usage, UsagePage};

    #[test]
    fn convert_tuple_to_arr_item() {
        let expected_field = ExpectedField::ArrayItem(ExpectedFieldItem {
            usage_page: UsagePage::Keyboard,
            usage: Usage::Undefined,
            index_in_raw: 0,
            size_bits: 1,
            options: Default::default(),
        });

        let result = ArrayValueItem::try_from((&expected_field, 0x12));

        assert_eq!(
            result,
            Ok(ArrayValueItem {
                usage_page: UsagePage::Keyboard,
                usage: Usage::Keyboard(KeyboardUsage::KeyboardoandO), //KeyboardUsage::from(0x12)
                options: Default::default()
            })
        );
    }

    #[test]
    fn convert_tuple_to_var_item() {
        let expected_field = ExpectedField::Variable(ExpectedFieldItem {
            usage_page: UsagePage::GenericDesktopControls,
            usage: Usage::GenericDesktopControls(GenericDesktopControlsUsage::X),
            index_in_raw: 0,
            size_bits: 1,
            options: Default::default(),
        });

        let result = VarItem::try_from((&expected_field, 0x12));

        assert_eq!(
            result,
            Ok(VarItem {
                usage_page: UsagePage::GenericDesktopControls,
                usage: Usage::GenericDesktopControls(GenericDesktopControlsUsage::X),
                value: 0x12,
                options: Default::default()
            })
        );
    }
}
