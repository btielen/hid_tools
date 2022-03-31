use crate::usage_table::{Usage, UsagePage};

/// Data payload in a HID Report Descriptor Item
#[derive(Clone)]
pub struct Payload<'a> {
    data: &'a [u8],
}

/// The payload of an HID Report Descriptor Item can only be 0, 1, 2 or 4 bytes.
/// Note: a raw size of 0x3 means payload size 4
#[derive(Debug, PartialEq)]
pub enum Size {
    Empty,
    One,
    Two,
    Four,
    //Long(u8) unimplemented
}

impl<'a> Payload<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Payload { data }
    }

    pub fn data(&self) -> &[u8] {
        self.data
    }
}

impl<'a> TryFrom<Payload<'a>> for u16 {
    type Error = ();

    fn try_from(value: Payload<'a>) -> Result<Self, Self::Error> {
        match value.data.len() {
            0 => Ok(0),
            1 => Ok(value.data[0] as u16),
            2 => Ok(u16::from_le_bytes(value.data.try_into().unwrap())),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<Payload<'a>> for u32 {
    type Error = ();

    fn try_from(value: Payload<'a>) -> Result<Self, Self::Error> {
        match value.data.len() {
            0 => Ok(0),
            1 => Ok(value.data[0] as u32),
            2 => Ok(u32::from(u16::try_from(value).unwrap())),
            4 => Ok(u32::from_le_bytes(value.data.try_into().unwrap())),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<Payload<'a>> for UsagePage {
    type Error = String;

    fn try_from(value: Payload<'a>) -> Result<Self, Self::Error> {
        let u16 =
            u16::try_from(value).map_err(|_| "Payload can not be converted to u16".to_string())?;

        Ok(Self::from(u16))
    }
}

impl<'a> TryFrom<(&UsagePage, Payload<'a>)> for Usage {
    type Error = String;

    fn try_from(key: (&UsagePage, Payload<'a>)) -> Result<Self, Self::Error> {
        let usage_u16 = u16::try_from(key.1)
            .map_err(|_| "Could not convert payload to u16 integer".to_string())?;

        Ok(Usage::from((key.0, usage_u16)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u16_empty_payload() {
        let data = Vec::new();
        let p = Payload::new(&data);

        assert_eq!(u16::try_from(p), Ok(0));
    }

    #[test]
    fn u16_one_payload_byte() {
        let data = vec![0x05];
        let p = Payload::new(&data);

        assert_eq!(u16::try_from(p), Ok(5))
    }

    #[test]
    fn u16_two_payload_bytes() {
        let data = vec![0x10, 0x02];
        let p = Payload::new(&data);

        assert_eq!(u16::try_from(p), Ok(528));
    }

    #[test]
    fn u16_too_many_bytes() {
        let data = vec![0xf0, 0x20, 0x10];
        let p = Payload::new(&data);
        assert!(u16::try_from(p).is_err());
    }

    #[test]
    fn u32_some_bytes() {
        let data = vec![0xf0, 0x20, 0x10, 0x01];
        let p = Payload::new(&data);
        assert_eq!(u32::try_from(p), Ok(17834224));
    }
}
