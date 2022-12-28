use std::mem;
use std::ops::{Add, Deref};

use crate::dram_edo::{PsMemMap, PS_MEM_REGIONS};
use crate::hard_common::{kibi2byte, REGDword, REGWord};

#[derive(Default)]
pub struct CpuDeveloperPines {
    // We want to control every unique cpu cycle, and perform real time operations inside the CPU
    // for achieve this objective, we need to accumulate the "needed" cycles for doing a CPU
    // operation like execute a instruction that's will waste 6 cycles or access the memory
    // properly!
    cycle_accumulator: u64,
}

const CPU_CACHE_SIZE: u32 = kibi2byte!(5); // 5KiB of cache size
const CPU_L1_INST_LIMITS: u32 = kibi2byte!(4); // maximum cache instruction region

// Risc MIPS addressing cache system map
// 31-13 tags, 12-5 index, 4-0 offsets
const CPU_CACHE_LINE_SIZE: u8 = 32; // 32 bytes per cache line
                                    // Count of lines inside the cache; total = (160) cache lines
const CPU_CACHE_LINE_COUNT: u32 = CPU_CACHE_SIZE / CPU_CACHE_LINE_SIZE as u32;

// 4 bits is needed for identifier the data offset
const CPU_CACHE_WAY_BLOCK_SIZE: u32 = CPU_CACHE_LINE_SIZE as u32 / mem::size_of::<REGWord>() as u32;

#[repr(C)]
#[derive(Default, Clone)]
struct CpuCacheWay {
    blocks: [REGWord; CPU_CACHE_WAY_BLOCK_SIZE as usize],
}
impl Copy for CpuCacheWay {}

// Number of sets: 5120 = X * 2 * 32 [R: 80]
// 7 bits is needed for identifier the index
// Count of ways by each cache set; total of sets = 320/
const CPU_CACHE_WAYS: u8 = 2;

#[repr(C)]
#[derive(Default)]
struct CpuCacheSets {
    ways: [CpuCacheWay; CPU_CACHE_WAYS as usize],
}

const CPU_CACHE_SETS_COUNT: u16 = u16::pow(2, 9);

pub struct CpuMipsR3000A {
    rip: REGDword,
    debug_hardware: CpuDeveloperPines,
    l1_cache: [u8; CPU_CACHE_SIZE as usize],
    cache_miss: usize,
    cache_hit: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct CpuInst {
    inst_op: u8,
}

impl CpuMipsR3000A {
    pub fn new() -> Self {
        Self {
            rip: 0,
            debug_hardware: Default::default(),
            l1_cache: [0; CPU_CACHE_SIZE as usize],
            cache_miss: 0,
            cache_hit: 0,
        }
    }

    pub fn cpu_power_reset(&mut self) {
        let ps_mem_bios: &PsMemMap = PS_MEM_REGIONS.get(0).unwrap();
        self.rip = ps_mem_bios.deref().kseg1;
    }

    pub fn perform_cycle(&mut self, cycles_count: u64) {
        let debug_cycle = &mut self.debug_hardware;
        // Otherwise cycle_accumulator overflow, Rust will advise for us!
        debug_cycle.cycle_accumulator = debug_cycle.cycle_accumulator.add(cycles_count);
        self.perform_pipeline();
    }

    #[inline(always)]
    fn perform_fetch(&mut self) -> Option<REGDword> {
        if self.debug_hardware.cycle_accumulator > 0 {
            return None;
        }
        let ip_fetched = self
            .read_memory(self.rip, mem::size_of::<REGDword>())
            .unwrap();
        // Pointing to the next RIP location
        self.rip = self.rip.wrapping_add(mem::size_of::<REGDword>() as u32);
        Some(ip_fetched)
    }
    fn read_memory(&mut self, _memory_address: REGDword, read_size: usize) -> Option<REGDword> {
        return match read_size {
            _ => None,
        };
    }

    #[inline(always)]
    fn perform_decode(&mut self, _collected_inst: REGDword) -> Option<CpuInst> {
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
