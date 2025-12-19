pub mod monitor;

// ===== ENUMS (INLINE) =====

pub enum RamInfo {
    TotalMemory,
    UsedMemory,
}

pub enum SystemInfo {
    Name,
    KernelVersion,
    OSVersion,
    HostName,
}

pub enum CpuInfo {
    NBCpus,
}

// Re-export Sysmon
pub use monitor::Sysmon;