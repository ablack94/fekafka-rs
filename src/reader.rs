use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ReaderError {
    #[error("Unexpected end of input!")]
    UnexpectedEndOfInput,
    #[error("Negative length specified!")]
    NegativeLength,
    #[error("Invalid null value!")]
    InvalidNull,
}

pub type ReaderResult<T> = Result<T, ReaderError>;

pub trait Reader {
    fn next_raw_bytes(&mut self, count: u32) -> ReaderResult<Vec<u8>>;

    fn next_bool(&mut self) -> ReaderResult<bool>;

    fn next_i8(&mut self) -> ReaderResult<i8>;

    fn next_i16(&mut self) -> ReaderResult<i16>;

    fn next_i32(&mut self) -> ReaderResult<i32>;

    fn next_i64(&mut self) -> ReaderResult<i64>;

    fn next_u32(&mut self) -> ReaderResult<u32>;

    fn next_varint(&mut self) -> ReaderResult<i32>;

    fn next_unsigned_varint(&mut self) -> ReaderResult<u32>;

    fn next_varlong(&mut self) -> ReaderResult<i64>;

    fn next_unsigned_varlong(&mut self) -> ReaderResult<u64>;

    fn next_uuid(&mut self) -> ReaderResult<String>;

    fn next_f64(&mut self) -> ReaderResult<f64>;

    fn next_string(&mut self) -> ReaderResult<String>;

    fn next_compact_string(&mut self) -> ReaderResult<String>;

    fn next_nullable_string(&mut self) -> ReaderResult<Option<String>>;

    fn next_compact_nullable_string(&mut self) -> ReaderResult<Option<String>>;

    fn next_bytes(&mut self) -> ReaderResult<Vec<u8>>;

    fn next_compact_bytes(&mut self) -> ReaderResult<Vec<u8>>;

    fn next_nullable_bytes(&mut self) -> ReaderResult<Vec<u8>>;

    fn next_compact_nullable_bytes(&mut self) -> ReaderResult<Option<Vec<u8>>>;
}
