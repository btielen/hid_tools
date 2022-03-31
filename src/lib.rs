use nom::{IResult, ToUsize};
use nom::bytes::complete::take;
use nom::combinator::map_res;
use nom::multi::many0;
use crate::hid::{GlobalHidItem, HidItem, HidReportDescriptorItem, LocalHidItem, MainHidItem};

pub mod hid;
pub mod data;
mod later;
pub mod display;
mod payload;

#[derive(Debug, PartialEq)]
enum Size {
    Empty,
    One,
    Two,
    Four,
}

impl TryFrom<u8> for Size {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Size::Empty),
            1 => Ok(Size::One),
            2 => Ok(Size::Two),
            3 => Ok(Size::Four), //Note: a size of 0x3 means payload size 4
            i => Err(ParseError::SizeNotValid(i))
        }
    }
}

impl ToUsize for Size {
    fn to_usize(&self) -> usize {
        match self {
            Size::Empty => 0,
            Size::One => 1,
            Size::Two => 2,
            Size::Four => 4
        }
    }
}

impl TryFrom<u8> for HidItem {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {

            // Main hid items
            // https://www.usb.org/sites/default/files/hid1_11.pdf - page 28
            0b10000000 => Ok(HidItem::Main(MainHidItem::Input)),
            0b10010000 => Ok(HidItem::Main(MainHidItem::Output)),
            0b10110000 => Ok(HidItem::Main(MainHidItem::Feature)),
            0b10100000 => Ok(HidItem::Main(MainHidItem::Collection)),
            0b11000000 => Ok(HidItem::Main(MainHidItem::EndCollection)),

            // Global hid items
            // https://www.usb.org/sites/default/files/hid1_11.pdf - page 35
            0b00000100 => Ok(HidItem::Global(GlobalHidItem::UsagePage)),
            0b00010100 => Ok(HidItem::Global(GlobalHidItem::LogicalMinimum)),
            0b00100100 => Ok(HidItem::Global(GlobalHidItem::LogicalMaximum)),
            0b00110100 => Ok(HidItem::Global(GlobalHidItem::PhysicalMinimum)),
            0b01000100 => Ok(HidItem::Global(GlobalHidItem::PhysicalMaximum)),
            0b01010100 => Ok(HidItem::Global(GlobalHidItem::UnitExponent)),
            0b01100100 => Ok(HidItem::Global(GlobalHidItem::Unit)),
            0b01110100 => Ok(HidItem::Global(GlobalHidItem::ReportSize)),
            0b10000100 => Ok(HidItem::Global(GlobalHidItem::ReportID)),
            0b10010100 => Ok(HidItem::Global(GlobalHidItem::ReportCount)),
            0b10100100 => Ok(HidItem::Global(GlobalHidItem::Push)),
            0b10110100 => Ok(HidItem::Global(GlobalHidItem::Push)),

            // Local hid items
            // https://www.usb.org/sites/default/files/hid1_11.pdf - page 39
            0b00001000 => Ok(HidItem::Local(LocalHidItem::Usage)),
            0b00011000 => Ok(HidItem::Local(LocalHidItem::UsageMinimum)),
            0b00101000 => Ok(HidItem::Local(LocalHidItem::UsageMaximum)),
            0b00111000 => Ok(HidItem::Local(LocalHidItem::DesignatorIndex)),
            0b01001000 => Ok(HidItem::Local(LocalHidItem::DesignatorMinimum)),
            0b01011000 => Ok(HidItem::Local(LocalHidItem::DesignatorMaximum)),
            0b01111000 => Ok(HidItem::Local(LocalHidItem::StringIndex)),
            0b10001000 => Ok(HidItem::Local(LocalHidItem::StringMinimum)),
            0b10011000 => Ok(HidItem::Local(LocalHidItem::StringMaximum)),
            0b10101000 => Ok(HidItem::Local(LocalHidItem::Delimiter)),

            i => Err(ParseError::UnknownHeader(i))
        }
    }
}

pub enum ParseError {
    SizeNotValid(u8),
    UnknownHeader(u8),
}

type ByteInput<'a> = &'a[u8];

/// Parse the size from an BitInput
fn size(input: ByteInput) -> IResult<ByteInput, Size> {
    map_res(
        take(1usize),
        |s: &[u8]| Size::try_from(s[0] & 0x3),
    )(input)
}

/// Parse hid header item
fn hid_item(input: ByteInput) -> IResult<ByteInput, HidItem> {
    map_res(
        take(1usize),
        |hid: &[u8]| HidItem::try_from(hid[0] & 0xfc)
    )(input)
}

/// Parse one HID report descriptor item
fn descriptor_item(input: ByteInput) -> IResult<ByteInput, HidReportDescriptorItem> {

    let (input, header) = take(1usize)(input)?;
    let (_, hid) = hid_item(header)?;
    let (_, size) = size(header)?;

    let size = size.to_usize();
    let (input, payload) = take(size)(input)?;

    let mut payload = payload.to_vec();
    let mut raw = header.to_vec();
    raw.append(&mut payload);

    Ok((input, HidReportDescriptorItem {
        header: hid,
        payload_size: size,
        raw,
    }))
}

fn hid_report(input: ByteInput) -> IResult<ByteInput, Vec<HidReportDescriptorItem>> {
    many0(descriptor_item)(input)
}

#[cfg(test)]
mod tests {
    use nom::Finish;
    use crate::{hid_item, HidReportDescriptorItem, descriptor_item, size, Size, hid_report};
    use crate::hid::{GlobalHidItem, HidItem};

    #[test]
    fn size_of_two() {
        let bytes: [u8; 3] = [0x06, 0xd0, 0xf1];
        let result = size(&bytes);

        assert_eq!(result,
           Ok(
               (
                   &bytes[1..],
                   Size::Two
               )
           )
        )
    }

    #[test]
    fn size_of_three_makes_four() {
        let bytes: [u8; 1] = [0x07];
        let result = size(&bytes);

        assert_eq!(result,
                   Ok(
                       (
                           &bytes[1..],
                           Size::Four
                       )
                   )
        )
    }

    #[test]
    fn usage_page_header() {
        let bytes: [u8; 1] = [0x06];
        let result = hid_item(&bytes);

        assert_eq!(result,
            Ok(
                (
                    &bytes[1..],
                    HidItem::Global(GlobalHidItem::UsagePage)
                )
            )
        );
    }

    #[test]
    fn fido_usage_page() {
        let bytes: [u8; 3] = [0x06, 0xd0, 0xf1];
        let result = descriptor_item(&bytes);

        assert_eq!(result, Ok((&bytes[3..], HidReportDescriptorItem {
            header: HidItem::Global(GlobalHidItem::UsagePage),
            payload_size: 2,
            raw: vec![0x06, 0xd0, 0xf1],
        })))
    }

    #[test]
    fn full_report_for_keyboard() {
        let report_descriptor = [
            0x05, 0x01,                    // Usage Page (Generic Desktop)
            0x09, 0x06,                    // Usage (Keyboard)
            0xa1, 0x01,                    // Collection (Application)
            0x85, 0x01,                    // .Report ID (1)
            0x05, 0x07,                    // .Usage Page (Keyboard)
            0x19, 0xe0,                    // .Usage Minimum (224)
            0x29, 0xe7,                    // .Usage Maximum (231)
            0x15, 0x00,                    // .Logical Minimum (0)
            0x25, 0x01,                    // .Logical Maximum (1)
            0x75, 0x01,                    // .Report Size (1)
            0x95, 0x08,                    // .Report Count (8)
            0x81, 0x02,                    // .Input (Data,Var,Abs)
            0x19, 0x00,                    // .Usage Minimum (0)
            0x29, 0x97,                    // .Usage Maximum (151)
            0x15, 0x00,                    // .Logical Minimum (0)
            0x25, 0x01,                    // .Logical Maximum (1)
            0x75, 0x01,                    // .Report Size (1)
            0x95, 0x98,                    // .Report Count (152)
            0x81, 0x02,                    // .Input (Data,Var,Abs)
            0xc0,                          // End Collection
        ];

        let result = hid_report(&report_descriptor);
        dbg!(result);
    }
}
