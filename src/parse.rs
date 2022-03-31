use crate::error::Error;
use crate::hid::{
    GlobalType, ItemType, LocalType, MainType, ReportDescriptorItem, ReportDescriptorItemList,
};
use crate::payload::Size;
use nom::bytes::complete::take;
use nom::combinator::{all_consuming, map, map_res};
use nom::error::{
    context, ContextError, FromExternalError, ParseError, VerboseError, VerboseErrorKind,
};
use nom::multi::many0;
use nom::{Finish, IResult, ToUsize};

/// Parse the payload size from the first byte
fn size<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], Size, E>
where
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]> + FromExternalError<&'a [u8], MapResultError>,
{
    context(
        "parse size from prefix byte",
        map_res(take(1usize), |s: &[u8]| match s[0] & 0x3 {
            0 => Ok(Size::Empty),
            1 => Ok(Size::One),
            2 => Ok(Size::Two),
            3 => Ok(Size::Four), // Note: a size of 0x3 means payload size 4
            _n => Err(MapResultError::Impossible), // This will never happen because s[0] & 0x3 <= 3
        }),
    )(input)
}

/// Parse the HID Report Descriptor Item Type from the first byte
fn item_type<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], ItemType, E>
where
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]> + FromExternalError<&'a [u8], MapResultError>,
{
    context(
        "parse hid item type from prefix byte",
        map_res(take(1usize), |hid: &[u8]| match hid[0] & 0xfc {
            // Main hid items
            // https://www.usb.org/sites/default/files/hid1_11.pdf - page 28
            0b10000000 => Ok(ItemType::Main(MainType::Input)),
            0b10010000 => Ok(ItemType::Main(MainType::Output)),
            0b10110000 => Ok(ItemType::Main(MainType::Feature)),
            0b10100000 => Ok(ItemType::Main(MainType::Collection)),
            0b11000000 => Ok(ItemType::Main(MainType::EndCollection)),

            // Global hid items
            // https://www.usb.org/sites/default/files/hid1_11.pdf - page 35
            0b00000100 => Ok(ItemType::Global(GlobalType::UsagePage)),
            0b00010100 => Ok(ItemType::Global(GlobalType::LogicalMinimum)),
            0b00100100 => Ok(ItemType::Global(GlobalType::LogicalMaximum)),
            0b00110100 => Ok(ItemType::Global(GlobalType::PhysicalMinimum)),
            0b01000100 => Ok(ItemType::Global(GlobalType::PhysicalMaximum)),
            0b01010100 => Ok(ItemType::Global(GlobalType::UnitExponent)),
            0b01100100 => Ok(ItemType::Global(GlobalType::Unit)),
            0b01110100 => Ok(ItemType::Global(GlobalType::ReportSize)),
            0b10000100 => Ok(ItemType::Global(GlobalType::ReportID)),
            0b10010100 => Ok(ItemType::Global(GlobalType::ReportCount)),
            0b10100100 => Ok(ItemType::Global(GlobalType::Push)),
            0b10110100 => Ok(ItemType::Global(GlobalType::Push)),

            // Local hid items
            // https://www.usb.org/sites/default/files/hid1_11.pdf - page 39
            0b00001000 => Ok(ItemType::Local(LocalType::Usage)),
            0b00011000 => Ok(ItemType::Local(LocalType::UsageMinimum)),
            0b00101000 => Ok(ItemType::Local(LocalType::UsageMaximum)),
            0b00111000 => Ok(ItemType::Local(LocalType::DesignatorIndex)),
            0b01001000 => Ok(ItemType::Local(LocalType::DesignatorMinimum)),
            0b01011000 => Ok(ItemType::Local(LocalType::DesignatorMaximum)),
            0b01111000 => Ok(ItemType::Local(LocalType::StringIndex)),
            0b10001000 => Ok(ItemType::Local(LocalType::StringMinimum)),
            0b10011000 => Ok(ItemType::Local(LocalType::StringMaximum)),
            0b10101000 => Ok(ItemType::Local(LocalType::Delimiter)),
            i => Err(MapResultError::PrefixTypeInvalid(i)),
        }),
    )(input)
}

/// Parse a long item
fn long_item_size_and_type<'a, E>(_input: &'a [u8]) -> IResult<&'a [u8], (Size, ItemType), E>
where
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]> + FromExternalError<&'a [u8], MapResultError>,
{
    unimplemented!();
    // From specs: "No long item tags are defined in this document. These
    // tags are reserved for future use."
    // Page 27 (https://www.usb.org/sites/default/files/hid1_11.pdf)
}

/// Parse size and byte from first byte
fn size_and_type<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], (Size, ItemType), E>
where
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]> + FromExternalError<&'a [u8], MapResultError>,
{
    let (input, prefix) = context("take prefix byte", take(1usize))(input)?;

    // From long item
    if prefix[0] == 0b11111110 {
        return long_item_size_and_type(input);
    }

    // Both item type and size are in the first bye
    let (_, hid) = item_type(prefix)?;
    let (_, size) = size(prefix)?;

    Ok((input, (size, hid)))
}

/// Parse one HID report descriptor item
fn descriptor_item<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], ReportDescriptorItem, E>
where
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]> + FromExternalError<&'a [u8], MapResultError>,
{
    // Get size and type from first byte
    let (input, prefix) = context("take prefix byte", take(1usize))(input)?;
    let (_, (size, hid)) = size_and_type(prefix)?;

    // Get the next few bytes as payload
    let (input, payload) = context("take payload bytes", take(&size))(input)?;

    // Clone all taken bytes into raw vector
    let mut payload = payload.to_vec();
    let mut raw = prefix.to_vec();
    raw.append(&mut payload);

    Ok((
        input,
        ReportDescriptorItem {
            kind: hid,
            payload_size: size,
            raw,
        },
    ))
}

/// Parse all bytes from input into a ReportDescriptorItemList
fn full_report_descriptor<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], ReportDescriptorItemList, E>
where
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]> + FromExternalError<&'a [u8], MapResultError>,
{
    context(
        "parse all bytes to report descriptor",
        map(all_consuming(many0(descriptor_item)), |items| {
            ReportDescriptorItemList::new(items)
        }),
    )(input)
}

/// Parse all bytes as a vector of ReportDescriptorItem's
pub fn report_descriptor(input: &[u8]) -> Result<ReportDescriptorItemList, Error> {
    // Parse report descriptor or result in a VerboseError
    let result = full_report_descriptor::<VerboseError<&[u8]>>(input);

    result
        .finish()
        .map(|r| {
            assert!(r.0.is_empty()); // Verify that all the bytes have been successfully parsed
            r.1
        })
        .map_err(|e| Error::ParsingFailed(user_friendly_error(e)))
}

/// Errors for converting size and Hid Item Type
enum MapResultError {
    PrefixTypeInvalid(u8),
    Impossible,
}

/// Implementation of ToUsize, used for taking the payload bytes
impl ToUsize for &Size {
    fn to_usize(&self) -> usize {
        match self {
            Size::Empty => 0,
            Size::One => 1,
            Size::Two => 2,
            Size::Four => 4,
        }
    }
}

/// Attempt to create an error message that is better understandable by humans
fn user_friendly_error(error: VerboseError<&[u8]>) -> String {
    let mut message = Vec::new();

    for e in error.errors.into_iter() {
        match &e.1 {
            VerboseErrorKind::Context(context) => {
                message.push(format!("For input {:04x?} could not {} \n\n", e.0, context))
            }
            //_ => {}
            VerboseErrorKind::Char(c) => message.push(format!("'{}':\t{:?}\n", c, e.0)),
            VerboseErrorKind::Nom(kind) => message.push(format!("{:?}:\t{:?}\n", kind, e.0)),
        };
    }

    message.into_iter().collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::error::Error::ParsingFailed;
    use crate::hid::GlobalType;

    type SimpleError<'a> = nom::error::Error<&'a [u8]>;
    type VerboseError<'a> = nom::error::VerboseError<&'a [u8]>;

    #[test]
    fn size_of_two() {
        let bytes: Vec<u8> = vec![0x06, 0xd0, 0xf1];
        let result = size::<SimpleError>(&bytes);

        assert_eq!(result, Ok((&bytes[1..], Size::Two)))
    }

    #[test]
    fn size_of_three_makes_four() {
        let bytes: Vec<u8> = vec![0x03];
        let result = size::<SimpleError>(&bytes);

        assert_eq!(result, Ok((&bytes[1..], Size::Four)))
    }

    #[test]
    fn usage_page_header() {
        let bytes: [u8; 1] = [0x06];
        let result = item_type::<SimpleError>(&bytes);

        assert_eq!(
            result,
            Ok((&bytes[1..], ItemType::Global(GlobalType::UsagePage)))
        );
    }

    #[test]
    fn fido_usage_page() {
        let bytes: Vec<u8> = vec![0x06, 0xd0, 0xf1];
        let result = descriptor_item::<VerboseError>(&bytes);

        assert_eq!(
            result,
            Ok((
                &bytes[3..],
                ReportDescriptorItem {
                    kind: ItemType::Global(GlobalType::UsagePage),
                    payload_size: Size::Two,
                    raw: vec![0x06, 0xd0, 0xf1],
                }
            ))
        )
    }

    #[test]
    fn fido_size_and_type() {
        let bytes: Vec<u8> = vec![0x06, 0xd0, 0xf1];
        let result = size_and_type::<VerboseError>(&bytes);

        assert_eq!(
            result,
            Ok((
                &bytes[1..],
                (Size::Two, ItemType::Global(GlobalType::UsagePage))
            ))
        )
    }

    #[test]
    fn item_type_failure() {
        let bytes: Vec<u8> = vec![0xff];
        let result = item_type::<SimpleError>(&bytes);

        assert!(result.is_err())
    }

    #[test]
    fn full_report_failure() {
        let bytes: Vec<u8> = vec![0x06, 0xd0, 0xf1, 0xff, 0x09, 0x01];
        let result = full_report_descriptor::<VerboseError>(&bytes);

        assert!(result.is_err())
    }

    #[test]
    fn report_error_string() {
        let bytes: Vec<u8> = vec![0x06, 0xd0, 0xf1, 0xff, 0x09, 0x01];
        let result = report_descriptor(&bytes).expect_err("bytes is not valid");

        assert_eq!(
            result,
            ParsingFailed("Eof:\t[255, 9, 1]\nFor input [0006, 00d0, 00f1, 00ff, 0009, 0001] could not parse all bytes to report descriptor \n\n".to_string())
        )
    }
}
