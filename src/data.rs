use crate::hid::{GlobalType, ItemType, LocalType, MainType};

#[derive(Clone, Debug, PartialEq)]
pub enum SizedPayload {
    Empty,
    One([u8; 1]),
    Two([u8; 2]),
    Four([u8; 4]),
}

impl SizedPayload {
    pub fn size(&self) -> Size {
        Size::from(self)
    }

    /// Get data. Will return None if
    pub fn data(&self) -> Option<&[u8]> {
        match self {
            SizedPayload::Empty => None,
            SizedPayload::One(data) => Some(&data[..]),
            SizedPayload::Two(data) => Some(&data[..]),
            SizedPayload::Four(data) => Some(&data[..]),
        }
    }

    /// Convert payload to owned vector by consuming it's data
    pub fn to_vec(self) -> Vec<u8> {
        match self {
            SizedPayload::Empty => Vec::new(),
            SizedPayload::One(p) => p.to_vec(),
            SizedPayload::Two(p) => p.to_vec(),
            SizedPayload::Four(p) => p.to_vec(),
        }
    }

    /// Returns true if self is empty
    pub fn is_empty(&self) -> bool {
        self.data().is_none()
    }
}

impl From<u8> for SizedPayload {
    fn from(value: u8) -> Self {
        SizedPayload::One([value; 1])
    }
}

impl From<u16> for SizedPayload {
    fn from(value: u16) -> Self {
        if value <= u8::MAX as u16 {
            let u8 = u8::try_from(value).unwrap();
            return SizedPayload::from(u8);
        }

        SizedPayload::Two(value.to_le_bytes())
    }
}

impl From<u32> for SizedPayload {
    fn from(value: u32) -> Self {
        if value <= u8::MAX as u32 {
            let u8 = u8::try_from(value).unwrap();
            return SizedPayload::from(u8);
        }

        if value <= u16::MAX as u32 {
            let u16 = u16::try_from(value).unwrap();
            return SizedPayload::from(u16);
        }

        SizedPayload::Four(value.to_le_bytes())
    }
}

impl From<i8> for SizedPayload {
    fn from(value: i8) -> Self {
        SizedPayload::One(value.to_le_bytes())
    }
}

impl From<i16> for SizedPayload {
    fn from(value: i16) -> Self {
        if i8::MIN as i16 <= value && value <= i8::MAX as i16 {
            let i8 = i8::try_from(value).unwrap();
            return SizedPayload::from(i8);
        }

        SizedPayload::Two(value.to_le_bytes())
    }
}

impl From<i32> for SizedPayload {
    fn from(value: i32) -> Self {
        if i8::MIN as i32 <= value && value <= i8::MAX as i32 {
            let i8 = i8::try_from(value).unwrap();
            return SizedPayload::from(i8);
        }

        if i16::MIN as i32 <= value && value <= i16::MAX as i32 {
            let i16 = i16::try_from(value).unwrap();
            return SizedPayload::from(i16);
        }

        SizedPayload::Four(value.to_le_bytes())
    }
}

impl From<[u8; 0]> for SizedPayload {
    fn from(_: [u8; 0]) -> Self {
        SizedPayload::Empty
    }
}

impl From<[u8; 1]> for SizedPayload {
    fn from(value: [u8; 1]) -> Self {
        SizedPayload::One(value)
    }
}

impl From<[u8; 2]> for SizedPayload {
    fn from(value: [u8; 2]) -> Self {
        SizedPayload::Two(value)
    }
}

impl From<[u8; 4]> for SizedPayload {
    fn from(value: [u8; 4]) -> Self {
        SizedPayload::Four(value)
    }
}
impl TryFrom<&[u8]> for SizedPayload {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value.len() {
            0 => Ok(SizedPayload::Empty),
            1 => {
                let p: [u8; 1] = value.try_into().unwrap();
                Ok(SizedPayload::from(p))
            }
            2 => {
                let p: [u8; 2] = value.try_into().unwrap();
                Ok(SizedPayload::from(p))
            }
            4 => {
                let p: [u8; 4] = value.try_into().unwrap();
                Ok(SizedPayload::from(p))
            }
            _ => Err(()),
        }
    }
}

impl TryFrom<SizedPayload> for u8 {
    type Error = ();

    fn try_from(value: SizedPayload) -> Result<Self, Self::Error> {
        match value {
            SizedPayload::One(data) => Ok(u8::from_le_bytes(data)),
            _ => Err(()),
        }
    }
}

impl TryFrom<SizedPayload> for u16 {
    type Error = ();

    fn try_from(value: SizedPayload) -> Result<Self, Self::Error> {
        match value {
            SizedPayload::One(data) => Ok(u16::from(data[0])),
            SizedPayload::Two(data) => Ok(u16::from_le_bytes(data)),
            _ => Err(()),
        }
    }
}

impl From<SizedPayload> for i32 {
    fn from(value: SizedPayload) -> Self {
        match value {
            SizedPayload::Empty => 0,
            SizedPayload::One(data) => i8::from_le_bytes(data).into(),
            SizedPayload::Two(data) => i16::from_le_bytes(data).into(),
            SizedPayload::Four(data) => i32::from_le_bytes(data),
        }
    }
}

impl From<SizedPayload> for u32 {
    fn from(value: SizedPayload) -> Self {
        match value {
            SizedPayload::Empty => 0,
            SizedPayload::One(data) => u8::from_le_bytes(data).into(),
            SizedPayload::Two(data) => u16::from_le_bytes(data).into(),
            SizedPayload::Four(data) => u32::from_le_bytes(data),
        }
    }
}

impl Default for SizedPayload {
    fn default() -> Self {
        Self::Empty
    }
}

/// The payload of an HID Report Descriptor Item can only be 0, 1, 2 or 4 bytes.
/// Note: a raw size of 0x3 means payload size 4
#[derive(Debug, PartialEq, Clone)]
pub enum Size {
    Empty,
    One,
    Two,
    Four,
    //Long(u8) unimplemented
}

impl From<&[u8; 0]> for Size {
    fn from(_: &[u8; 0]) -> Self {
        Self::Empty
    }
}

impl From<&[u8; 1]> for Size {
    fn from(_: &[u8; 1]) -> Self {
        Self::One
    }
}

impl From<&[u8; 2]> for Size {
    fn from(_: &[u8; 2]) -> Self {
        Self::Two
    }
}

impl From<&[u8; 4]> for Size {
    fn from(_: &[u8; 4]) -> Self {
        Self::Four
    }
}

impl From<&SizedPayload> for Size {
    fn from(value: &SizedPayload) -> Self {
        match value {
            SizedPayload::Empty => Size::Empty,
            SizedPayload::One(_) => Size::One,
            SizedPayload::Two(_) => Size::Two,
            SizedPayload::Four(_) => Size::Four,
        }
    }
}

impl TryFrom<&[u8]> for Size {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value.len() {
            0 => Ok(Self::Empty),
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            4 => Ok(Self::Four),
            _ => Err(()),
        }
    }
}

impl PartialEq<usize> for Size {
    fn eq(&self, other: &usize) -> bool {
        match self {
            Size::Empty => *other == 0,
            Size::One => *other == 1,
            Size::Two => *other == 2,
            Size::Four => *other == 4,
        }
    }
}

impl PartialEq<Size> for usize {
    fn eq(&self, other: &Size) -> bool {
        match self {
            0 => Size::Empty == *other,
            1 => Size::One == *other,
            2 => Size::Two == *other,
            4 => Size::Four == *other,
            _ => false,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct PrefixByte(u8);

impl PrefixByte {
    /// Convert the prefix byte to an u8 by consuming it
    pub fn u8(self) -> u8 {
        self.into()
    }

    /// Convert the prefix byte to an vector and append some payload
    /// after the prefix byte
    pub fn into_vec_append(self, mut payload: Vec<u8>) -> Vec<u8> {
        let mut raw = vec![self.u8()];
        raw.append(&mut payload);

        raw
    }
}

impl From<(&ItemType, &Size)> for PrefixByte {
    fn from(value: (&ItemType, &Size)) -> Self {
        let prefix_mask: u8 = match value.0 {
            // Main hid items
            // https://www.usb.org/sites/default/files/hid1_11.pdf - page 28
            ItemType::Main(MainType::Input) => 0b10000000,
            ItemType::Main(MainType::Output) => 0b10010000,
            ItemType::Main(MainType::Feature) => 0b10110000,
            ItemType::Main(MainType::Collection) => 0b10100000,
            ItemType::Main(MainType::EndCollection) => 0b11000000,

            // Global hid items
            // https://www.usb.org/sites/default/files/hid1_11.pdf - page 35
            ItemType::Global(GlobalType::UsagePage) => 0b00000100,
            ItemType::Global(GlobalType::LogicalMinimum) => 0b00010100,
            ItemType::Global(GlobalType::LogicalMaximum) => 0b00100100,
            ItemType::Global(GlobalType::PhysicalMinimum) => 0b00110100,
            ItemType::Global(GlobalType::PhysicalMaximum) => 0b01000100,
            ItemType::Global(GlobalType::UnitExponent) => 0b01010100,
            ItemType::Global(GlobalType::Unit) => 0b01100100,
            ItemType::Global(GlobalType::ReportSize) => 0b01110100,
            ItemType::Global(GlobalType::ReportID) => 0b10000100,
            ItemType::Global(GlobalType::ReportCount) => 0b10010100,
            ItemType::Global(GlobalType::Push) => 0b10100100,
            ItemType::Global(GlobalType::Pop) => 0b10110100,

            // Local hid items
            // https://www.usb.org/sites/default/files/hid1_11.pdf - page 39
            ItemType::Local(LocalType::Usage) => 0b00001000,
            ItemType::Local(LocalType::UsageMinimum) => 0b00011000,
            ItemType::Local(LocalType::UsageMaximum) => 0b00101000,
            ItemType::Local(LocalType::DesignatorIndex) => 0b00111000,
            ItemType::Local(LocalType::DesignatorMinimum) => 0b01001000,
            ItemType::Local(LocalType::DesignatorMaximum) => 0b01011000,
            ItemType::Local(LocalType::StringIndex) => 0b01111000,
            ItemType::Local(LocalType::StringMinimum) => 0b10001000,
            ItemType::Local(LocalType::StringMaximum) => 0b10011000,
            ItemType::Local(LocalType::Delimiter) => 0b10101000,
        };

        let size_mask: u8 = match value.1 {
            Size::Empty => 0,
            Size::One => 1,
            Size::Two => 2,
            Size::Four => 3, // Note: payload size = 0x03
        };

        PrefixByte(prefix_mask | size_mask)
    }
}

impl From<PrefixByte> for u8 {
    fn from(value: PrefixByte) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usage_page_size_1() {
        let result = PrefixByte::from((&ItemType::Global(GlobalType::UsagePage), &Size::One));
        assert_eq!(result, PrefixByte(0b00000101));
    }

    #[test]
    fn usage_page_size_2() {
        let result = PrefixByte::from((&ItemType::Global(GlobalType::UsagePage), &Size::Two));
        assert_eq!(result, PrefixByte(0b00000110));
    }
}
