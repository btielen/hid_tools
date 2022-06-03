use crate::report_descriptor::data::*;
use crate::usage_table::{Usage, UsagePage};
use parse::data_field_options_from_payload;

/// Parsed descriptor report data
pub mod data;

/// Parse descriptor report items
pub mod parse;

/// HID Descriptor Report item type
#[derive(Debug, PartialEq, Clone)]
pub enum ItemType {
    /// Main item type
    Main(MainType),

    /// Global item type
    Global(GlobalType),

    /// Local item type
    Local(LocalType),
}

/// Main item types
#[derive(Debug, PartialEq, Clone)]
pub enum MainType {
    /// Input item tag
    ///
    /// An Input item describes information about the data provided by one or more
    /// physical controls. An application can use this information to interpret the data
    /// provided by the device. All data fields defined in a single item share an
    /// identical data format.
    Input,

    /// Output item tag
    ///
    /// Refers to the data to one or more similar controls on a device
    /// such as setting the position of a single axis or a group of levers (variable data).
    /// Or, it can represent data to one or more LEDs (array data).
    Output,

    /// Feature item tag
    ///
    /// Describes device input and output not intended for
    /// consumption by the end user —for example, a software feature or Control
    /// Panel toggle.
    Feature,

    /// Collection item tag
    ///
    /// A meaningful grouping of Input, Output, and Feature
    /// items—for example, mouse, keyboard, joystick, and pointer
    Collection,

    /// End Collection item tag
    ///
    /// A terminating item used to specify the end of a
    /// collection of items.
    EndCollection,
}

/// Local item
///
/// Local item tags define characteristics of controls. These items do not carry over to
/// the next Main item. If a Main item defines more than one control, it may be
/// preceded by several similar Local item tags.
#[derive(Debug, PartialEq, Clone)]
pub enum LocalType {
    /// Usage
    ///
    /// Usage index for an item usage; represents a
    /// suggested usage for the item or collection. In the
    /// case where an item represents multiple controls, a
    /// Usage tag may suggest a usage for every variable
    /// or element in an array.
    Usage,

    /// Usage minimum
    ///
    /// Defines the starting usage associated with an array or bitmap.
    UsageMinimum,

    /// Usage maximum
    ///
    /// Defines the ending usage associated with an array
    /// or bitmap.
    UsageMaximum,

    /// Designator Index
    ///
    /// Determines the body part used for a control. Index
    /// points to a designator in the Physical descriptor.
    DesignatorIndex,

    /// Designator Minimum
    ///
    /// Defines the index of the starting designator
    /// associated with an array or bitmap.
    DesignatorMinimum,

    /// Designator Maximum
    ///
    /// Defines the index of the ending designator
    /// associated with an array or bitmap.
    DesignatorMaximum,

    /// String Index
    ///
    /// String index for a String descriptor; allows a string
    /// to be associated with a particular item or control.
    StringIndex,

    /// String Minimum
    ///
    /// Specifies the first string index when assigning a
    /// group of sequential strings to controls in an array
    /// or bitmap.
    StringMinimum,

    /// String maximum
    ///
    /// Specifies the last string index when assigning a
    /// group of sequential strings to controls in an array
    /// or bitmap.
    StringMaximum,

    /// Delimiter
    ///
    /// Defines the beginning or end of a set of local items
    /// (1 = open set, 0 = close set).
    Delimiter,
}

/// Global item
///
/// Global items describe rather than define data from a control. A new Main item
/// assumes the characteristics of the item state table. Global items can change the
/// state table. As a result Global item tags apply to all subsequently defined items
/// unless overridden by another Global item
#[derive(Debug, PartialEq, Clone)]
pub enum GlobalType {
    /// Usage Page
    ///
    /// Describes the Usage Page
    UsagePage,

    /// Logical Minimum
    ///
    /// This is the minimum value that a variable or array item will
    /// report. For example, a mouse reporting x
    /// position values from 0 to 128 would have a
    /// Logical Minimum of 0
    LogicalMinimum,

    /// Logical Maximum
    ///
    /// This is the minimum value that a variable or array item will
    /// report. For example, a mouse reporting x position values from 0 to 128 would have a
    /// Logical Maximum of 128
    LogicalMaximum,

    /// Physical Minimum
    ///
    /// Minimum value for the physical extent of a
    /// variable item. This represents the Logical
    /// Minimum with units applied to it.
    PhysicalMinimum,

    /// Physical Maximum
    ///
    /// Maximum value for the physical extent of a
    /// variable item
    PhysicalMaximum,

    /// Unit Exponent
    ///
    /// Value of the unit exponent in base 10
    UnitExponent,

    /// Unit
    Unit,

    /// Report Size
    ///
    /// Unsigned integer specifying the size of the report fields in bits.
    ReportSize,

    /// Report ID
    ///
    /// Unsigned value that specifies the Report ID. If a
    /// Report ID tag is used anywhere in Report descriptor, all data reports
    /// for the device are preceded by a single byte ID field. A
    ReportID,

    /// Report Count
    ///
    /// Unsigned integer specifying the number of data fields for the item; determines how
    /// many fields are included in the report for this particular item (and consequently
    /// how many bits are added to the report).
    ReportCount,

    /// Push
    ///
    /// Places a copy of the global item state table on the stack.
    Push,

    /// Pop
    ///
    /// Replaces the item state table with the top structure from the stack.
    Pop,
}

/// Data field options for Input, Output and Feature Items
///
/// Input, Output, and Feature items are used to create data fields within a report. Here
/// we define a struct to hold the available options.
/// <https://www.usb.org/sites/default/files/hid1_11.pdf> - page 28
#[derive(Default, Debug, PartialEq, Clone)]
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
    /// Returns true if Mutability is data
    pub fn is_data(&self) -> bool {
        self.0 == Mutability::Data
    }

    /// Returns true if Mutability is constant
    pub fn is_constant(&self) -> bool {
        self.0 == Mutability::Constant
    }

    /// Returns true if Structure is array
    pub fn is_array(&self) -> bool {
        self.1 == Structure::Array
    }

    /// Returns true if Structure is variable
    pub fn is_var(&self) -> bool {
        self.1 == Structure::Variable
    }

    /// Returns true is Value is absolute
    pub fn is_absolute(&self) -> bool {
        self.2 == Value::Absolute
    }

    /// Returns true if Value is relative
    pub fn is_relative(&self) -> bool {
        self.2 == Value::Relative
    }

    /// Get the Mutability
    pub fn mutability(&self) -> &Mutability {
        &self.0
    }

    /// Get the structure
    pub fn structure(&self) -> &Structure {
        &self.1
    }

    /// Get the value (absolute or relative)
    pub fn value(&self) -> &Value {
        &self.2
    }

    /// Get the Wrap
    pub fn wrap(&self) -> &Wrap {
        &self.3
    }

    /// Get the Linear
    pub fn linear(&self) -> &Linear {
        &self.4
    }

    /// Get the State
    pub fn state(&self) -> &State {
        &self.5
    }

    /// Get the NullState
    pub fn null_state(&self) -> &NullState {
        &self.6
    }

    /// Get the Volatile
    pub fn volatile(&self) -> &Volatile {
        &self.7
    }

    /// Get the Data enum
    pub fn data(&self) -> &Data {
        &self.8
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

/// Mutability indicates whether the item is data or a constant value
///
/// For definition see <https://www.usb.org/sites/default/files/hid1_11.pdf> - page 30
#[derive(Debug, PartialEq, Clone)]
pub enum Mutability {
    /// Data indicates the item is defining report
    /// fields that contain modifiable device data
    Data,

    /// Indicates the item is a static read-only field in a
    /// report and cannot be modified (written) by the
    /// host.
    Constant,
}

impl Default for Mutability {
    fn default() -> Self {
        Mutability::Data
    }
}

/// Indicates whether the item creates variable or array
/// data fields in reports.
///
/// For definition in the HID protocol see
/// <https://www.usb.org/sites/default/files/hid1_11.pdf> - page 30
#[derive(Debug, PartialEq, Clone)]
pub enum Structure {
    /// An array provides an alternate means for
    /// describing the data returned from a group of
    /// buttons.
    Array,

    /// In variable fields, each field
    /// represents data from a physical control.
    Variable,
}

impl Default for Structure {
    fn default() -> Self {
        Structure::Array
    }
}

/// Indicates whether the data is absolute (based on a fixed origin) or
/// relative (indicating the change in value from the last report).
///
/// For definition in the HID protocol see
/// <https://www.usb.org/sites/default/files/hid1_11.pdf> - page 30
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    /// Absolute value based on a fixed origin
    Absolute,

    /// Relative to the value from the last report
    Relative,
}

impl Default for Value {
    fn default() -> Self {
        Value::Absolute
    }
}

/// Indicates whether the data “rolls over” when
/// reaching either the extreme high or low value.
///
/// For definition in the HID protocol see
/// <https://www.usb.org/sites/default/files/hid1_11.pdf> - page 31
#[derive(Debug, PartialEq, Clone)]
pub enum Wrap {
    /// No wrapping of the value
    NoWrap,

    /// Wrap the value
    Wrap,
}

impl Default for Wrap {
    fn default() -> Self {
        Wrap::NoWrap
    }
}

/// Indicates whether the raw data from the device has been processed in some way, and no longer
/// represents a linear relationship between what is measured and the data that is reported.
///
/// For definition in the HID protocol see
/// <https://www.usb.org/sites/default/files/hid1_11.pdf> - page 31
#[derive(Debug, PartialEq, Clone)]
pub enum Linear {
    /// Linear data
    Linear,

    /// Non linear data
    NonLinear,
}

impl Default for Linear {
    fn default() -> Self {
        Linear::Linear
    }
}

/// Indicates whether the control has a preferred state to which it will return
/// when the user is not physically interacting with the control.
///
/// For definition in the HID protocol see
/// <https://www.usb.org/sites/default/files/hid1_11.pdf> - page 31
#[derive(Debug, PartialEq, Clone)]
pub enum State {
    /// Control has a preferred state
    Preferred,

    /// Control doesn't have a preferred state (push buttons for example)
    NoPreferred,
}

impl Default for State {
    fn default() -> Self {
        State::Preferred
    }
}

/// Indicates whether the control has a state in which it
/// is not sending meaningful data.
///
/// For definition in the HID protocol see
/// <https://www.usb.org/sites/default/files/hid1_11.pdf> - page 31
#[derive(Debug, PartialEq, Clone)]
pub enum NullState {
    /// Control doesn't have a null state
    NoNullPosition,

    /// Control has a null state
    NullState,
}

impl Default for NullState {
    fn default() -> Self {
        NullState::NoNullPosition
    }
}

/// Indicates whether the Feature or Output control's
/// value should be changed by the host or not.
///
/// For definition in the HID protocol see
/// <https://www.usb.org/sites/default/files/hid1_11.pdf> - page 31
#[derive(Debug, PartialEq, Clone)]
pub enum Volatile {
    /// Non volatile data
    NonVolatile,

    /// Volatile output can change with or without host interaction.
    Volatile,
}

impl Default for Volatile {
    fn default() -> Self {
        Volatile::NonVolatile
    }
}

/// Indicates that the control emits a fixed-size stream of bytes.
///
/// For definition in the HID protocol see
/// <https://www.usb.org/sites/default/files/hid1_11.pdf> - page 31
#[derive(Debug, PartialEq, Clone)]
pub enum Data {
    /// Fixed size data
    BitField,

    /// Bytes
    BufferedBytes,
}

impl Default for Data {
    fn default() -> Self {
        Data::BitField
    }
}

/// The collection main item
///
/// For definition in the HID protocol see
/// <https://www.usb.org/sites/default/files/hid1_11.pdf> - page 28
#[derive(Debug, PartialEq, Clone)]
pub enum Collection {
    /// Physical collection
    Physical,

    /// Application collection (mouse or keyboard for example)
    Application,

    /// Logical collection
    Logical,

    /// Report collection
    Report,

    /// Named array collection
    NamedArray,

    /// Usage switch collection
    UsageSwitch,

    /// Usage modifier collection
    UsageModifier,

    /// Reserved for future use
    Reserved(u8),

    /// A vendor defined collection
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

/// Represents one item in the Report Descriptor
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

/// HID Report Descriptor is a list of ReportDescriptorItem's
#[derive(Debug, PartialEq)]
pub struct ReportDescriptor {
    items: Vec<ReportDescriptorItem>,
}

impl ReportDescriptor {
    /// Create a new ReportDescriptor
    pub fn new(items: Vec<ReportDescriptorItem>) -> Self {
        ReportDescriptor { items }
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
    use crate::report_descriptor::Collection;

    #[test]
    fn collection_from_u8_reserved() {
        assert_eq!(Collection::from(0x08), Collection::Reserved(8));
    }

    #[test]
    fn collection_from_u8_vendor_defined() {
        assert_eq!(Collection::from(0xF0), Collection::VendorDefined(0xF0))
    }
}
