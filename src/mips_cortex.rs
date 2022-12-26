use std::ops::{Add, Deref};

use crate::hard_common::REGDword;
use crate::dram_edo::{PS_MEM_REGIONS, PsMemMap};

pub struct CpuDeveloperPines {
    // We want to control every unique cpu cycle, and perform real time operations inside the CPU
    // for achieve this objective, we need to accumulate the "needed" cycles for doing a CPU
    // operation like execute a instruction that's will waste 6 cycles or access the memory
    // properly!
    #[allow(unused_variables)]
    cycle_accumulator: u64
}

impl CpuDeveloperPines {
    pub fn new() -> CpuDeveloperPines {
        CpuDeveloperPines {
            cycle_accumulator: 0
        }
    }
}

pub struct CpuMipsR3000A {
    rip: REGDword,
    #[allow(unused_variables)]
    debug_hardware: CpuDeveloperPines
}

#[allow(dead_code)]
pub struct CpuInst {
    #[allow(unused_variables)]
    inst_op : u8
}
impl CpuInst {
    #[allow(dead_code)]
    pub fn new() -> CpuInst {
        CpuInst { inst_op: 0 }
    }
}

impl CpuMipsR3000A {
    pub fn new() -> CpuMipsR3000A {
        CpuMipsR3000A {
            rip: 0,
            debug_hardware: CpuDeveloperPines::new()
        }
    }
    pub fn cpu_power_reset(&mut self) {
        let ps_mem_bios : &PsMemMap = PS_MEM_REGIONS.get(0).unwrap();
        self.rip = ps_mem_bios.deref().kseg1;
    }

    #[allow(dead_code)]
    pub fn perform_cycle(&mut self, cycles_count : u64) {
        let debug_cycle = &mut self.debug_hardware;
        // Otherwise cycle_accumulator overflow, Rust will advise for us!
        debug_cycle.cycle_accumulator = debug_cycle.cycle_accumulator.add(cycles_count);
        self.perform_release_ctrl();

    }
    #[inline(always)]
    fn perform_fetch(&mut self) -> Option<REGDword> {
        if self.debug_hardware.cycle_accumulator > 0 {
            return None
        }
        // Pointing to the next RIP location
        self.rip = self.rip.wrapping_add(4);
        Some(10)
    }
    #[inline(always)]
    fn perform_decode(&mut self, _collected_inst : REGDword ) -> Option<CpuInst> {
        None
    }
    #[inline(always)]
    fn perform_execute(&mut self, _inst: &CpuInst) {}

    pub fn perform_release_ctrl(&mut self) {
        let fetched_rip = self.perform_fetch().unwrap();
        let fetched_inst = self.perform_decode(fetched_rip).unwrap();
        self.perform_execute(&fetched_inst);

    }

}

