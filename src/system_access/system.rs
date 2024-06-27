use crate::backend::hm2::Hm2;
use crate::backend::hmc::HmC;
use crate::GameData;
use sysinfo::System;

/// Get the process ID of available games
pub fn get_game(sys: &mut System) -> Option<Box<dyn GameData>> {
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

    return None;
}
