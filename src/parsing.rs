use core::str;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserNextError {
    #[error("Unexpected end of input.")]
    UnexpectedEndOfInput,
}

#[derive(Debug, Error)]
pub enum ParserNextStringError {
    #[error(transparent)]
    UnexpectedEndOfInput(#[from] ParserNextError),
    #[error("Negative length.")]
    NegativeLength,
}

pub struct Parser<'a> {
    buf: &'a [u8],
}

pub trait KafkaPrimitive<T> {
    fn from_be_bytes(bytes: &[u8]) -> Option<T>;
}

macro_rules! prim {
    ($t:ty) => {
        impl KafkaPrimitive<$t> for $t {
            fn from_be_bytes(bytes: &[u8]) -> Option<$t> {
                Some(<$t>::from_be_bytes(bytes.try_into().ok()?))
            }
        }
    };
}

prim!(i8); // INT8
prim!(i16); // INT16
prim!(u16); // UINT16
prim!(i32); // INT32
prim!(u32); // UINT32
prim!(i64); // INT64
prim!(u64); // UINT64
prim!(f64); // FLOAT64

type NextResult<T> = Result<T, ParserNextError>;

impl<'a> Parser<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf }
    }

    pub fn next_bytes(&mut self, count: usize) -> NextResult<&'a [u8]> {
        if count > self.buf.len() {
            Err(ParserNextError::UnexpectedEndOfInput)
        } else {
            let (left, right) = self.buf.split_at(count);
            self.buf = right;
            Ok(left)
        }
    }

    pub fn next_bool(&mut self) -> NextResult<bool> {
        let value;
        unsafe {
            value = *self.next_bytes(1)?.as_ptr();
        }
        if value == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    pub fn next_primitive<T: KafkaPrimitive<T> + Sized>(&mut self) -> NextResult<T> {
        let bytes = self.next_bytes(std::mem::size_of::<T>())?;
        T::from_be_bytes(bytes).ok_or(ParserNextError::UnexpectedEndOfInput)
    }

    pub fn next_string(&mut self) -> Result<String, ParserNextStringError> {
        let len = self.next_primitive::<i16>()?;
        let real_len =
            usize::try_from(len).map_err(|_err| ParserNextStringError::NegativeLength)?;
        let bytes = self.next_bytes(real_len)?;
        unsafe {
            let value = str::from_utf8_unchecked(bytes);
            Ok(value.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify parsing an i8 from an empty buffer results in an error.
    #[test]
    fn test_parse_i8_empty() {
        // Setup
        let bytes: [u8; 0] = [];
        let mut parser = Parser::new(&bytes);
        // Test
        assert!(matches!(
            parser.next_primitive::<i8>(),
            Err(ParserNextError::UnexpectedEndOfInput)
        ))
    }

    /// Verify parsing an i8 from a single byte buffer.
    #[test]
    fn test_parse_i8_single_byte() {
        // Setup
        let bytes: [u8; 1] = [42];
        let mut parser = Parser::new(&bytes);
        // Test
        assert!(matches!(parser.next_primitive::<i8>(), Ok(42)));
        assert!(matches!(
            parser.next_primitive::<i8>(),
            Err(ParserNextError::UnexpectedEndOfInput)
        ))
    }

    /// Verify parsing an i8 from a mult-byte buffer ignores the trailing bytes.
    #[test]
    fn test_parse_i8_multiple_byte() {
        // Setup
        let bytes: [u8; 2] = [42, 93];
        let mut parser = Parser::new(&bytes);
        // Test
        assert!(matches!(parser.next_primitive::<i8>(), Ok(42)));
        assert!(matches!(parser.next_primitive::<i8>(), Ok(93)));
        assert!(matches!(
            parser.next_primitive::<i8>(),
            Err(ParserNextError::UnexpectedEndOfInput)
        ))
    }

    /// Verify that an i16 is read from a 2 byte buffer.
    #[test]
    fn test_parse_i16_2bytes() {
        // Setup
        let bytes = b"\x23\x29";
        let mut parser = Parser::new(bytes);
        // Test
        assert!(matches!(parser.next_primitive::<i16>(), Ok(9001)));
        assert!(matches!(
            parser.next_primitive::<i8>(),
            Err(ParserNextError::UnexpectedEndOfInput)
        ))
    }

    /// Verify that an f64 is read from an 8 byte buffer.
    #[test]
    fn test_parse_f64_8bytes() {
        // Setup
        let bytes = b"\x40\x09\x1e\xb8\x51\xeb\x85\x1f";
        let mut parser = Parser::new(bytes);
        // Test
        assert!(matches!(parser.next_primitive::<f64>(), Ok(x) if x == 3.14));
        assert!(matches!(
            parser.next_primitive::<f64>(),
            Err(ParserNextError::UnexpectedEndOfInput)
        ))
    }

    /// Verify that an inaccurate f64 is read from an 8 byte buffer.
    #[test]
    fn test_parse_f64_inaccurate_8bytes() {
        // Setup
        let bytes = b"\x40\x09\x1e\xb8\xd8\x23\x42\x25";
        let mut parser = Parser::new(bytes);
        // Test
        assert!(matches!(parser.next_primitive::<f64>(), Ok(x) if x == 3.1400010000000003));
        assert!(matches!(
            parser.next_primitive::<f64>(),
            Err(ParserNextError::UnexpectedEndOfInput)
        ))
    }
}
