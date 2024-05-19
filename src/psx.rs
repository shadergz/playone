pub type Word = u16;
pub type Double = u32;

macro_rules! kz2b {
    ($KiB:expr) => {
        1024 * $KiB
    };
}
pub(crate) use kz2b;
