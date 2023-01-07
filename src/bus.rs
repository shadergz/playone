
use crate::dram_edo;
use crate::psx_comm;

#[derive(PartialEq, Clone)]
pub enum AccessMode {
    WriteDataInto,
    ReadDataFrom,
}

#[derive(Clone)]
pub struct Bus {
    address_bus: psx_comm::DoubleWord,
    data_bus: psx_comm::DoubleWord,
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
        address_a: psx_comm::DoubleWord,
        content: psx_comm::DoubleWord,
        mode: AccessMode,
    ) {
        self.address_bus = address_a;
        self.data_bus = content;
        self.current_mode = mode;
    }

    pub fn commit_access(&mut self) -> Option<psx_comm::DoubleWord> {
        let memory_chip = self.installed_memory.as_mut().unwrap();

        if self.current_mode == AccessMode::WriteDataInto {
            memory_chip.write_fast_uint32le(self.address_bus, self.data_bus);
        } else {
            Some(self.data_bus = memory_chip.read_fetch_uint32le(self.address_bus));
        }
        None
    }
}
