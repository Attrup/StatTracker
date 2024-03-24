/// Shared API for game data retrieval
pub trait GameData {
    fn update(&mut self) -> Option<(&str, u32, Option<([u32; 8], bool)>)>;
}

/// Modules for each supported game
pub mod backend {
    pub mod hm2;
    pub mod hmc;
}

/// GUI application
pub mod app{
    pub mod gui;
    pub mod misc;
}

// Memory reading + system calls
pub mod system_access {
    pub mod memory;
    pub mod system;
}

