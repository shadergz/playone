use crate::psx_comm;

pub enum PsMemIndex {
    MainMemory,
    ExpansionRegion1,
    ScratchPad,
    HardwareRegisters,
    BiosRom,
}

pub struct PsMemMap {
    pub k_used: psx_comm::DoubleWord,
    pub k_seg0: psx_comm::DoubleWord,
    pub k_seg1: psx_comm::DoubleWord,
    pub size: psx_comm::DoubleWord,
}

pub const PS_MEM_REGIONS: [PsMemMap; 5] = [
    PsMemMap {
        k_used: 0x00000000, k_seg0: 0x80000000,
        k_seg1: 0xa0000000, size: psx_comm::kibisz!(2048),
    },
    PsMemMap {
        k_used: 0x1f000000, k_seg0: 0x9f000000,
        k_seg1: 0xbf000000, size: psx_comm::kibisz!(8192),
    },
    PsMemMap {
        k_used: 0x1f800000, k_seg0: 0x9f800000,
        k_seg1: 0xbf800000, size: psx_comm::kibisz!(1),
    },
    PsMemMap {
        k_used: 0x1f801000, k_seg0: 0x9f801000,
        k_seg1: 0xbf801000, size: psx_comm::kibisz!(8),
    },
    PsMemMap {
        k_used: 0x1fc00000, k_seg0: 0x9f801000, k_seg1: 0xbfc00000,
        size: psx_comm::kibisz!(512),
    },
];

#[derive(Clone)]
pub struct RamChip {
    _ram_memory: Box<Vec<u8>>,
}

impl RamChip {
    pub fn new() -> Self {
        // PSX uses 2 MiB of RAM to operate
        let edo_dram: Box<Vec<u8>> = Box::new(vec![0; psx_comm::kibisz!(2048)]);
        Self {
            _ram_memory: edo_dram,
        }
    }
    pub fn read_u32(&mut self, _read: u32) -> u32 { 0 }
    pub fn write_u32(&mut self, _write: u32, _value: u32) { }
}
