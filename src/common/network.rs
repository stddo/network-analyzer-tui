pub mod link;
pub mod ethernet2;

pub enum ReadError {
    IPUnexpectedVersion(u8),
    DataOffsetTooSmall(usize)
}

trait SufficientOffset {
    const SIZE: usize;

    fn assert_offset_size(bytes_len: usize) -> Result<(), ReadError> {
        return if bytes_len < Self::SIZE {
            Err(ReadError::DataOffsetTooSmall(Self::SIZE - bytes_len))
        } else {
            Ok(())
        }
    }
}