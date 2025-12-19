mod sysmon;
mod ui;

use std::{io, time::Duration};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
    event::{self, Event, KeyCode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use ui::app::App;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::default();

    let res = run_app(&mut terminal, &mut app);

    // ===== CLEANUP (ALWAYS RUNS) =====
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    while !app.should_quit {
        terminal.draw(|f| ui::tui::draw(f))?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.quit(),
                    _ => {}
                }
            }
        }
    }
    Ok(())
}



// #![allow(warnings)]
// use sysinfo::{System,Disks};
// use tabled::{builder::Builder, settings::Style};
// use crossterm::{
//     execute,
//     cursor::MoveTo,
//     terminal::{Clear, ClearType},
// };
// use std::io::{stdout, Write};

// struct TableStruct {
//     sysmon: Sysmon,
//     disks: Disks,
// }

// enum TableData {
//     RamInfo,
//     SystemInfo,
//     CpuInfo,
//     NetworkInfo,
//     DiskInfo,
// }

// struct Sysmon {
//     system: System,
//     disks: Disks,
// }

// enum RamInfo {
//     TotalMemory,
//     UsedMemory,
// }

// enum SystemInfo {
//     Name,
//     KernelVersion,
//     OSVersion,
//     HostName,
//     Process,
//     Components,
// }

// enum CpuInfo {
//     NBCpus,
// }

// enum NetworkInfo {
//     InterfaceName,
//     TotalReceived,
//     TotalTransmitted,
// }

// enum DiskInfo {
//     DiskNames,
// }

// impl Sysmon {
//     fn ram(&self, info: RamInfo) -> u64 {
//         match info {
//             RamInfo::TotalMemory => self.system.total_memory(),
//             RamInfo::UsedMemory => self.system.used_memory(),
//         }
//     }

//     fn bytes_to_gb(bytes: u64) -> f64 {
//         bytes as f64 / 1024.0 / 1024.0 / 1024.0
//     }

//     fn system_info(&self, info: SystemInfo) -> Option<String> {
//         match info {
//             SystemInfo::Name => System::name(),
//             SystemInfo::KernelVersion => System::kernel_version(),
//             SystemInfo::OSVersion => System::os_version(),
//             SystemInfo::HostName => System::host_name(),
//             _ => None,
//         }
//     }

//     fn cpu_info(&self, info: CpuInfo) -> Option<usize> {
//         match info {
//             CpuInfo::NBCpus => Some(self.system.cpus().len()),
//         }
//     }

//     fn disk_info(&self, info: DiskInfo) -> Option<String> {
//         match info {
//             DiskInfo::DiskNames => Some(
//                 self.disks
//                     .iter()
//                     .map(|d| d.name().to_string_lossy().into_owned())
//                     .collect::<Vec<String>>()
//                     .join(", "),
//             ),
//         }
//     }
// }

// fn main() {
//     loop {
//         display_data();
//     }
// }

// fn display_data() {
//     let mut stdout = stdout();

//     let mut table_height: usize = 0;

//     let mut sysmon = Sysmon {
//         system: System::new_all(),
//         disks: Disks::new(),
//     };

//     sysmon.system.refresh_memory();

//     let raw_total: u64 = sysmon.ram(RamInfo::TotalMemory);
//     let raw_used: u64  = sysmon.ram(RamInfo::UsedMemory);

//     let gb_total = format!("{:.2} GB", Sysmon::bytes_to_gb(raw_total));
//     let gb_used  = format!("{:.2} GB", Sysmon::bytes_to_gb(raw_used));


//     let nb_cpus = sysmon.cpu_info(CpuInfo::NBCpus).unwrap_or(0);

//     let os_name = sysmon.system_info(SystemInfo::Name).unwrap_or("Unknown".into());
//     let os_version = sysmon.system_info(SystemInfo::OSVersion).unwrap_or("Unknown".into());
//     let kernel_version = sysmon.system_info(SystemInfo::KernelVersion).unwrap_or("Unknown".into());
//     let host_name = sysmon.system_info(SystemInfo::HostName).unwrap_or("Unknown".into());

//     let disk_info = sysmon
//         .disk_info(DiskInfo::DiskNames)
//         .unwrap_or_else(|| "No disks found".to_string());

//     let mut builder = Builder::new();
//     builder.push_record(["OS NAME", &os_name]);
//     builder.push_record(["OS VERSION", &os_version]);
//     builder.push_record(["KERNEL VERSION", &kernel_version]);
//     builder.push_record(["HOST NAME", &host_name]);
//     builder.push_record(["DISKS", &disk_info]);
//     builder.push_record(["NB CPUS", &nb_cpus.to_string()]);
//     builder.push_record(["RAM TOTAL", &gb_total]);
//     builder.push_record(["RAM USED", &gb_used]);

//     let mut table = builder.build();
//     table.with(Style::ascii_rounded());

//     // ===== ONLY TABLE RENDERING LOGIC BELOW =====

//     let table_string = table.to_string();
//     let height = table_string.lines().count();

//     unsafe {
//         execute!(
//             stdout,
//             MoveTo(0, 0),
//             Clear(ClearType::FromCursorDown)
//         ).unwrap();

//         print!("{}", table_string);
//         stdout.flush().unwrap();

//         table_height = height;
//     }

//     // ===========================================

//     std::thread::sleep(std::time::Duration::from_secs(1));
// }
