use crate::{
    readable::Readable,
    reader::ReaderResult,
    writable::Writable,
    writer::{WriterError, WriterResult},
};

mod v0 {
    use crate::{
        readable::Readable, reader::ReaderResult, writable::Writable, writer::WriterResult,
    };

    /*
     * RequestHeader
     */

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct RequestHeader {
        pub request_api_key: i16,
        pub request_api_version: i16,
        pub correlation_id: i32,
    }

    impl Readable for RequestHeader {
        fn read(reader: &mut impl crate::reader::Reader) -> ReaderResult<Self> {
            Ok(Self {
                request_api_key: reader.next_i16()?,
                request_api_version: reader.next_i16()?,
                correlation_id: reader.next_i32()?,
            })
        }
    }

    impl Writable for RequestHeader {
        fn write(&self, writer: &mut impl crate::writer::Writer) -> WriterResult {
            writer.write_i16(self.request_api_key)?;
            writer.write_i16(self.request_api_version)?;
            writer.write_i32(self.correlation_id)?;
            Ok(())
        }
    }

    /*
     * ResponseHeader
     */
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct ResponseHeader {
        pub correlation_id: i32,
    }

    impl Readable for ResponseHeader {
        fn read(reader: &mut impl crate::reader::Reader) -> ReaderResult<Self> {
            Ok(Self {
                correlation_id: reader.next_i32()?,
            })
        }
    }

    impl Writable for ResponseHeader {
        fn write(&self, writer: &mut impl crate::writer::Writer) -> WriterResult {
            writer.write_i32(self.correlation_id)?;
            Ok(())
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TaggedField {
    pub tag: u32,
    pub field: Vec<u8>,
}

impl Readable for TaggedField {
    fn read(reader: &mut impl crate::reader::Reader) -> ReaderResult<Self>
    where
        Self: Sized,
    {
        let tag = reader.next_unsigned_varint()?;
        let field_len = reader.next_unsigned_varint()?;
        let field = reader.next_raw_bytes(field_len)?;
        Ok(Self { tag, field })
    }
}

impl Writable for TaggedField {
    fn write(&self, writer: &mut impl crate::writer::Writer) -> WriterResult {
        writer.write_unsigned_varint(self.tag)?;
        writer.write_unsigned_varint(
            self.field
                .len()
                .try_into()
                .map_err(|_err| WriterError::ValueOutOfRange)?,
        )?;
        Ok(())
    }
}

mod v1 {
    use crate::{
        readable::Readable, reader::ReaderResult, writable::Writable, writer::WriterResult,
    };

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct RequestHeader {
        pub request_api_key: i16,
        pub request_api_version: i16,
        pub correlation_id: i32,
        pub client_id: Option<String>,
    }

    impl Readable for RequestHeader {
        fn read(reader: &mut impl crate::reader::Reader) -> ReaderResult<Self> {
            Ok(Self {
                request_api_key: reader.next_i16()?,
                request_api_version: reader.next_i16()?,
                correlation_id: reader.next_i32()?,
                client_id: reader.next_nullable_string()?,
            })
        }
    }

    impl Writable for RequestHeader {
        fn write(&self, writer: &mut impl crate::writer::Writer) -> WriterResult {
            writer.write_i16(self.request_api_key)?;
            writer.write_i16(self.request_api_version)?;
            writer.write_i32(self.correlation_id)?;
            writer.write_nullable_string(self.client_id.as_deref())?;
            Ok(())
        }
    }

    /*
     * ResponeHeader
     */
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct ResponseHeader {
        pub correlation_id: i32,
        pub tagged_fields: Vec<u8>,
    }
}

mod v2 {
    use super::TaggedField;

    /*
     * RequestHeader
     */
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct RequestHeader {
        pub request_api_key: i16,
        pub request_api_version: i16,
        pub correlation_id: i32,
        pub client_id: Option<String>,
        pub tagged_fields: Vec<TaggedField>,
    }

    /*
     * ResponseHeader
     */
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct ResponseHeader {

    }

}
