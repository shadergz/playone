pub type GeneralWord = u16;
pub type DoubleWord = u32;

macro_rules! kibi_size {
    ($KiB:expr) => {
        1024 * $KiB
    };
}

pub(crate) use kibi_size;
