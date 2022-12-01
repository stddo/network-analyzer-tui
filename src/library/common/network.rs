pub mod link;
pub mod ethernet2;
pub mod packet;

pub enum ReadError {
    IPUnexpectedVersion(u8),
    DataOffsetTooSmall(usize)
}