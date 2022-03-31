use crate::payload::{Payload, Size};
use crate::usage_table::Usage;
use crate::usage_table::UsagePage;

/// HID Descriptor Report item type
#[derive(Debug, PartialEq)]
pub enum ItemType {
    Main(MainType),
    Global(GlobalType),
    Local(LocalType),
}

#[derive(Debug, PartialEq)]
pub enum MainType {
    Input,
    Output,
    Feature,
    Collection,
    EndCollection,
}

// https://www.usb.org/sites/default/files/hid1_11.pdf - page 28
#[derive(Debug, PartialEq)]
pub enum Collection {
    Physical,
    Application,
    Logical,
    Report,
    NamedArray,
    UsageSwitch,
    UsageModifier,
    Reserved(u8),
    VendorDefined(u8),
}

#[derive(Debug, PartialEq)]
pub enum LocalType {
    Usage,
    UsageMinimum,
    UsageMaximum,
    DesignatorIndex,
    DesignatorMinimum,
    DesignatorMaximum,
    StringIndex,
    StringMinimum,
    StringMaximum,
    Delimiter,
}

/// Global items describe rather than define data from a control. A new Main item
/// assumes the characteristics of the item state table. Global items can change the
/// state table. As a result Global item tags apply to all subsequently defined items
/// unless overridden by another Global item.
#[derive(Debug, PartialEq)]
pub enum GlobalType {
    UsagePage,
    LogicalMinimum,
    LogicalMaximum,
    PhysicalMinimum,
    PhysicalMaximum,
    UnitExponent,
    Unit,
    ReportSize,
    ReportID,
    ReportCount,
    Push,
    Pop,
}

impl From<u8> for Collection {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Collection::Physical,
            0x01 => Collection::Application,
            0x02 => Collection::Logical,
            0x03 => Collection::Report,
            0x04 => Collection::NamedArray,
            0x05 => Collection::UsageSwitch,
            0x06 => Collection::UsageModifier,
            i if (0x07..=0x7f).contains(&i) => Collection::Reserved(i),
            i => Collection::VendorDefined(i),
        }
    }
}

/// Represents one item in the Report Descriptor. This is a variable-sized
/// element with one header byte and 0, 1, 2, 4 payload bytes.
#[derive(Debug, PartialEq)]
pub struct ReportDescriptorItem {
    pub(crate) kind: ItemType,
    pub(crate) payload_size: Size,
    pub(crate) raw: Vec<u8>,
}

impl ReportDescriptorItem {
    /// Get the raw payload of the Report Descriptor Item
    pub fn raw_payload(&self) -> Payload {
        Payload::new(&self.raw[1..])
    }

    /// Determine if current item describes the Usage Page
    pub fn is_usage_page(&self) -> bool {
        self.kind == ItemType::Global(GlobalType::UsagePage)
    }

    /// Determine if current item describes the Usage
    pub fn is_usage(&self) -> bool {
        self.kind == ItemType::Local(LocalType::Usage)
    }

    /// Determine if current item describes the Usage Minimum
    pub fn is_usage_minimum(&self) -> bool {
        self.kind == ItemType::Local(LocalType::UsageMinimum)
    }

    /// Determine if current item describes the Usage Maximum
    pub fn is_usage_maximum(&self) -> bool {
        self.kind == ItemType::Local(LocalType::UsageMaximum)
    }

    /// Determine if current item describes the Logical Minimum
    pub fn is_logical_minimum(&self) -> bool {
        self.kind == ItemType::Global(GlobalType::LogicalMinimum)
    }

    /// Determine if current item describes the Logical Maximum
    pub fn is_logical_maximum(&self) -> bool {
        self.kind == ItemType::Global(GlobalType::LogicalMaximum)
    }

    /// Determine if current item describes the Report Size
    pub fn is_report_size(&self) -> bool {
        self.kind == ItemType::Global(GlobalType::ReportSize)
    }

    /// Determine if current item describes the Report Count
    pub fn is_report_count(&self) -> bool {
        self.kind == ItemType::Global(GlobalType::ReportCount)
    }

    /// Determine if current item describes the start of a Collection
    pub fn is_collection(&self) -> bool {
        self.kind == ItemType::Main(MainType::Collection)
    }

    /// Determine if current item describes the start of a Collection
    pub fn is_end_collection(&self) -> bool {
        self.kind == ItemType::Main(MainType::EndCollection)
    }

    /// Get the Usage Page. Will return None if this item
    /// is a item that doesn't describe the Usage Page, or if the
    /// payload data couldn't be converted to an UsagePage
    pub fn usage_page(&self) -> Option<UsagePage> {
        if !self.is_usage_page() {
            return None;
        }

        UsagePage::try_from(self.raw_payload()).ok()
    }

    /// Get the Usage given a UsagePage. This function will return None
    /// if the current item does not describe a Usage or if the payload
    /// couldn't be converted.
    pub fn usage(&self, usage_page: &UsagePage) -> Option<Usage> {
        if !self.is_usage() {
            return None;
        }

        Usage::try_from((usage_page, self.raw_payload())).ok()
    }

    /// Get the Collection type. If this item doesn't describe the Collection
    /// or the payload is not exactly one byte the function will return None.
    pub fn collection(&self) -> Option<Collection> {
        if !self.is_collection() || self.payload_size != Size::One {
            return None;
        }

        Some(Collection::from(self.raw_payload().data()[0]))
    }
}

/// A list of Report Descriptor Items
#[derive(Debug, PartialEq)]
pub struct ReportDescriptorItemList {
    items: Vec<ReportDescriptorItem>,
}

impl ReportDescriptorItemList {
    /// Create a new ReportDescriptorList
    pub fn new(items: Vec<ReportDescriptorItem>) -> Self {
        ReportDescriptorItemList { items }
    }

    /// Get a list of all items
    pub fn items(&self) -> &Vec<ReportDescriptorItem> {
        &self.items
    }

    /// Find the first item that describes the Usage Page
    pub fn find_first_usage_page_item(&self) -> Option<&ReportDescriptorItem> {
        self.items.iter().find(|&item| item.is_usage_page())
    }

    /// Find the first item that describes the Usage
    pub fn find_first_usage_item(&self) -> Option<&ReportDescriptorItem> {
        self.items.iter().find(|&item| item.is_usage())
    }

    /// Find the first item that describes the Logical Minimum
    pub fn find_first_logical_minimum_item(&self) -> Option<&ReportDescriptorItem> {
        self.items.iter().find(|&item| item.is_logical_minimum())
    }

    /// Find the first item that describes the Logical Maximum
    pub fn find_first_logical_maximum_item(&self) -> Option<&ReportDescriptorItem> {
        self.items.iter().find(|&item| item.is_logical_maximum())
    }

    /// Find the first item that describes the Report Size
    pub fn find_first_report_size_item(&self) -> Option<&ReportDescriptorItem> {
        self.items.iter().find(|&item| item.is_report_size())
    }

    /// Find the first item that describes the Report Count
    pub fn find_first_report_count_item(&self) -> Option<&ReportDescriptorItem> {
        self.items.iter().find(|&item| item.is_report_count())
    }

    /// Find the first item of a type
    pub fn find_first(&self, key: ItemType) -> Option<&ReportDescriptorItem> {
        self.items.iter().find(|&item| item.kind == key)
    }

    /// Filter the item
    pub fn filter_by_type(&self, key: ItemType) -> Vec<&ReportDescriptorItem> {
        self.items.iter().filter(|&item| item.kind == key).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collection_from_u8_reserved() {
        assert_eq!(Collection::from(0x08), Collection::Reserved(8));
    }

    #[test]
    fn collection_from_u8_vendor_defined() {
        assert_eq!(Collection::from(0xF0), Collection::VendorDefined(0xF0))
    }
}
