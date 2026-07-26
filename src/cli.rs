use std::io::{self, Write};
use std::process::Command;

pub fn terminal_size() -> (u16, u16) {
    let output = Command::new("stty")
        .arg("-f")
        .arg("/dev/tty")
        .arg("size")
        .output();
    match output {
        Ok(o) if o.status.success() => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            let parts: Vec<&str> = stdout.trim().split_whitespace().collect();
            if parts.len() == 2 {
                let rows: u16 = parts[0].parse().unwrap_or(24);
                let cols: u16 = parts[1].parse().unwrap_or(80);
                return (cols, rows);
            }
            (80, 24)
        }
        _ => (80, 24),
    }
}

pub fn enable_raw_mode() {
    Command::new("stty")
        .arg("-f")
        .arg("/dev/tty")
        .arg("raw")
        .arg("-echo")
        .status()
        .unwrap();
}

pub fn disable_raw_mode() {
    Command::new("stty")
        .arg("-f")
        .arg("/dev/tty")
        .arg("sane")
        .status()
        .unwrap();
}

pub fn enter_alternate_screen() {
    print!("\x1B[?1049h");
    io::stdout().flush().unwrap();
}

pub fn leave_alternate_screen() {
    print!("\x1B[?1049l");
    io::stdout().flush().unwrap();
}
