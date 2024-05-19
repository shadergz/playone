use crate::dram_edo;
use crate::psx;

#[derive(PartialEq, Clone)]
pub enum AccessMode {
    WriteDataInto,
    ReadDataFrom,
}
#[derive(Clone)]
pub struct Bus {
    address_bus: psx::Double,
    data_bus: psx::Double,
    current_mode: AccessMode,

    installed_memory: Option<Box<dram_edo::RamChip>>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            address_bus: 0,
            data_bus: 0,
            current_mode: AccessMode::ReadDataFrom,

            installed_memory: None,
        }
    }
    pub fn startup_memory(&mut self, ram_storage: &mut Box<dram_edo::RamChip>) {
        self.installed_memory = Some(ram_storage.clone())
    }

    pub fn register_access(
        &mut self,
        address_a: psx::Double,
        content: psx::Double,
        mode: AccessMode,
    ) {
        self.address_bus = address_a & 0x1fff_ffff;
        self.data_bus = content;
        self.current_mode = mode;
    }

    pub fn commit_access(&mut self) -> Option<psx::Double> {
        let memory_chip = self.installed_memory.as_mut().unwrap();
        let req_finished;

        if self.current_mode == AccessMode::WriteDataInto {
            memory_chip.write_u32(self.address_bus, self.data_bus);
            req_finished = true;
        } else {
            self.data_bus = memory_chip.read_u32(self.address_bus);
            req_finished = true;
        }

        if req_finished {
            return Some(self.data_bus);
        }
        None
    }
}
