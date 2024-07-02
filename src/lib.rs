/// Shared API for game data retrieval
pub trait GameData {
    fn update(&mut self) -> Option<(&str, u32, Option<([u32; 8], bool)>)>;
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
    pub mod system;

    /// Backends for each supported game
    pub mod backends {
        pub mod hm2;
        pub mod hmc;
    }
}
