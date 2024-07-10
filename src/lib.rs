/// Shared API for game data retrieval
pub trait Backend {
    fn update(&mut self) -> Option<GameData>;

    fn game_window(&self) -> Option<Window>;
}

/// Structs for passing data retrieved from the game to the GUI
pub struct GameData {
    pub mission_name: String,
    pub mission_time: u32,
    pub rating: Option<Rating>,
}

impl GameData {
    pub fn new(mission_name: String, mission_time: u32, rating: Option<Rating>) -> Self {
        GameData {
            mission_name,
            mission_time,
            rating,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Rating {
    pub stats: MissionStats,
    pub sa_rating: bool,
}

impl Rating {
    pub fn new(stats: MissionStats, sa_rating: bool) -> Self {
        Rating { stats, sa_rating }
    }
}
#[derive(Copy, Clone, Debug)]
pub struct MissionStats {
    pub shots_fired: u32,
    pub close_encounters: u32,
    pub headshots: u32,
    pub alerts: u32,
    pub enemies_killed: u32,
    pub enemies_harmed: u32,
    pub innocents_killed: u32,
    pub innocents_harmed: u32,
}

impl Default for MissionStats {
    fn default() -> Self {
        MissionStats {
            shots_fired: 0,
            close_encounters: 0,
            headshots: 0,
            alerts: 0,
            enemies_killed: 0,
            enemies_harmed: 0,
            innocents_killed: 0,
            innocents_harmed: 0,
        }
    }
}

impl MissionStats {
    pub const fn new(
        shots_fired: u32,
        close_encounters: u32,
        headshots: u32,
        alerts: u32,
        enemies_killed: u32,
        enemies_harmed: u32,
        innocents_killed: u32,
        innocents_harmed: u32,
    ) -> Self {
        MissionStats {
            shots_fired,
            close_encounters,
            headshots,
            alerts,
            enemies_killed,
            enemies_harmed,
            innocents_killed,
            innocents_harmed,
        }
    }

    pub fn from_array(arr: [u32; 8]) -> Self {
        MissionStats {
            shots_fired: arr[0],
            close_encounters: arr[1],
            headshots: arr[2],
            alerts: arr[3],
            enemies_killed: arr[4],
            enemies_harmed: arr[5],
            innocents_killed: arr[6],
            innocents_harmed: arr[7],
        }
    }
}

// Game Window Position
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
        pub mod backend_helpers;
        pub mod hm2;
        pub mod hmc;
    }
}
