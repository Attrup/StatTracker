use super::backends::{hm2::Hm2, hmc::HmC};
use crate::{Backend, Window};

use std::ptr::null;
use sysinfo::System;
use windows_sys::Win32::{
    Foundation::{HWND, RECT},
    UI::WindowsAndMessaging::{FindWindowW, GetWindowRect},
};

/// Get the process ID of available games
pub fn get_game(sys: &mut System) -> Option<Box<dyn Backend>> {
    // Refresh all runnning processes and match name with supported games
    sys.refresh_processes();

    // Check if Hitman 2 SA is running:
    if let Some(process) = sys.processes_by_exact_name("hitman2.exe").next() {
        return Some(Box::new(Hm2::new(process.pid().as_u32())));
    }

    // Check if Hitman Contracts is running:
    if let Some(process) = sys.processes_by_exact_name("HitmanContracts.exe").next() {
        return Some(Box::new(HmC::new(process.pid().as_u32())));
    }

    None
}

// Get the on screen position of the active game window
pub fn get_process_window(name: &str) -> Option<Window> {
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };

    // Get the window ID of the game
    let id: HWND = {
        // The Win32 API requires a null-terminated wide string
        let mut name = name.encode_utf16().collect::<Vec<u16>>();
        name.push(0);
        unsafe { FindWindowW(null(), name.as_ptr()) }
    };

    if id != 0 && unsafe { GetWindowRect(id, &mut rect) } != 0 {
        return Some(Window {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        });
    }

    None
}
