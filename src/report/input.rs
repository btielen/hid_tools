use super::InputError;
use crate::report::expected::{ExpectedField, ExpectedFieldItem};
use crate::report_descriptor::{GlobalType, Mutability, ReportDescriptorItem, Structure};
use crate::usage_table::{Usage, UsagePage};

#[derive(Debug, PartialEq, Clone)]
pub struct GlobalItemTracker {
    usage_page: Option<UsagePage>,
    report_size: Option<u32>,
    report_id: Option<u8>,
    report_count: Option<u32>,
    logical_minimum: Option<i32>,
    logical_maximum: Option<i32>,
}

#[derive(Clone)]
pub struct LocalItemTracker<'a> {
    usage: Vec<&'a ReportDescriptorItem>,
    usage_minimum: Option<u16>,
    usage_maximum: Option<u16>,
}

impl GlobalItemTracker {
    /// Set the global Usage Page
    pub(crate) fn set_usage_page(
        &mut self,
        item: &ReportDescriptorItem,
    ) -> Result<&Self, InputError> {
        if !item.is_usage_page() {
            return Err(InputError::InvalidItemType);
        }

        self.usage_page = Some(item.usage_page().ok_or(InputError::InvalidPayload)?);
        Ok(self)
    }

    /// Get the Usage Page as reference
    fn usage_page(&self) -> Option<&UsagePage> {
        self.usage_page.as_ref()
    }

    /// Set the global Report Size
    pub(crate) fn set_report_size(
        &mut self,
        item: &ReportDescriptorItem,
    ) -> Result<&Self, InputError> {
        if !item.is_report_size() {
            return Err(InputError::InvalidItemType);
        }

        self.report_size = Some(item.payload_u32());
        Ok(self)
    }

    /// Get the Report Size
    fn report_size(&self) -> Option<u32> {
        self.report_size
    }

    /// Set the global Report ID
    pub(crate) fn set_report_id(
        &mut self,
        item: &ReportDescriptorItem,
    ) -> Result<&Self, InputError> {
        if !item.is_report_id() {
            return Err(InputError::InvalidItemType);
        }

        let report_id = u8::try_from(item.raw_payload()).map_err(|_| InputError::InvalidPayload)?;
        self.report_id = Some(report_id);
        Ok(self)
    }

    /// Get the Report ID as a SizedPayload
    fn report_id(&self) -> Option<u8> {
        self.report_id
    }

    /// Set the global Report Count
    pub(crate) fn set_report_count(
        &mut self,
        item: &ReportDescriptorItem,
    ) -> Result<&Self, InputError> {
        if !item.is_report_count() {
            return Err(InputError::InvalidItemType);
        }

        self.report_count = Some(item.payload_u32());
        Ok(self)
    }

    /// Get the Report Count
    fn report_count(&self) -> Option<u32> {
        self.report_count
    }

    /// Set the Logical Minimum
    pub(crate) fn set_logical_minimum(
        &mut self,
        item: &ReportDescriptorItem,
    ) -> Result<&Self, InputError> {
        if !item.is_logical_minimum() {
            return Err(InputError::InvalidItemType);
        }

        self.logical_minimum = Some(item.payload_i32());
        Ok(self)
    }

    /// Set the Logical Maximum
    pub(crate) fn set_logical_maximum(
        &mut self,
        item: &ReportDescriptorItem,
    ) -> Result<&Self, InputError> {
        if !item.is_logical_maximum() {
            return Err(InputError::InvalidItemType);
        }

        self.logical_maximum = Some(item.payload_i32());
        Ok(self)
    }
}

impl<'a> LocalItemTracker<'a> {
    pub(crate) fn add_usage(
        &mut self,
        item: &'a ReportDescriptorItem,
    ) -> Result<&Self, InputError> {
        if !item.is_usage() {
            return Err(InputError::InvalidItemType);
        }

        self.usage.push(item);
        Ok(self)
    }

    pub(crate) fn set_usage_minimum(
        &mut self,
        item: &'a ReportDescriptorItem,
    ) -> Result<&Self, InputError> {
        if !item.is_usage_minimum() {
            return Err(InputError::InvalidItemType);
        }

        self.usage_minimum = Some(item.payload_u16().ok_or(InputError::InvalidPayload)?);
        Ok(self)
    }

    pub(crate) fn set_usage_maximum(
        &mut self,
        item: &'a ReportDescriptorItem,
    ) -> Result<&Self, InputError> {
        if !item.is_usage_maximum() {
            return Err(InputError::InvalidItemType);
        }

        self.usage_maximum = Some(item.payload_u16().ok_or(InputError::InvalidPayload)?);
        Ok(self)
    }
}

#[derive(Clone)]
pub struct Input<'a> {
    input_item: &'a ReportDescriptorItem,
    global_items: GlobalItemTracker,
    local_items: LocalItemTracker<'a>,
}

impl<'a> Input<'a> {
    pub fn report_id(&self) -> Option<u8> {
        self.global_items.report_id()
    }
}

impl<'a>
    TryFrom<(
        &'a ReportDescriptorItem,
        GlobalItemTracker,
        LocalItemTracker<'a>,
    )> for Input<'a>
{
    type Error = InputError;

    fn try_from(
        value: (
            &'a ReportDescriptorItem,
            GlobalItemTracker,
            LocalItemTracker<'a>,
        ),
    ) -> Result<Self, Self::Error> {
        if !value.0.is_input() {
            return Err(InputError::InvalidItemType);
        }

        Ok(Input {
            input_item: value.0,
            global_items: value.1,
            local_items: value.2,
        })
    }
}

impl Default for GlobalItemTracker {
    fn default() -> Self {
        GlobalItemTracker {
            usage_page: None,
            report_size: None,
            report_id: None,
            report_count: None,
            logical_minimum: None,
            logical_maximum: None,
        }
    }
}

impl<'a> Default for LocalItemTracker<'a> {
    fn default() -> Self {
        LocalItemTracker {
            usage: Vec::new(),
            usage_minimum: None,
            usage_maximum: None,
        }
    }
}

pub fn expected_fields(
    input: Input,
    index_in_raw: usize,
) -> Result<Vec<ExpectedField>, InputError> {
    let default_usage_page = UsagePage::default();
    let usage_page = input
        .global_items
        .usage_page()
        .unwrap_or(&default_usage_page);

    let options = input
        .input_item
        .data_field_options()
        .ok_or(InputError::InvalidPayload)?;

    let report_size = input
        .global_items
        .report_size()
        .ok_or(InputError::GlobalItemNotSet(GlobalType::ReportSize))?;

    let report_count = input
        .global_items
        .report_count()
        .ok_or(InputError::GlobalItemNotSet(GlobalType::ReportCount))?;

    let mut expected_fields: Vec<ExpectedFieldItem> = Vec::new();
    // Create a bunch of ExpectedFieldItems
    for i in 0..report_count as usize {
        let usage_page = usage_page.clone();
        let options = options.clone();

        let usage = match (
            options.mutability(),
            options.structure(),
            input.local_items.usage_minimum,
            input.local_items.usage_maximum,
            input.local_items.usage.get(i),
        ) {
            (Mutability::Data, Structure::Variable, Some(min), Some(_max), _) => {
                Usage::from((&usage_page, min + i as u16))
            }
            (Mutability::Data, Structure::Variable, _, _, Some(&item)) => {
                item.usage(&usage_page).unwrap_or_default()
            }
            (Mutability::Constant, _, _, _, Some(&item)) => {
                item.usage(&usage_page).unwrap_or_default()
            }
            _ => Usage::default(),
        };

        let size_bits = report_size as usize;
        let index_in_raw = index_in_raw + size_bits * i;

        let item = ExpectedFieldItem {
            usage_page,
            usage,
            index_in_raw,
            size_bits,
            options,
        };

        expected_fields.push(item);
    }

    let expected_fields: Vec<ExpectedField> = match (options.mutability(), options.structure()) {
        (Mutability::Data, Structure::Array) => expected_fields
            .into_iter()
            .map(ExpectedField::ArrayItem)
            .collect(),
        (Mutability::Data, Structure::Variable) => expected_fields
            .into_iter()
            .map(ExpectedField::Variable)
            .collect(),
        (Mutability::Constant, _) => expected_fields
            .into_iter()
            .map(ExpectedField::Constant)
            .collect(),
    };

    Ok(expected_fields)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hid::*;
    use crate::report_descriptor::data::Size;
    use crate::report_descriptor::{ItemType, MainType};

    #[test]
    fn input_struct_from_input_descriptor_item_is_ok() {
        let input_item = ReportDescriptorItem {
            kind: ItemType::Main(MainType::Input),
            payload_size: Size::Empty,
            raw: vec![],
        };

        let result = Input::try_from((
            &input_item,
            GlobalItemTracker::default(),
            LocalItemTracker::default(),
        ));
        assert!(result.is_ok())
    }

    #[test]
    fn input_struct_from_non_input_descriptor_item_is_err() {
        let input_item = ReportDescriptorItem {
            kind: ItemType::Main(MainType::Output),
            payload_size: Size::Empty,
            raw: vec![],
        };

        let result = Input::try_from((
            &input_item,
            GlobalItemTracker::default(),
            LocalItemTracker::default(),
        ));
        assert!(result.is_err())
    }
}
