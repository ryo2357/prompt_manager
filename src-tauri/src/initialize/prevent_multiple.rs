use sysinfo::{System, SystemExt, ProcessExt};
use std::env::current_exe;

pub fn prevent_multiple(){
    if is_multiple_activation() {
        std::process::exit(1);
    }
}

fn is_multiple_activation() -> bool {
    let my_path = current_exe().unwrap().display().to_string();
    let mut count = 0;
    for (_, process) in System::new_all().processes() {
        if my_path == process.exe().display().to_string() {
            if count > 1 { return true; };
            count += 1;
        }
    }
    return count > 1;
}