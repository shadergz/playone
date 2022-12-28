pub type REGDword = u32;
pub type REGWord = u16;

macro_rules! kibi2byte {
    ($KiB:expr) => {
        1024 * $KiB
    };
}

pub(crate) use kibi2byte;
