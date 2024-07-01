/// Shared API for game data retrieval
pub trait GameData {
    fn update(&mut self) -> Option<(&str, u32, Option<([u32; 8], bool)>)>;
}

/// Shared functions for app setup
pub mod setup {
    pub mod colors;
    pub mod fonts;
}

/// Backends for each supported game
pub mod backends {
    pub mod hm2;
    pub mod hmc;
}

/// Memory reading + system calls
pub mod system_access {
    pub mod memory;
    pub mod system;
}

/// Applications
pub mod applications {
    pub mod overlay {
        pub mod app;
        pub mod input;
    }

    pub mod standalone {
        pub mod app;
        pub mod overlay_control;
    }
}
