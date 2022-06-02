use crate::report_descriptor::data::SizedPayload;
use crate::report_descriptor::{
    Collection, GlobalType, ItemType, LocalType, MainType, ReportDescriptor, ReportDescriptorItem,
};
use crate::usage_table::{UsageId, UsagePage};

/// A builder for a ReportDescriptor
///
/// # Example
///
/// ```rust
/// use hid_tools::report_builder::ReportDescriptorBuilder;
/// use hid_tools::report_descriptor::Collection;
/// use hid_tools::usage_table::{UsagePage};
/// use hid_tools::usage_table::generic_desktop::GenericDesktopControlsUsage;
///
///     let raw_report = ReportDescriptorBuilder::new()
///         .usage_page(UsagePage::GenericDesktopControls)
///         .usage(GenericDesktopControlsUsage::Mouse)
///         .item(Collection::Application)
///         // add more items here (see also examples)
///         .end_collection()
///         .build()
///         .bytes();
///
///     println!("{:02x?}", raw_report)
/// ```
#[derive(Debug, PartialEq)]
pub struct ReportDescriptorBuilder {
    items: Vec<ReportDescriptorItem>,
}

impl ReportDescriptorBuilder {
    /// Create a new DescriptorReportBuilder
    pub fn new() -> Self {
        ReportDescriptorBuilder::default()
    }

    /* MAIN ITEMS */

    /// Add Input item
    pub fn input<T: Into<SizedPayload>>(self, value: T) -> Self {
        let kind = ItemType::Main(MainType::Input);
        self.item_with_payload(kind, value)
    }

    /// Add Output item
    pub fn output<T: Into<SizedPayload>>(self, value: T) -> Self {
        let kind = ItemType::Main(MainType::Output);
        self.item_with_payload(kind, value)
    }

    /// Add Feature item
    pub fn feature<T: Into<SizedPayload>>(self, value: T) -> Self {
        let kind = ItemType::Main(MainType::Feature);
        self.item_with_payload(kind, value)
    }

    /// Add a Collection item
    pub fn collection<T: Into<u8>>(self, collection: T) -> Self {
        let kind = ItemType::Main(MainType::Collection);
        self.item_with_payload(kind, SizedPayload::from(collection.into()))
    }

    /// Add an End Collection item
    pub fn end_collection(self) -> Self {
        let kind = ItemType::Main(MainType::EndCollection);
        self.item_with_payload(kind, SizedPayload::Empty)
    }

    /* GLOBAL ITEMS */

    /// Add Usage Page item
    pub fn usage_page<T: Into<u16>>(self, usage_page: T) -> Self {
        let kind = ItemType::Global(GlobalType::UsagePage);
        self.item_with_payload(kind, SizedPayload::from(usage_page.into()))
    }

    /// Add Logical Minimum item
    pub fn logical_minimum<T: Into<SizedPayload>>(self, value: T) -> Self {
        let kind = ItemType::Global(GlobalType::LogicalMinimum);
        self.item_with_payload(kind, value)
    }

    /// Add Logical Maximum item
    pub fn logical_maximum<T: Into<SizedPayload>>(self, value: T) -> Self {
        let kind = ItemType::Global(GlobalType::LogicalMaximum);
        self.item_with_payload(kind, value)
    }
    /// Add Physical Minimum item
    pub fn physical_minimum<T: Into<SizedPayload>>(self, value: T) -> Self {
        let kind = ItemType::Global(GlobalType::PhysicalMinimum);
        self.item_with_payload(kind, value)
    }

    /// Add Physical Maximum item
    pub fn physical_maximum<T: Into<SizedPayload>>(self, value: T) -> Self {
        let kind = ItemType::Global(GlobalType::PhysicalMaximum);
        self.item_with_payload(kind, value)
    }

    /// Add Unit Exponent item
    pub fn unit_exponent<T: Into<SizedPayload>>(self, value: T) -> Self {
        let kind = ItemType::Global(GlobalType::UnitExponent);
        self.item_with_payload(kind, value)
    }

    /// Add Unit item
    pub fn unit<T: Into<u8>>(self, value: T) -> Self {
        let kind = ItemType::Global(GlobalType::Unit);
        self.item_with_payload(kind, value.into())
    }

    /// Add Report Size item
    pub fn report_size<T: Into<SizedPayload>>(self, value: T) -> Self {
        let kind = ItemType::Global(GlobalType::ReportSize);
        self.item_with_payload(kind, value)
    }

    /// Add Report ID item
    pub fn report_id<T: Into<SizedPayload>>(self, value: T) -> Self {
        let kind = ItemType::Global(GlobalType::ReportID);
        self.item_with_payload(kind, value)
    }

    /// Add Report Count item
    pub fn report_count<T: Into<SizedPayload>>(self, value: T) -> Self {
        let kind = ItemType::Global(GlobalType::ReportCount);
        self.item_with_payload(kind, value)
    }

    /// Add Push item
    pub fn push<T: Into<SizedPayload>>(self, value: T) -> Self {
        let kind = ItemType::Global(GlobalType::Push);
        self.item_with_payload(kind, value)
    }

    /// Add Pop item
    pub fn pop<T: Into<SizedPayload>>(self, value: T) -> Self {
        let kind = ItemType::Global(GlobalType::Pop);
        self.item_with_payload(kind, value)
    }

    /* LOCAL ITEMS */

    /// Add an Usage item
    pub fn usage<T: Into<u16>>(self, usage: T) -> Self {
        let kind = ItemType::Local(LocalType::Usage);
        self.item_with_payload(kind, SizedPayload::from(usage.into()))
    }

    /// Add an Usage Minimum item
    pub fn usage_minimum<T: Into<u16>>(self, usage: T) -> Self {
        let kind = ItemType::Local(LocalType::UsageMinimum);
        self.item_with_payload(kind, SizedPayload::from(usage.into()))
    }

    /// Add an Usage Maximum item
    pub fn usage_maximum<T: Into<u16>>(self, usage: T) -> Self {
        let kind = ItemType::Local(LocalType::UsageMaximum);
        self.item_with_payload(kind, SizedPayload::from(usage.into()))
    }

    /// Add Designator Index item
    pub fn designator_index<T: Into<SizedPayload>>(self, value: T) -> Self {
        let kind = ItemType::Local(LocalType::DesignatorIndex);
        self.item_with_payload(kind, value)
    }

    /// Add Designator Minimum item
    pub fn designator_minimum<T: Into<SizedPayload>>(self, value: T) -> Self {
        let kind = ItemType::Local(LocalType::DesignatorMinimum);
        self.item_with_payload(kind, value)
    }

    /// Add Designator Maximum item
    pub fn designator_maximum<T: Into<SizedPayload>>(self, value: T) -> Self {
        let kind = ItemType::Local(LocalType::DesignatorMaximum);
        self.item_with_payload(kind, value)
    }

    /// Add String Index item
    pub fn string_index<T: Into<SizedPayload>>(self, value: T) -> Self {
        let kind = ItemType::Local(LocalType::StringIndex);
        self.item_with_payload(kind, value)
    }

    /// Add String Minimum item
    pub fn string_minimum<T: Into<SizedPayload>>(self, value: T) -> Self {
        let kind = ItemType::Local(LocalType::StringMinimum);
        self.item_with_payload(kind, value)
    }

    /// Add String Maximum item
    pub fn string_maximum<T: Into<SizedPayload>>(self, value: T) -> Self {
        let kind = ItemType::Local(LocalType::StringMaximum);
        self.item_with_payload(kind, value)
    }

    /// Add Delimiter item
    pub fn delimiter<T: Into<SizedPayload>>(self, value: T) -> Self {
        let kind = ItemType::Local(LocalType::Delimiter);
        self.item_with_payload(kind, value)
    }

    /* HELPERS */

    /// Add a ReportDescriptorItem
    fn push_item(mut self, item: ReportDescriptorItem) -> Self {
        self.items.push(item);

        self
    }

    /// Add an item with payload
    pub fn item_with_payload<T, U>(self, item_type: T, payload: U) -> Self
    where
        T: Into<ItemType>,
        U: Into<SizedPayload>,
    {
        let item = ReportDescriptorItem::from((item_type.into(), payload.into()));
        self.push_item(item)
    }

    /// Add an item
    pub fn item<T>(self, item: T) -> Self
    where
        T: Into<ItemType> + Into<SizedPayload> + Clone,
    {
        let kind: ItemType = item.clone().into();
        let payload: SizedPayload = item.into();

        let item = ReportDescriptorItem::from((kind, payload));
        self.push_item(item)
    }

    /// Build ReportDescriptor
    pub fn build(self) -> ReportDescriptor {
        ReportDescriptor::new(self.items)
    }
}

impl Default for ReportDescriptorBuilder {
    fn default() -> Self {
        ReportDescriptorBuilder { items: Vec::new() }
    }
}

/*


  The next few implementations make it convenient
  to work with the DescriptorReportBuilder


*/

impl From<UsagePage> for ItemType {
    fn from(_: UsagePage) -> Self {
        ItemType::Global(GlobalType::UsagePage)
    }
}

impl From<UsagePage> for SizedPayload {
    fn from(value: UsagePage) -> Self {
        SizedPayload::from(u16::from(value))
    }
}

impl From<Collection> for SizedPayload {
    fn from(value: Collection) -> Self {
        SizedPayload::from(u8::from(value))
    }
}

impl<T: UsageId> From<T> for SizedPayload {
    fn from(value: T) -> Self {
        SizedPayload::from(value.usage_id())
    }
}

impl<T: UsageId> From<T> for ItemType {
    fn from(_: T) -> Self {
        ItemType::Local(LocalType::Usage)
    }
}

impl From<Collection> for ItemType {
    fn from(_: Collection) -> Self {
        ItemType::Main(MainType::Collection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report_descriptor::data::Size;
    use crate::report_descriptor::{GlobalType, ItemType, ReportDescriptorItem};
    use crate::usage_table::keyboard::KeyboardUsage;
    use crate::usage_table::{Usage, UsagePage};

    #[test]
    fn usage_page_from_u16() {
        let builder = ReportDescriptorBuilder::default();
        let result = builder.usage_page::<u16>(0x01);

        assert_eq!(
            result,
            ReportDescriptorBuilder {
                items: vec![ReportDescriptorItem {
                    kind: ItemType::Global(GlobalType::UsagePage),
                    payload_size: Size::One,
                    raw: vec![0x05, 0x01]
                }],
            }
        );
    }

    #[test]
    fn usage_page_from_enum() {
        let builder = ReportDescriptorBuilder::default();
        let result = builder.usage_page(UsagePage::FIDOAlliance);

        assert_eq!(
            result,
            ReportDescriptorBuilder {
                items: vec![ReportDescriptorItem {
                    kind: ItemType::Global(GlobalType::UsagePage),
                    payload_size: Size::Two,
                    raw: vec![0x06, 0xd0, 0xf1]
                }],
            }
        );
    }

    #[test]
    fn usage_page_from_usage_page() {
        let builder = ReportDescriptorBuilder::new();
        let result = builder.item(UsagePage::FIDOAlliance);
        assert_eq!(
            result,
            ReportDescriptorBuilder {
                items: vec![ReportDescriptorItem {
                    kind: ItemType::Global(GlobalType::UsagePage),
                    payload_size: Size::Two,
                    raw: vec![0x06, 0xd0, 0xf1]
                }],
            }
        );
    }

    #[test]
    fn keyboard_usage() {
        let builder = ReportDescriptorBuilder::new();
        let result = builder.item(Usage::Keyboard(KeyboardUsage::KeyboardaandA));
        assert_eq!(
            result,
            ReportDescriptorBuilder {
                items: vec![ReportDescriptorItem {
                    kind: ItemType::Local(LocalType::Usage),
                    payload_size: Size::One,
                    raw: vec![0x09, 0x04]
                }],
            }
        );
    }

    #[test]
    fn keyboard_usage_enum() {
        let builder = ReportDescriptorBuilder::new();
        let result = builder.item(KeyboardUsage::KeyboardaandA);
        assert_eq!(
            result,
            ReportDescriptorBuilder {
                items: vec![ReportDescriptorItem {
                    kind: ItemType::Local(LocalType::Usage),
                    payload_size: Size::One,
                    raw: vec![0x09, 0x04]
                }],
            }
        );
    }

    #[test]
    fn logical_minimum_negative() {
        let result = ReportDescriptorBuilder::new().logical_minimum(-1);

        assert_eq!(
            result,
            ReportDescriptorBuilder {
                items: vec![ReportDescriptorItem {
                    kind: ItemType::Global(GlobalType::LogicalMinimum),
                    payload_size: Size::One,
                    raw: vec![0b00010100 | 1, 0xff]
                }],
            }
        );
    }
}
