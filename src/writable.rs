use crate::writer::{Writer, WriterResult};

pub trait Writable {
    fn write(&self, writer: &mut impl Writer) -> WriterResult;
}
