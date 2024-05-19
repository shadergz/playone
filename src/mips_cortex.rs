use crate::bus;
use crate::bus::AccessMode;
use std::collections::HashMap;
use std::mem::size_of;
use std::ops::Add;

use crate::dram_edo;
use crate::psx;

// 4 KB instruction cache
const CACHE_SIZE: usize = psx::kz2b!(4);

// Risc MIPS addressing cache system map
// 32 bytes per cache line
const CACHE_LINE_SIZE: usize = 32;
// Count of lines inside the cache, total = (1024) cache lines
const CACHE_LINE_BLK_SZ: usize = CACHE_SIZE / (CACHE_LINE_SIZE / 8);

#[derive(Copy, Clone)]
struct CacheLine {
    tag: u32,
    data: u32,
}

#[derive(Default)]
pub struct R3000A {
    rip: psx::Double,
    inst_cache: Vec<CacheLine>,
    psx_inst: HashMap<u32, String>,
    installed_bus: Option<Box<bus::Bus>>,
    cache_miss: usize,
    cache_hit: usize,
    cycle_acc: u64,
}

#[repr(C)]
#[derive(Default)]
pub struct CpuInst {
    opcode: psx::Double,
}

fn nop() {}

impl R3000A {
    pub fn setup_bus(&mut self, bus: &mut Box<bus::Bus>) {
        self.installed_bus = Some(bus.clone());

        self.psx_inst.insert(0, "nop".to_string());
    }
    pub fn cpu_reset(&mut self) {
        let ps_mem: &dram_edo::PsMemMap = dram_edo::PS_MEM_REGIONS.get(4).unwrap();
        self.rip = ps_mem.k_seg1;

        if self.inst_cache.capacity() != CACHE_LINE_BLK_SZ {
            self.inst_cache.reserve(CACHE_LINE_BLK_SZ);
        }
        assert_eq!(self.inst_cache.capacity(), 1024);

        self.inst_cache.clear();
    }

    pub fn perform_cycle(&mut self, cycles_count: u64) {
        // Before `cycle_acc` overflow, Rust will advise for us!
        self.cycle_acc = self.cycle_acc.add(cycles_count);

        self.perform_pipeline();
    }

    #[inline(always)]
    fn perform_fetch(&mut self) -> Option<psx::Double> {
        if self.cycle_acc <= 0 {
            return None;
        }
        let ip_fetched = self.read_memory(self.rip);
        // Pointing to the next RIP location
        self.rip = self.rip.wrapping_add(size_of::<psx::Word>() as u32);
        Some(ip_fetched)
    }

    fn read_memory(&mut self, memory_address: psx::Double) -> psx::Double {
        let internal_bus = self.installed_bus.as_mut().unwrap();

        let tag = (memory_address & 0xffff_f000) >> 12 | 1;
        let index = ((memory_address & 0xffc) >> 2) as usize;
        let cached = self.inst_cache.get_mut(index);

        if cached.is_some() {
            if self.inst_cache[index].tag == tag {
                self.cache_hit += 1;
                return self.inst_cache[index].data;
            }
        }
        internal_bus.register_access(memory_address, 0, AccessMode::ReadDataFrom);
        if memory_address > 0xa000_0000 {
            return internal_bus.commit_access().unwrap();
        }
        self.cache_miss += 1;

        let result = internal_bus.commit_access().unwrap();
        self.inst_cache[index].tag = tag;
        let cache = CacheLine { tag, data: result };
        self.inst_cache[index] = cache;

        return result;
    }

    #[inline(always)]
    fn perform_execute(&mut self, inst: &CpuInst) {
        self.cycle_acc -= 4;

        match (inst.opcode >> 26) & 0xff {
            0 => {
                nop();
            }
            _ => {}
        }
    }

    pub fn perform_pipeline(&mut self) {
        let fetched_rip = self.perform_fetch().unwrap();
        let fetched_inst = CpuInst {
            opcode: fetched_rip,
        };

        self.perform_execute(&fetched_inst);
        self.rip += 4;
    }
}
