mod dram_edo;
mod hard_common;
mod mips_cortex;

use crate::mips_cortex::CpuMipsR3000A;

extern crate glium;

struct PsxConsole {
    central_unit: CpuMipsR3000A,
}

impl PsxConsole {
    pub fn new() -> PsxConsole {
        PsxConsole {
            central_unit: CpuMipsR3000A::new(),
        }
    }

    fn power_on(&mut self) {
        // Performing a reset signal inside all playstation components/hardware(s)
        PsxConsole::console_hardware_reset(self);
    }
    fn console_hardware_reset(&mut self) {
        let console_cpu = &mut self.central_unit;
        console_cpu.cpu_power_reset();
    }
}

pub fn main() {
    let mut playone_console = PsxConsole::new();
    playone_console.power_on();
}
