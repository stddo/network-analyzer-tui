pub mod link;
pub mod ethernet2;

pub enum ReadError {
    IPUnexpectedVersion(u8),
    DataOffsetTooSmall(usize)
}