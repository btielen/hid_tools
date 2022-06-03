use bitstream_io::{BitRead, BitReader, LittleEndian};
use std::io::Cursor;

/// Parse the report_id from the first byte of HID Report
pub(super) fn report_id(input: &[u8]) -> Result<u8, ParseError> {
    let mut reader = create_bit_reader(input);

    let result: u8 = reader
        .read(8)
        .map_err(|_| ParseError::ReadingBitsFailed(8))?;

    Ok(result)
}

/// Parse a value from a Report at some bit position
pub(super) fn val(input: &[u8], position: u32, count: u32) -> Result<i64, ParseError> {
    let mut reader = create_bit_reader(input);

    reader
        .skip(position)
        .map_err(|_| ParseError::SkippingBitsFailed(position))?;

    let result: i64 = reader
        .read(count)
        .map_err(|_| ParseError::ReadingBitsFailed(count))?;

    Ok(result)
}

/// Create a BitReader
fn create_bit_reader(input: &[u8]) -> BitReader<Cursor<&[u8]>, LittleEndian> {
    let cursor = Cursor::new(input);
    BitReader::endian(cursor, LittleEndian)
}

#[derive(Debug, PartialEq)]
pub(super) enum ParseError {
    SkippingBitsFailed(u32),
    ReadingBitsFailed(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_report_id() {
        let report: Vec<u8> = vec![42, 0];
        assert_eq!(report_id(&report), Ok(42));
    }

    #[test]
    fn take_4_bits_alt() {
        let report: Vec<u8> = vec![0xf0];
        assert_eq!(val(&report, 0, 4), Ok(0));
    }

    #[test]
    fn take_12_bits_alt() {
        let report: Vec<u8> = vec![0xf0, 0x00];
        assert_eq!(val(&report, 0, 12), Ok(0x0f0));
    }
}
