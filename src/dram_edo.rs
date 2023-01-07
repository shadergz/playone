use crate::psx_comm;

pub enum PsMemIndex {
    MemMainMemory,
    MemExpansionRegion1,
    MemScratched,
    MemHardwareRegisters,
    MemBiosRom,
}

pub struct PsMemMap {
    pub kused: psx_comm::DoubleWord,
    pub kseg0: psx_comm::DoubleWord,
    pub kseg1: psx_comm::DoubleWord,
    pub region_length: psx_comm::DoubleWord,
}

pub const PS_MEM_REGIONS: [PsMemMap; 5] = [
    PsMemMap {
        kused: 0x00000000,
        kseg0: 0x80000000,
        kseg1: 0xa0000000,
        region_length: psx_comm::kibi_size!(2048),
    },
    PsMemMap {
        kused: 0x1f000000,
        kseg0: 0x9f000000,
        kseg1: 0xbf000000,
        region_length: psx_comm::kibi_size!(8192),
    },
    PsMemMap {
        kused: 0x1f800000,
        kseg0: 0x9f800000,
        kseg1: 0xbf800000,
        region_length: psx_comm::kibi_size!(1),
    },
    PsMemMap {
        kused: 0x1f801000,
        kseg0: 0x9f801000,
        kseg1: 0xbf801000,
        region_length: psx_comm::kibi_size!(8),
    },
    PsMemMap {
        kused: 0x1fc00000,
        kseg0: 0x9f801000,
        kseg1: 0xbfc00000,
        region_length: psx_comm::kibi_size!(512),
    },
];

#[derive(Clone)]
pub struct RamChip {
    _ram_content: Box<Vec<u8>>,
}

impl RamChip {
    pub fn new() -> Self {
        // PSX uses a 2MiB of RAM for operate
        let edo_dram_storage: Box<Vec<u8>> = Box::new(vec![0; psx_comm::kibi_size!(2048)]);
        Self {
            _ram_content: edo_dram_storage,
        }
    }

    pub fn read_fetch_uint32le(&mut self, _read_u32: u32) -> u32 {
        0
    }

    pub fn write_fast_uint32le(&mut self, _write_u32: u32, _value: u32) {}
}
