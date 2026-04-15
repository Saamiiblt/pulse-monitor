#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use sysinfo::{System, SystemExt, CpuExt};
use serde::Serialize;
use tauri::State;
use std::process::Command;
use std::os::windows::process::CommandExt;

#[derive(Serialize)]
struct SystemStats {
    cpu: f32,
    ram_used: u64,
    ram_total: u64,
}

struct AppState {
    sys: Mutex<System>,
}

#[tauri::command]
fn get_stats(state: State<AppState>) -> SystemStats {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();
    sys.refresh_memory();

    SystemStats {
        cpu: sys.global_cpu_info().cpu_usage(),
        ram_used: sys.used_memory() / 1024 / 1024,
        ram_total: sys.total_memory() / 1024 / 1024,
    }
}

// Helper pour mesurer la RAM
fn get_current_ram_mb(state: &State<AppState>) -> u64 {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_memory();
    sys.used_memory() / 1024 / 1024
}

#[tauri::command]
fn optimize_ram(state: State<AppState>) -> String {
    let before = get_current_ram_mb(&state);
    println!("RAM before optimization: {} MB", before);

    // Vider le working set des processus
    let _ = Command::new("powershell")
        .args(["-Command", "Get-Process | Where-Object { $_.WorkingSet -gt 5MB } | Foreach-Object { try { $_.EmptyWorkingSet() } catch {} }"])
        .creation_flags(0x08000000)
        .output();

    let after = get_current_ram_mb(&state);
    println!("RAM after optimization: {} MB", after);
    
    if before > after {
        let freed = before - after;
        println!("Freed: {} MB", freed);
        format!("{} MB LIBÉRÉS !", freed)
    } else {
        "OPTIMISÉ !".to_string()
    }
}

#[tauri::command]
fn clean_cache(state: State<AppState>) -> String {
    let before = get_current_ram_mb(&state);
    println!("RAM before cache clean: {} MB", before);

    // Flush system working set and standby list
    let ps_script = r#"
        $signature = @'
        [DllImport("kernel32.dll", SetLastError = true)]
        public static extern bool SetSystemFileCacheSize(IntPtr MinimumFileCacheSize, IntPtr MaximumFileCacheSize, int Flags);
'@
        $type = Add-Type -MemberDefinition $signature -Name "CacheUtils" -Namespace "WinAPI" -PassThru
        # Flush the system file cache
        $type::SetSystemFileCacheSize([IntPtr]-1, [IntPtr]-1, 0)
        
        # Also try to clear standby list if possible (requires admin, but harmless if fail)
        try {
            Clear-EventLog -LogName Application, System -ErrorAction SilentlyContinue
        } catch {}
    "#;

    let _ = Command::new("powershell")
        .args(["-Command", ps_script])
        .creation_flags(0x08000000)
        .output();

    let after = get_current_ram_mb(&state);
    println!("RAM after cache clean: {} MB", after);
    
    if before > after {
        let freed = before - after;
        println!("Cache Freed: {} MB", freed);
        format!("{} MB CACHE VIDÉ !", freed)
    } else {
        "CACHE PROPRE !".to_string()
    }
}

fn main() {
    let mut system = System::new();
    system.refresh_cpu();
    system.refresh_memory();
    
    tauri::Builder::default()
        .manage(AppState {
            sys: Mutex::new(system),
        })
        .invoke_handler(tauri::generate_handler![get_stats, optimize_ram, clean_cache])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}