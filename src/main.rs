pub mod bus;
pub mod dram_edo;
pub mod mips_cortex;
pub mod psx_comm;

extern crate glium;

struct PsxConsole {
    central_unit: Box<mips_cortex::R3000A>,
    edo_chip: Box<dram_edo::RamChip>,
    bus_physical: Box<bus::Bus>,
}

impl PsxConsole {
    pub fn new() -> Self {
        let memory_shared = Box::new(dram_edo::RamChip::new());
        let bus_shared = Box::new(bus::Bus::new());
        let mips_shared = Box::new(mips_cortex::R3000A::new());

        Self {
            edo_chip: memory_shared,
            bus_physical: bus_shared,
            central_unit: mips_shared,
        }
    }

    pub fn power_on(&mut self) {
        self.bus_physical.startup_memory(&mut self.edo_chip);
        self.central_unit.setup_bus(&mut self.bus_physical);
    }

    pub fn hardware_reset(&mut self) {
        let console_cpu = &mut self.central_unit;
        console_cpu.cpu_power_reset();
    }
}

pub fn main() {
    let mut playone_console = PsxConsole::new();

    playone_console.power_on();
    // Performing a reset signal inside all playstation components/hardware(s)
    playone_console.hardware_reset();
}
