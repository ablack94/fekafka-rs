use crate::reader::{Reader, ReaderResult};

pub trait Readable {
    fn read(reader: &mut impl Reader) -> ReaderResult<Self>
    where
        Self: Sized;
}
