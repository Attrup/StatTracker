use crate::app::{memory::*, system_access::get_process_window};
use crate::{Backend, GameData, MissionStats, Rating, Window};
use std::collections::HashMap;

/// Memory addresses
const BASE_ADDRESS: usize = 0x400000;
const MAP_ADDRESS: usize = 0x2A6C5C;
const TIMER_ADDRESS: usize = 0x2A6C58;
const DATA_ADDRESS: usize = 0x2A6C50;
const SHOTS_ADDRESS: usize = 0x3981C;

// Pointer offsets
const MAP: [usize; 2] = [0x98, 0xBC2];
const TIMER: [usize; 5] = [0x118, 0xB38, 0x8, 0x1084, 0x24];
const SHOTS_FIRED: [usize; 2] = [0x3CC, 0x11C7];
const CLOSE_ENCOUNTERS: [usize; 3] = [0x28, 0, 0x220];
const HEADSHOTS: [usize; 3] = [0x28, 0, 0x208];
const ALERTS: [usize; 3] = [0x28, 0, 0x21C];
const ENEMIES_KILLED: [usize; 3] = [0x28, 0, 0x210];
const ENEMIES_HARMED: [usize; 3] = [0x28, 0, 0x20C];
const INNOCENTS_KILLED: [usize; 3] = [0x28, 0, 0x218];
const INNOCENTS_HARMED: [usize; 3] = [0x28, 0, 0x214];

const MAP_OFFSETS: [usize; 20] = [
    0x838, 0xB24, 0x8A0, 0x138, 0xB88, 0xBB8, 0xB48, 0xCE8, 0x136C, 0xAD0, 0xF50, 0x8D4, 0x9EC,
    0x400, 0x9EC, 0x644, 0xB08, 0x96C, 0xB00, 0x8,
];

/// Valid silent assassin combinations
const SA_COMBINATIONS: [MissionStats; 27] = [
    MissionStats::new(0, 1, 0, 0, 1, 2, 0, 0),
    MissionStats::new(0, 1, 0, 0, 0, 5, 0, 0),
    MissionStats::new(0, 1, 0, 0, 0, 2, 0, 1),
    MissionStats::new(0, 0, 0, 1, 2, 0, 0, 0),
    MissionStats::new(0, 0, 0, 1, 1, 3, 0, 0),
    MissionStats::new(0, 0, 0, 1, 1, 0, 0, 1),
    MissionStats::new(0, 0, 0, 1, 0, 6, 0, 0),
    MissionStats::new(0, 0, 0, 1, 0, 3, 0, 1),
    MissionStats::new(0, 0, 0, 1, 0, 0, 1, 0),
    MissionStats::new(0, 0, 0, 1, 0, 0, 0, 2),
    MissionStats::new(0, 0, 0, 0, 1, 0, 0, 1),
    MissionStats::new(1, 1, 1, 0, 0, 2, 0, 0),
    MissionStats::new(1, 1, 0, 0, 1, 0, 0, 0),
    MissionStats::new(1, 1, 0, 0, 0, 3, 0, 0),
    MissionStats::new(1, 1, 0, 0, 0, 0, 0, 1),
    MissionStats::new(1, 0, 1, 1, 1, 0, 0, 0),
    MissionStats::new(1, 0, 1, 1, 0, 3, 0, 0),
    MissionStats::new(1, 0, 1, 1, 0, 0, 0, 1),
    MissionStats::new(1, 0, 0, 1, 1, 1, 0, 0),
    MissionStats::new(1, 0, 0, 1, 0, 4, 0, 0),
    MissionStats::new(1, 0, 0, 1, 0, 1, 0, 1),
    MissionStats::new(1, 0, 0, 0, 1, 1, 0, 0),
    MissionStats::new(2, 1, 1, 0, 0, 0, 0, 0),
    MissionStats::new(2, 1, 0, 0, 0, 1, 0, 0),
    MissionStats::new(2, 0, 2, 1, 0, 0, 0, 0),
    MissionStats::new(2, 0, 1, 1, 0, 1, 0, 0),
    MissionStats::new(3, 0, 0, 1, 0, 0, 0, 0),
];

pub struct Hm2 {
    pid: u32,
    map_decoder: HashMap<String, (String, Option<usize>)>,
    shots_fired_backup: u32,
}

impl Hm2 {
    pub fn new(pid: u32) -> Self {
        Hm2 {
            pid,
            map_decoder: HashMap::from([
                (
                    String::from("C0-1\\"),
                    (String::from("The Gontranno Sanctuary"), None),
                ),
                (String::from("C1-1\\"), (String::from("Anathema"), Some(0))),
                (
                    String::from("C2-1\\"),
                    (String::from("St. Petersburg Stakeout"), Some(1)),
                ),
                (
                    String::from("C2-2\\"),
                    (String::from("Kirov Park Meeting"), Some(2)),
                ),
                (
                    String::from("C2-3\\"),
                    (String::from("Tubeway Torpedo"), Some(3)),
                ),
                (
                    String::from("C2-4\\"),
                    (String::from("Invitation to a Party"), Some(4)),
                ),
                (
                    String::from("C3-1\\"),
                    (String::from("Tracking Hayamoto"), Some(5)),
                ),
                (
                    String::from("C3-2a"),
                    (String::from("Hidden Valley"), Some(6)),
                ),
                (
                    String::from("C3-2b"),
                    (String::from("At the Gates"), Some(7)),
                ),
                (
                    String::from("C3-3\\"),
                    (String::from("Shogun Showdown"), Some(8)),
                ),
                (
                    String::from("C4-1\\"),
                    (String::from("Basement Killing"), Some(9)),
                ),
                (
                    String::from("C4-2\\"),
                    (String::from("The Graveyard Shift"), Some(10)),
                ),
                (
                    String::from("C4-3\\"),
                    (String::from("The Jacuzzi Job"), Some(11)),
                ),
                (
                    String::from("C5-1\\"),
                    (String::from("Murder At The Bazaar"), Some(12)),
                ),
                (
                    String::from("C5-2\\"),
                    (String::from("The Motorcade Interception"), Some(13)),
                ),
                (
                    String::from("C5-3\\"),
                    (String::from("Tunnel Rat"), Some(14)),
                ),
                (
                    String::from("C6-1\\"),
                    (String::from("Temple City Ambush"), Some(15)),
                ),
                (
                    String::from("C6-2\\"),
                    (String::from("The Death of Hannelore"), Some(16)),
                ),
                (
                    String::from("C6-3\\"),
                    (String::from("Terminal Hospitality"), Some(17)),
                ),
                (
                    String::from("C7-1\\"),
                    (String::from("St. Petersburg Revisited"), Some(18)),
                ),
                (
                    String::from("C8-1\\"),
                    (String::from("Redemption at Gontranno"), Some(19)),
                ),
            ]),
            // Shots fired memory location is somewhat volatile so we need a backup
            shots_fired_backup: 0,
        }
    }

    /// Load all the game stats from program memory
    fn load_stats(&self, map_no: usize) -> Option<MissionStats> {
        let mut stats = [0; 8];

        // Shots fired is independent of the map, but the address tends to
        // shift around depending on the map and the player location. To
        // combat this, we use a backup value if the read is unsuccessful.
        stats[0] = match decode_to_u32(read_memory(
            BASE_ADDRESS + SHOTS_ADDRESS,
            self.pid,
            4,
            SHOTS_FIRED.to_vec(),
        )) {
            Some(shots) => shots,
            None => self.shots_fired_backup,
        };

        // Remaing stats are dependent on the map
        for (i, stat) in [
            CLOSE_ENCOUNTERS,
            HEADSHOTS,
            ALERTS,
            ENEMIES_KILLED,
            ENEMIES_HARMED,
            INNOCENTS_KILLED,
            INNOCENTS_HARMED,
        ]
        .iter()
        .enumerate()
        {
            let mut offset = *stat;
            offset[1] = MAP_OFFSETS[map_no];

            stats[i + 1] = decode_to_u32(read_memory(
                BASE_ADDRESS + DATA_ADDRESS,
                self.pid,
                4,
                offset.to_vec(),
            ))?;
        }
        return Some(MissionStats::from_array(stats));
    }
}

impl Backend for Hm2 {
    fn update(&mut self) -> Option<GameData> {
        // Get map bytes and decode
        let map_bytes = match read_memory(BASE_ADDRESS + MAP_ADDRESS, self.pid, 5, MAP.to_vec()) {
            Ok(bytes) => bytes,
            Err(_) => return None,
        };

        let (map_name, ratings, enable_timer) =
            match self.map_decoder.get(&decode_to_string(map_bytes)?) {
                Some((map, ratings)) => (map.as_str(), ratings, true),
                None => ("Hitman 2 SA", &None, false),
            };

        if enable_timer {
            // Get mission timer
            let mission_time = match decode_to_u32(read_memory(
                BASE_ADDRESS + TIMER_ADDRESS,
                self.pid,
                4,
                TIMER.to_vec(),
            )) {
                Some(time) => time,
                None => 0,
            };

            // Only get ratings if they are active on current map
            if mission_time > 0 {
                match *ratings {
                    Some(map_no) => {
                        // Get game stats
                        let stats = self.load_stats(map_no)?;

                        // Backup shots fired
                        self.shots_fired_backup = stats.shots_fired;

                        // Check for SA rating
                        let mut silent_assasin = false;

                        for combination in SA_COMBINATIONS {
                            if stats <= combination {
                                silent_assasin = true;
                                break;
                            }
                        }

                        return Some(GameData::new(
                            map_name.to_string(),
                            mission_time,
                            Some(Rating::new(stats, silent_assasin)),
                        ));
                    }
                    None => return Some(GameData::new(map_name.to_string(), mission_time, None)),
                }
            }
        }
        return Some(GameData::new(map_name.to_string(), 0, None));
    }

    fn game_window(&self) -> Option<Window> {
        get_process_window("Hitman2")
    }
}
