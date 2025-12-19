use ratatui::{
    Frame,
    layout::{Layout, Constraint, Direction},
    widgets::{Block, Borders, Table, Row, Paragraph},
    style::{Style, Color},
};

use crate::sysmon::*;

/// Draws the full TUI each frame
pub fn draw(frame: &mut Frame) {
    let sysmon = Sysmon::new();

    // RAM
    let total_ram = format!(
        "{:.2} GB",
        Sysmon::bytes_to_gb(sysmon.ram(RamInfo::TotalMemory))
    );
    let used_ram = format!(
        "{:.2} GB",
        Sysmon::bytes_to_gb(sysmon.ram(RamInfo::UsedMemory))
    );

    // Table rows
    let rows = vec![
        row("OS NAME", sysmon.system_info(SystemInfo::Name)),
        row("OS VERSION", sysmon.system_info(SystemInfo::OSVersion)),
        row("KERNEL", sysmon.system_info(SystemInfo::KernelVersion)),
        row("HOST", sysmon.system_info(SystemInfo::HostName)),
        row(
            "NB CPUS",
            sysmon.cpu_info(CpuInfo::NBCpus).map(|v| v.to_string()),
        ),
        row("DISKS", sysmon.disk_info(DiskInfo::DiskNames)),
        Row::new(vec!["RAM TOTAL".into(), total_ram]),
        Row::new(vec!["RAM USED".into(), used_ram]),
    ];

    // ===== Table =====
    let table = Table::new(rows, &[Constraint::Length(20), Constraint::Min(10)])
        .header(
            Row::new(vec!["KEY", "VALUE"])
                .style(Style::default().fg(Color::Yellow)),
        )
        .block(
            Block::default()
                .title(" System Monitor ")
                .borders(Borders::ALL),
        );

    // ===== Layout: vertical split (table + footer) =====
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Length(1)])
        .split(frame.area());

    // Render table
    frame.render_widget(table, chunks[0]);

    // Render footer
    let footer = Paragraph::new("Press 'q' to quit")
        .style(Style::default().fg(Color::White).bg(Color::Blue));
    frame.render_widget(footer, chunks[1]);
}

/// Helper to create a table row from label + optional value
fn row(label: &str, value: Option<String>) -> Row {
    Row::new(vec![label.into(), value.unwrap_or_else(|| "Unknown".into())])
}