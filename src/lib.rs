/// Shared API for game data retrieval
pub trait Backend {
    fn update(&mut self) -> Option<(&str, u32, Option<([u32; 8], bool)>)>;

    fn game_window(&self) -> Option<Window>;
}

// Game Window Positions
pub struct Window {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

/// Application
pub mod app {
    // App Components
    pub mod main;
    pub mod overlay;

    // App Setup
    pub mod colors;
    pub mod fonts;

    // System Access {Memory reading + system calls)
    pub mod memory;
    pub mod system_access;

    /// Backends for each supported game
    pub mod backends {
        pub mod hm2;
        pub mod hmc;
    }
}
