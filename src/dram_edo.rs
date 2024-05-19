use crate::psx;
use std::io::Write;

pub enum PsMemIndex {
    MainMemory,
    ExpansionRegion1,
    ScratchPad,
    HardwareRegisters,
    BiosRom,
}

pub struct PsMemMap {
    pub k_used: psx::Double,
    pub k_seg0: psx::Double,
    pub k_seg1: psx::Double,
    pub size: psx::Double,
}

pub const PS_MEM_REGIONS: [PsMemMap; 5] = [
    PsMemMap {
        k_used: 0x00000000,
        k_seg0: 0x80000000,
        k_seg1: 0xa0000000,
        size: psx::kz2b!(2048),
    },
    PsMemMap {
        k_used: 0x1f000000,
        k_seg0: 0x9f000000,
        k_seg1: 0xbf000000,
        size: psx::kz2b!(8192),
    },
    PsMemMap {
        k_used: 0x1f800000,
        k_seg0: 0x9f800000,
        k_seg1: 0xbf800000,
        size: psx::kz2b!(1),
    },
    PsMemMap {
        k_used: 0x1f801000,
        k_seg0: 0x9f801000,
        k_seg1: 0xbf801000,
        size: psx::kz2b!(8),
    },
    PsMemMap {
        k_used: 0x1fc00000,
        k_seg0: 0x9f801000,
        k_seg1: 0xbfc00000,
        size: psx::kz2b!(512),
    },
];
#[derive(Clone)]
pub struct RamChip {
    ram_memory: Box<Vec<u8>>,
}

impl RamChip {
    pub fn new() -> Self {
        // PSX uses 2 MiB of RAM to operate
        let dram: Box<Vec<u8>> = Box::new(vec![0; psx::kz2b!(2048)]);
        Self { ram_memory: dram }
    }
    pub fn read_u32(&mut self, read: u32) -> u32 {
        return self.ram_memory[read as usize & self.ram_memory.capacity()] as u32;
    }
    pub fn write_u32(&mut self, write: u32, value: u32) {
        let region = self.ram_memory.by_ref();
        let valid_area = region.capacity();

        region[(write as usize & valid_area) + 0] = (value & 0xff) as u8;
        region[(write as usize & valid_area) + 1] = (value >> 8 & 0xff) as u8;
        region[(write as usize & valid_area) + 2] = (value >> 16 & 0xff) as u8;
        region[(write as usize & valid_area) + 3] = (value >> 24 & 0xff) as u8;
    }
}
