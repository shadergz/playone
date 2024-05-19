pub mod bus;
pub mod dram_edo;
pub mod mips_cortex;
pub mod psx;

extern crate glium;
struct PsxConsole {
    cpu: Box<mips_cortex::R3000A>,
    edo: Box<dram_edo::RamChip>,
    bus_physical: Box<bus::Bus>,
}

impl PsxConsole {
    pub fn new() -> Self {
        let memory_shared = Box::new(dram_edo::RamChip::new());
        let bus_shared = Box::new(bus::Bus::new());
        let mips_shared = Box::new(mips_cortex::R3000A::default());

        Self {
            edo: memory_shared,
            bus_physical: bus_shared,
            cpu: mips_shared,
        }
    }

    pub fn start_system(&mut self) {
        self.bus_physical.startup_memory(&mut self.edo);
        self.cpu.setup_bus(&mut self.bus_physical);
    }
    pub fn reset_system(&mut self) {
        let console_cpu = &mut self.cpu;
        console_cpu.cpu_reset();
    }

    pub fn run(&mut self) {
        let mut steps = 10;
        while steps != 0 {
            self.cpu.as_mut().perform_cycle(4);
            steps -= 1;
        }
    }
}

pub fn main() {
    #![allow(unused_parens)]
    let mut console = PsxConsole::new();

    console.start_system();
    // Performing a reset signal across all PlayOne components/hardware
    console.reset_system();

    console.run();
}
