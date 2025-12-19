use sysinfo::{System,Disks};

struct Sysmon {
    system: System,
    disks: Disks,
}

enum RamInfo {
    TotalMemory,
    UsedMemory,
}

enum SystemInfo {
    Name,
    KernelVersion,
    OSVersion,
    HostName,
    Process,
    Components,
}

enum CpuInfo {
    NBCpus,
}

enum NetworkInfo {
    InterfaceName,
    TotalReceived,
    TotalTransmitted,
}

enum DiskInfo {
    DiskNames,
}

impl Sysmon {
    /// Get RAM information in KB
    fn ram(&self, info: RamInfo) -> u64 {
        match info {
            RamInfo::TotalMemory => self.system.total_memory(),
            RamInfo::UsedMemory => self.system.used_memory(),
        }
    }

    /// Convert KB to GB
    fn kb_to_gb(kb: u64) -> f64 {
        kb as f64 / 1024.0 / 1024.0
    }

    /// Get system information (OS name, kernel, etc.)
    fn system_info(&self, info: SystemInfo) -> Option<String> {
        match info {
            SystemInfo::Name => System::name(),
            SystemInfo::KernelVersion => System::kernel_version(),
            SystemInfo::OSVersion => System::os_version(),
            SystemInfo::HostName => System::host_name(),
            _ => None,
        }
    }

    fn cpu_info(&self, info: CpuInfo) -> Option<usize> {
        match info {
            CpuInfo::NBCpus => 
            Some(self.system.cpus().len()),
        }
    }

    // fn network_info(&self, _info: NetworkInfo) -> Option<String> {
    //     match info {
    //         NetworkInfo::InterfaceName => {
    //             // Placeholder for network interface name retrieval
    //             NetworkInfo::InterfaceName => None,
    //         }
    //         NetworkInfo::TotalReceived => {
    //             // Placeholder for total received bytes retrieval
    //             NetworkInfo::TotalReceived => None,
    //         }
    //         NetworkInfo::TotalTransmitted => {
    //             // Placeholder for total transmitted bytes retrieval
    //             NetworkInfo::TotalTransmitted => None,
    //         }
    //     }
    // }

    fn disk_info(&self, info: DiskInfo) -> Option<String> {
        match info {
            DiskInfo::DiskNames => Some(
                self.disks
                    .iter()
                    .map(|d| d.name().to_string_lossy().into_owned())
                    .collect::<Vec<String>>()
                    .join(", "),
            ),
        }
    }
}

fn main() {
    loop {
        display_data();
    }
}

fn display_data() {
    
    let mut sysmon = Sysmon {
        system: System::new_all(),
        disks: Disks::new(),
    };
    // Refresh only RAM information
    sysmon.system.refresh_memory();

    // RAM info
    let total = sysmon.ram(RamInfo::TotalMemory);
    let used = sysmon.ram(RamInfo::UsedMemory);

    println!(
        "RAM: {:.2} / {:.2} GB used",
        Sysmon::kb_to_gb(used),
        Sysmon::kb_to_gb(total),
    );

    // System info
    let os_name = sysmon
        .system_info(SystemInfo::Name)
        .unwrap_or_else(|| "Unknown".to_string());
    let kernel_version = sysmon
        .system_info(SystemInfo::KernelVersion)
        .unwrap_or_else(|| "Unknown".to_string());
    let os_version = sysmon
        .system_info(SystemInfo::OSVersion)
        .unwrap_or_else(|| "Unknown".to_string());
    let host_name = sysmon
        .system_info(SystemInfo::HostName)
        .unwrap_or_else(|| "Unknown".to_string());
    let process_name = sysmon
        .system_info(SystemInfo::Process)
        .unwrap_or_else(|| "Unknown".to_string());
    let components = sysmon
        .system_info(SystemInfo::Components)
        .unwrap_or_else(|| "Unknown".to_string());
    
    println!("Host Name: {}", host_name);
    println!("OS: {}", os_name);
    println!("OS Version: {}", os_version);
    println!("Kernel Version: {}", kernel_version);
    println!("Process ID: {}", process_name);
    println!("Components: {}", components);

    // CPU info
    let nb_cpus = sysmon
        .cpu_info(CpuInfo::NBCpus)
        .unwrap_or(0);

    println!("Number of CPUs: {}", nb_cpus);

    // // Network Info
    // let nic_name = sysmon
    //     .network_info(NetworkInfo::InterfaceName)
    //     .unwrap_or_default();
    // let nic_total_received = sysmon
    //     .network_info(NetworkInfo::TotalReceived)
    //     .unwrap_or_default();
    // let nic_total_transmitted = sysmon
    //     .network_info(NetworkInfo::TotalTransmitted)
    //     .unwrap_or_default();

    // println!("Network Interface: {}", nic_name);
    // println!("Network Bytes TotalReceived: {}", nic_total_received);
    // println!("Network Bytes TotalTransmitted: {}", nic_total_transmitted);

    // Disk info
    let disk_info = sysmon
        .disk_info(DiskInfo::DiskNames)
        .unwrap_or_else(|| "No disks found".to_string());

    println!("Disks: {}", disk_info);

    std::thread::sleep(std::time::Duration::from_secs(1));
}