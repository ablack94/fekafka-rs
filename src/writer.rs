use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum WriterError {
    #[error("Value out of range!")]
    ValueOutOfRange,
    #[error("Buffer overflow!")]
    BufferOverflow,
}

pub type WriterResult = Result<(), WriterError>;

pub trait Writer {
    fn write_raw_bytes(&mut self, value: &[u8]) -> WriterResult;

    fn write_bool(&mut self, value: bool) -> WriterResult;

    fn write_i8(&mut self, value: i8) -> WriterResult;

    fn write_i16(&mut self, value: i16) -> WriterResult;

    fn write_i32(&mut self, value: i32) -> WriterResult;

    fn write_i64(&mut self, value: i64) -> WriterResult;

    fn write_u32(&mut self, value: u32) -> WriterResult;

    fn write_varint(&mut self, value: i32) -> WriterResult;

    fn write_unsigned_varint(&mut self, value: u32) -> WriterResult;

    fn write_varlong(&mut self, value: i64) -> WriterResult;

    fn write_unsigned_varlong(&mut self, value: u64) -> WriterResult;

    fn write_uuid(&mut self, uuid: &str) -> WriterResult;

    fn write_f64(&mut self, value: f64) -> WriterResult;

    fn write_string(&mut self, value: &str) -> WriterResult;

    fn write_compact_string(&mut self, value: &str) -> WriterResult;

    fn write_nullable_string(&mut self, value: Option<&str>) -> WriterResult;

    fn write_compact_nullable_string(&mut self, value: Option<&str>) -> WriterResult;

    fn write_bytes(&mut self, value: &[u8]) -> WriterResult;

    fn write_compact_bytes(&mut self, value: &[u8]) -> WriterResult;

    fn write_nullable_bytes(&mut self, value: Option<&[u8]>) -> WriterResult;

    fn write_compact_nullable_bytes(&mut self, value: Option<&[u8]>) -> WriterResult;
}
