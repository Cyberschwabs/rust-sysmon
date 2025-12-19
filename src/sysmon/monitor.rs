use sysinfo::{System, Disks};
use super::*; // <-- THIS pulls in the enums

pub struct Sysmon {
    pub system: System,
    pub disks: Disks,
}

impl Sysmon {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_memory();

        Self {
            system,
            disks: Disks::new(),
        }
    }

    pub fn ram(&self, info: RamInfo) -> u64 {
        match info {
            RamInfo::TotalMemory => self.system.total_memory(),
            RamInfo::UsedMemory => self.system.used_memory(),
        }
    }

    pub fn bytes_to_gb(bytes: u64) -> f64 {
        bytes as f64 / 1024.0 / 1024.0 / 1024.0
    }

    pub fn system_info(&self, info: SystemInfo) -> Option<String> {
        match info {
            SystemInfo::Name => System::name(),
            SystemInfo::KernelVersion => System::kernel_version(),
            SystemInfo::OSVersion => System::os_version(),
            SystemInfo::HostName => System::host_name(),
        }
    }

    pub fn cpu_info(&self, info: CpuInfo) -> Option<usize> {
        match info {
            CpuInfo::NBCpus => Some(self.system.cpus().len()),
        }
    }

    pub fn disk_info(&self, info: DiskInfo) -> Option<String> {
        match info {
            DiskInfo::DiskNames => Some(
                self.disks
                    .iter()
                    .map(|d| d.name().to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
        }
    }
}