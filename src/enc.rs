use thiserror::Error;

pub fn encode_uvarint(value: u32) -> impl Iterator<Item = u8> {
    let mut buf = Vec::default();
    let mut remaining = value;

    buf.push((remaining as u8) & 0x7f);
    remaining >>= 7;
    while remaining > 0 {
        buf.push(0x80 | ((remaining as u8) & 0x7f));
        remaining >>= 7;
    }
    // Convert to big-endian
    buf.reverse();
    // Return
    buf.into_iter()
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum DecodeUvarintError {
    #[error("Value exceeds 2^32")]
    ValueTooLarge,
    #[error("Unexpected end of file")]
    UnexpectedEof,
}

/// Decode an unsigned variable length encoded integer.
pub fn decode_uvarint<'a>(
    bytes: &'a mut impl Iterator<Item = &'a u8>,
) -> Result<u32, DecodeUvarintError> {
    let mut value: u32 = 0;
    let mut cur: u8;
    // First read
    cur = *bytes.next().ok_or(DecodeUvarintError::UnexpectedEof)?;
    value |= (cur & 0x7f) as u32;
    if cur & 0x80 == 0 {
        return Ok(value);
    }
    // Second read
    value <<= 7;
    cur = *bytes.next().ok_or(DecodeUvarintError::UnexpectedEof)?;
    value |= (cur & 0x7f) as u32;
    if cur & 0x80 == 0 {
        return Ok(value);
    }
    // Third read
    value <<= 7;
    cur = *bytes.next().ok_or(DecodeUvarintError::UnexpectedEof)?;
    value |= (cur & 0x7f) as u32;
    if cur & 0x80 == 0 {
        return Ok(value);
    }
    // Fourth read
    value <<= 7;
    cur = *bytes.next().ok_or(DecodeUvarintError::UnexpectedEof)?;
    value |= (cur & 0x7f) as u32;
    if cur & 0x80 == 0 {
        return Ok(value);
    }
    // Fifth read
    let prev = value;
    value <<= 7;
    cur = *bytes.next().ok_or(DecodeUvarintError::UnexpectedEof)?;
    if value >> 7 != prev || cur & 0x80 != 0 {
        Err(DecodeUvarintError::ValueTooLarge)
    } else {
        value |= cur as u32;
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use std::u32;

    use super::*;

    #[test]
    pub fn test_encode_uvarint_0() {
        // Setup
        let expected: [u8; 1] = [0];
        // Test
        let result: Vec<u8> = encode_uvarint(0).collect();
        // Assert
        assert_eq!(&expected, result.as_slice());
    }

    #[test]
    pub fn test_encode_uvarint_1() {
        // Setup
        let expected: [u8; 1] = [0b00000001];
        // Test
        let result: Vec<u8> = encode_uvarint(1).collect();
        // Assert
        assert_eq!(&expected, result.as_slice());
    }

    #[test]
    pub fn test_encode_uvarint_127() {
        // Setup
        let expected: [u8; 1] = [0b01111111];
        // Test
        let result: Vec<u8> = encode_uvarint(127).collect();
        // Assert
        assert_eq!(&expected, result.as_slice());
    }

    #[test]
    pub fn test_encode_uvarint_128() {
        // Setup
        let expected: [u8; 2] = [0b10000001, 0b00000000];
        // Test
        let result: Vec<u8> = encode_uvarint(128).collect();
        // Assert
        assert_eq!(&expected, result.as_slice());
    }

    #[test]
    pub fn test_encode_uvarint_150() {
        // Setup
        let expected: [u8; 2] = [0b10000001, 0b00010110];
        // Test
        let result: Vec<u8> = encode_uvarint(150).collect();
        // Assert
        assert_eq!(&expected, result.as_slice());
    }

    #[test]
    pub fn test_encode_uvarint_max() {
        // Setup
        let expected: [u8; 5] = [0b10001111, 0b11111111, 0b11111111, 0b11111111, 0b01111111];
        // Test
        let result: Vec<u8> = encode_uvarint(u32::MAX).collect();
        // Assert
        assert_eq!(&expected, result.as_slice());
    }

    #[test]
    pub fn test_decode_uvarint_max() {
        // Setup
        let input: [u8; 5] = [0b10001111, 0b11111111, 0b11111111, 0b11111111, 0b01111111];
        let expected = u32::MAX;
        // Test
        let result = decode_uvarint(&mut input.iter()).unwrap();
        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    pub fn test_decode_uvarint_value_too_big() {
        // Setup
        let input: [u8; 5] = [0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b01111111];
        let expected = DecodeUvarintError::ValueTooLarge;
        // Test
        let result = decode_uvarint(&mut input.iter()).unwrap_err();
        // Assert
        assert_eq!(expected, result);
    }

    /// Take a valid uvarint payload and erroneously set the next_bit on the last byte to a 1
    /// This means more than 5 bytes would be read and there's no way we can fit more than 5
    /// bytes in a u32 value.
    #[test]
    pub fn test_decode_uvarint_value_too_big_next_bit_set() {
        // Setup
        let input: [u8; 5] = [0b10001111, 0b11111111, 0b11111111, 0b11111111, 0b11111111];
        let expected = DecodeUvarintError::ValueTooLarge;
        // Test
        let result = decode_uvarint(&mut input.iter()).unwrap_err();
        // Assert
        assert_eq!(expected, result);
    }
}
