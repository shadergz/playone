use std::mem::size_of;
use std::ops::{Add, Rem};

use crate::bus;
use crate::bus::AccessMode;

use crate::dram_edo;
use crate::psx_comm;

const CACHE_SIZE: u32 = psx_comm::kibisz!(5); // 5 KiB of cache size
const _L1_INST_LIMITS: u32 = psx_comm::kibisz!(4); // Maximum cache instruction region

// Risc MIPS addressing cache system map
// 31-13 tags, 12-5 index, 4-0 offsets
const CACHE_LINE_SIZE: u8 = 32; // 32 bytes per cache line
// Count of lines inside the cache; total = (160) cache lines
const _CACHE_LINE_COUNT: u32 = CACHE_SIZE / CACHE_LINE_SIZE as u32;

// 4 bits is needed for identifier the data offset
const CACHE_WAY_BLOCK_SIZE: u32 = CACHE_LINE_SIZE as u32 / size_of::<psx_comm::DoubleWord>() as u32;

#[repr(C)]
#[derive(Default, Clone)]
struct CpuCacheWay {
    blocks: [psx_comm::DoubleWord; CACHE_WAY_BLOCK_SIZE as usize],
}
impl Copy for CpuCacheWay {}

// Number of sets: 5120 = X * 2 * 32 [R: 80]
// 7 bits are needed to identify the index count of ways for each cache set; total sets = 320
const CACHE_WAYS: u8 = 2;

#[repr(C)]
#[derive(Default)]
struct CpuCacheSets {
    ways: [CpuCacheWay; CACHE_WAYS as usize],
}

const _CACHE_SETS_COUNT: u16 = u16::pow(2, 9);
pub struct R3000A {
    rip: psx_comm::DoubleWord,
    _l1_cache: [u8; CACHE_SIZE as usize],
    installed_bus: Option<Box<bus::Bus>>,
    _cache_miss: usize,
    _cache_hit: usize,
    cycle_accumulator: u64,
}

#[repr(C)]
#[derive(Default)]
pub struct CpuInst {
    inst_op: u8,
}

impl R3000A {
    pub fn new() -> Self {
        Self {
            rip: 0,
            _l1_cache: [0; CACHE_SIZE as usize],

            installed_bus: None,
            _cache_miss: 0,
            _cache_hit: 0,
            cycle_accumulator: 0,
        }
    }

    pub fn setup_bus(&mut self, bus: &mut Box<bus::Bus>) {
        self.installed_bus = Some(bus.clone())
    }
    pub fn cpu_reset(&mut self) {
        let ps_mem: &dram_edo::PsMemMap = dram_edo::PS_MEM_REGIONS.get(0).unwrap();
        self.rip = ps_mem.k_seg1;
    }

    pub fn perform_cycle(&mut self, cycles_count: u64) {
        // Before "cycle_accumulator" overflow, Rust will advise for us!
        self.cycle_accumulator = self.cycle_accumulator.add(cycles_count);
        self.perform_pipeline();
    }

    #[inline(always)]
    fn perform_fetch(&mut self) -> Option<psx_comm::DoubleWord> {
        if self.cycle_accumulator > 0 {
            return None;
        }
        let ip_fetched = self
            .read_memory(self.rip, size_of::<psx_comm::DoubleWord>())
            .unwrap();
        // Pointing to the next RIP location
        self.rip = self.rip.wrapping_add(size_of::<psx_comm::GeneralWord>() as u32);
        Some(ip_fetched)
    }

    pub fn read_line(&mut self, mut read_at: u32) -> Vec<u32> {
        assert_eq!(read_at.rem(CACHE_LINE_SIZE as u32), 0, "Un-alignment address reading isn't acceptable");

        let mut accumulator: Vec<u32> = vec![CACHE_LINE_SIZE as u32; 0];
        let internal_bus = self.installed_bus.as_mut().unwrap();

        for v_position in 0..3 {
            read_at = read_at.wrapping_add(size_of::<u32>() as u32);
            internal_bus.register_access(read_at, 0, AccessMode::ReadDataFrom);
            accumulator[v_position] = internal_bus.commit_access().unwrap();
        }

        accumulator
    }

    fn read_memory(
        &mut self,
        _memory_address: psx_comm::DoubleWord,
        read_size: usize,
    ) -> Option<psx_comm::DoubleWord> {
        return match read_size {
            _ => None,
        };
    }

    #[inline(always)]
    fn perform_decode(&mut self, _collected_inst: psx_comm::DoubleWord) -> Option<CpuInst> {
        None
    }
    #[inline(always)]
    fn perform_execute(&mut self, _inst: &CpuInst) {}

    pub fn perform_pipeline(&mut self) {
        let fetched_rip = self.perform_fetch().unwrap();
        let fetched_inst = self.perform_decode(fetched_rip).unwrap();
        self.perform_execute(&fetched_inst);
    }
}
