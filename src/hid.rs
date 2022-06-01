use crate::data::PrefixByte;
use crate::data::{Size, SizedPayload};
use crate::parse::data_field_options_from_payload;
use crate::usage_table::Usage;
use crate::usage_table::UsagePage;

/// HID Descriptor Report item type
#[derive(Debug, PartialEq, Clone)]
pub enum ItemType {
    Main(MainType),
    Global(GlobalType),
    Local(LocalType),
}

#[derive(Debug, PartialEq, Clone)]
pub enum MainType {
    Input,
    Output,
    Feature,
    Collection,
    EndCollection,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

/// Input, Output, and Feature items are used to create data fields within a report. Here
/// we define a struct to hold the available options.
/// https://www.usb.org/sites/default/files/hid1_11.pdf - page 28
#[derive(Debug, PartialEq, Clone)]
pub struct DataFieldOptions(
    Mutability,
    Structure,
    Value,
    Wrap,
    Linear,
    State,
    NullState,
    Volatile,
    Data,
);

impl DataFieldOptions {
    pub fn is_data(&self) -> bool {
        self.0 == Mutability::Data
    }

    pub fn is_constant(&self) -> bool {
        self.0 == Mutability::Constant
    }

    pub fn is_array(&self) -> bool {
        self.1 == Structure::Array
    }

    pub fn is_var(&self) -> bool {
        self.1 == Structure::Variable
    }

    pub fn is_absolute(&self) -> bool {
        self.2 == Value::Absolute
    }

    pub fn is_relative(&self) -> bool {
        self.2 == Value::Relative
    }

    pub fn mutability(&self) -> &Mutability {
        &self.0
    }

    pub fn structure(&self) -> &Structure {
        &self.1
    }

    pub fn value(&self) -> &Value {
        &self.2
    }

    pub fn wrap(&self) -> &Wrap {
        &self.3
    }

    pub fn linear(&self) -> &Linear {
        &self.4
    }

    pub fn state(&self) -> &State {
        &self.5
    }

    pub fn null_state(&self) -> &NullState {
        &self.6
    }

    pub fn volatile(&self) -> &Volatile {
        &self.7
    }

    pub fn data(&self) -> &Data {
        &self.8
    }
}

impl Default for DataFieldOptions {
    fn default() -> Self {
        DataFieldOptions(
            Mutability::default(),
            Structure::default(),
            Value::default(),
            Wrap::default(),
            Linear::default(),
            State::default(),
            NullState::default(),
            Volatile::default(),
            Data::default(),
        )
    }
}

impl
    From<(
        Mutability,
        Structure,
        Value,
        Wrap,
        Linear,
        State,
        NullState,
        Volatile,
        Data,
    )> for DataFieldOptions
{
    fn from(
        value: (
            Mutability,
            Structure,
            Value,
            Wrap,
            Linear,
            State,
            NullState,
            Volatile,
            Data,
        ),
    ) -> Self {
        DataFieldOptions(
            value.0, value.1, value.2, value.3, value.4, value.5, value.6, value.7, value.8,
        )
    }
}

impl From<(Mutability, Structure, Value)> for DataFieldOptions {
    fn from(value: (Mutability, Structure, Value)) -> Self {
        DataFieldOptions::from((
            value.0,
            value.1,
            value.2,
            Wrap::default(),
            Linear::default(),
            State::default(),
            NullState::default(),
            Volatile::default(),
            Data::default(),
        ))
    }
}

/// Indicates whether the item is data or a constant value
/// https://www.usb.org/sites/default/files/hid1_11.pdf - page 30
#[derive(Debug, PartialEq, Clone)]
pub enum Mutability {
    Data,
    Constant,
}

impl Default for Mutability {
    fn default() -> Self {
        Mutability::Data
    }
}

/// Indicates whether the item creates variable or array
/// data fields in reports.
/// https://www.usb.org/sites/default/files/hid1_11.pdf - page 30
#[derive(Debug, PartialEq, Clone)]
pub enum Structure {
    Array,
    Variable,
}

impl Default for Structure {
    fn default() -> Self {
        Structure::Array
    }
}

/// Indicates whether the data is absolute (based on a fixed origin) or
/// relative (indicating the change in value from the last report).
/// https://www.usb.org/sites/default/files/hid1_11.pdf - page 30
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Absolute,
    Relative,
}

impl Default for Value {
    fn default() -> Self {
        Value::Absolute
    }
}

/// Indicates whether the data “rolls over” when
/// reaching either the extreme high or low value.
/// https://www.usb.org/sites/default/files/hid1_11.pdf - page 31
#[derive(Debug, PartialEq, Clone)]
pub enum Wrap {
    NoWrap,
    Wrap,
}

impl Default for Wrap {
    fn default() -> Self {
        Wrap::NoWrap
    }
}

/// Indicates whether the raw data from the device has been processed in some way, and no longer
/// represents a linear relationship between what is measured and the data that is reported.
/// https://www.usb.org/sites/default/files/hid1_11.pdf - page 31
#[derive(Debug, PartialEq, Clone)]
pub enum Linear {
    Linear,
    NonLinear,
}

impl Default for Linear {
    fn default() -> Self {
        Linear::Linear
    }
}

/// Indicates whether the control has a preferred state to which it will return
/// when the user is not physically interacting with the control.
/// https://www.usb.org/sites/default/files/hid1_11.pdf - page 31
#[derive(Debug, PartialEq, Clone)]
pub enum State {
    Preferred,
    NoPreferred,
}

impl Default for State {
    fn default() -> Self {
        State::Preferred
    }
}

/// Indicates whether the control has a state in which it
/// is not sending meaningful data.
/// https://www.usb.org/sites/default/files/hid1_11.pdf - page 31
#[derive(Debug, PartialEq, Clone)]
pub enum NullState {
    NoNullPosition,
    NullState,
}

impl Default for NullState {
    fn default() -> Self {
        NullState::NoNullPosition
    }
}

/// Indicates whether the Feature or Output control's
/// value should be changed by the host or not.
/// https://www.usb.org/sites/default/files/hid1_11.pdf - page 31
#[derive(Debug, PartialEq, Clone)]
pub enum Volatile {
    NonVolatile,
    Volatile,
}

impl Default for Volatile {
    fn default() -> Self {
        Volatile::NonVolatile
    }
}

/// Indicates that the control emits a fixed-size stream of bytes.
/// https://www.usb.org/sites/default/files/hid1_11.pdf - page 31
#[derive(Debug, PartialEq, Clone)]
pub enum Data {
    BitField,
    BufferedBytes,
}

impl Default for Data {
    fn default() -> Self {
        Data::BitField
    }
}

/// https://www.usb.org/sites/default/files/hid1_11.pdf - page 28
#[derive(Debug, PartialEq, Clone)]
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

impl From<Collection> for u8 {
    fn from(value: Collection) -> Self {
        match value {
            Collection::Physical => 0x00,
            Collection::Application => 0x01,
            Collection::Logical => 0x02,
            Collection::Report => 0x03,
            Collection::NamedArray => 0x04,
            Collection::UsageSwitch => 0x05,
            Collection::UsageModifier => 0x06,
            Collection::Reserved(i) => i,
            Collection::VendorDefined(i) => i,
        }
    }
}

/// Represents one item in the Report Descriptor. This is a variable-sized
/// element with one header byte and 0, 1, 2, 4 payload bytes.
#[derive(Debug, PartialEq, Clone)]
pub struct ReportDescriptorItem {
    pub(crate) kind: ItemType,
    pub(crate) payload_size: Size,
    pub(crate) raw: Vec<u8>,
}

impl ReportDescriptorItem {
    /// Get the raw payload of the Report Descriptor Item
    pub fn raw_payload(&self) -> SizedPayload {
        SizedPayload::try_from(&self.raw[1..]).unwrap()
    }

    /// Get the payload as u8 value
    pub fn payload_u8(&self) -> Option<u8> {
        u8::try_from(self.raw_payload()).ok()
    }

    /// Get the payload as u16 value
    pub fn payload_u16(&self) -> Option<u16> {
        u16::try_from(self.raw_payload()).ok()
    }

    /// Get the payload as u32 value
    pub fn payload_u32(&self) -> u32 {
        u32::from(self.raw_payload())
    }

    /// Get the payload as i32 value
    pub fn payload_i32(&self) -> i32 {
        i32::from(self.raw_payload())
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

    /// Determine if current item describes the Report ID
    pub fn is_report_id(&self) -> bool {
        self.kind == ItemType::Global(GlobalType::ReportID)
    }

    /// Determine if current item describes Input
    pub fn is_input(&self) -> bool {
        self.kind == ItemType::Main(MainType::Input)
    }

    /// Determine if current item describes Output
    pub fn is_output(&self) -> bool {
        self.kind == ItemType::Main(MainType::Output)
    }

    /// Determine if current item is a Input, Output or Feature
    pub fn is_input_output_or_feature(&self) -> bool {
        self.is_input() || self.is_output() || self.is_feature()
    }
    /// Determine if current item describes Feature
    pub fn is_feature(&self) -> bool {
        self.kind == ItemType::Main(MainType::Feature)
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

        Some(UsagePage::from(self.payload_u16()?))
    }

    /// Get the Usage given a UsagePage. This function will return None
    /// if the current item does not describe a Usage or if the payload
    /// couldn't be converted.
    pub fn usage(&self, usage_page: &UsagePage) -> Option<Usage> {
        if !self.is_usage() {
            return None;
        }

        let id = self.payload_u16()?;
        Some(Usage::from((usage_page, id)))
    }

    /// Get the Collection type. If this item doesn't describe the Collection
    /// or the payload is not exactly one byte the function will return None.
    pub fn collection(&self) -> Option<Collection> {
        if !self.is_collection() || self.payload_size != Size::One {
            return None;
        }

        Some(Collection::from(self.payload_u8()?))
    }

    /// Get the DataFieldOptions for Input, Output or Feature Items. Will return None
    /// if the current item is not a Input/Output/Feature Item or if the payload can
    /// not be converted to a DataFieldOptions
    pub fn data_field_options(&self) -> Option<DataFieldOptions> {
        if !self.is_input_output_or_feature() {
            return None;
        }

        let payload = self.raw_payload();
        if payload.is_empty() {
            return None;
        }

        data_field_options_from_payload(payload.data().unwrap(), payload.size()).ok()
    }
}

impl<T: Into<ItemType>, U: Into<SizedPayload>> From<(T, U)> for ReportDescriptorItem {
    fn from(value: (T, U)) -> Self {
        let kind: ItemType = value.0.into();
        let payload: SizedPayload = value.1.into();

        let size = payload.size();
        let prefix = PrefixByte::from((&kind, &size));

        ReportDescriptorItem {
            kind,
            payload_size: size,
            raw: prefix.into_vec_append(payload.to_vec()),
        }
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

    /// Filter the items
    pub fn filter_by_type(&self, key: ItemType) -> Vec<&ReportDescriptorItem> {
        self.items.iter().filter(|&item| item.kind == key).collect()
    }

    /// Return all raw bytes
    pub fn bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for mut item in self.items {
            bytes.append(&mut item.raw);
        }

        bytes
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
