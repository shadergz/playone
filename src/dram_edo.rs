use crate::hard_common::REGDword;

#[allow(dead_code)]
pub struct JohnVonBus {}

#[allow(dead_code)]
pub enum PsMemIndex {
    MemMainMemory,
    MemExpansionRegion1,
    MemScratched,
    MemHardwareRegisters,
    MemBiosRom,
}

pub struct PsMemMap {
    pub kused: REGDword,
    pub kseg0: REGDword,
    pub kseg1: REGDword,
    pub region_length: REGDword,
}

macro_rules! kibi_to_byte {
    ($KiB:expr) => {
        1024 * $KiB
    };
}

pub const PS_MEM_REGIONS: [PsMemMap; 5] = [
    PsMemMap {
        kused: 0x00000000,
        kseg0: 0x80000000,
        kseg1: 0xa0000000,
        region_length: kibi_to_byte!(2048),
    },
    PsMemMap {
        kused: 0x1f000000,
        kseg0: 0x9f000000,
        kseg1: 0xbf000000,
        region_length: kibi_to_byte!(8192),
    },
    PsMemMap {
        kused: 0x1f800000,
        kseg0: 0x9f800000,
        kseg1: 0xbf800000,
        region_length: kibi_to_byte!(1),
    },
    PsMemMap {
        kused: 0x1f801000,
        kseg0: 0x9f801000,
        kseg1: 0xbf801000,
        region_length: kibi_to_byte!(8),
    },
    PsMemMap {
        kused: 0x1fc00000,
        kseg0: 0x9f801000,
        kseg1: 0xbfc00000,
        region_length: kibi_to_byte!(512),
    },
];
