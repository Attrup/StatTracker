use crate::app::memory::*;
use crate::GameData;
use std::collections::HashMap;

/// Memory addresses
const BASE_ADDRESS: usize = 0x400000;
const MAP_ADDRESS: usize = 0x393D58;
const TIMER_ADDRESS: usize = 0x39457C;
const DATA_ADDRESS: usize = 0x3947C0;
const SHOTS_ADDRESS: usize = 0x3947B0;

// Pointer offsets
const MAP: [usize; 2] = [0x234, 0xBDE];
const TIMER: [usize; 1] = [0x24];
const SHOTS_FIRED: [usize; 3] = [0xBA0, 0x104, 0x82F];
const CLOSE_ENCOUNTERS: [usize; 1] = [0xB2F];
const HEADSHOTS: [usize; 1] = [0xB17];
const ALERTS: [usize; 1] = [0xB2B];
const ENEMIES_KILLED: [usize; 1] = [0xB1F];
const ENEMIES_HARMED: [usize; 1] = [0xB1B];
const INNOCENTS_KILLED: [usize; 1] = [0xB27];
const INNOCENTS_HARMED: [usize; 1] = [0xB23];

/// Valid silent assassin combinations
const SA_COMBINATIONS: [[u32; 8]; 17] = [
    [999, 0, 999, 1, 0, 0, 0, 0],
    [2, 1, 1, 0, 0, 0, 0, 0],
    [2, 1, 0, 0, 0, 1, 0, 0],
    [2, 0, 1, 1, 0, 1, 0, 0],
    [2, 0, 0, 0, 0, 2, 0, 0],
    [1, 1, 1, 0, 0, 2, 0, 0],
    [1, 1, 0, 0, 1, 0, 0, 0],
    [1, 1, 0, 0, 0, 3, 0, 0],
    [1, 0, 1, 1, 1, 0, 0, 0],
    [1, 0, 1, 1, 0, 3, 0, 0],
    [1, 0, 0, 1, 1, 1, 0, 0],
    [1, 0, 0, 1, 0, 4, 0, 0],
    [0, 1, 0, 0, 1, 2, 0, 0],
    [0, 1, 0, 0, 0, 5, 0, 0],
    [0, 0, 0, 1, 1, 3, 0, 0],
    [0, 0, 0, 1, 2, 0, 0, 0],
    [0, 0, 0, 1, 0, 6, 0, 0],
];

pub struct HmC {
    pid: u32,
    map_decoder: HashMap<String, String>,
}

impl HmC {
    pub fn new(pid: u32) -> Self {
        HmC {
            pid,
            map_decoder: HashMap::from([
                (String::from("C00-1"), String::from("Training")),
                (String::from("C01-1"), String::from("Asylum Aftermatch")),
                (String::from("C01-2"), String::from("The Meat King's Party")),
                (String::from("C02-1"), String::from("The Bjarkhov Bomb")),
                (String::from("C03-1"), String::from("Beldingford Manor")),
                (
                    String::from("C06-1"),
                    String::from("Rendezvous in Rotterdam"),
                ),
                (String::from("C06-1"), String::from("Deadly Cargo")),
                (
                    String::from("C07-1"),
                    String::from("Traditions of the Trade"),
                ),
                (String::from("C08-1"), String::from("Slaying a Dragon")),
                (String::from("C08-2"), String::from("The Wang Fou Incident")),
                (String::from("C08-3"), String::from("The Seafood Massacre")),
                (
                    String::from("C08-4"),
                    String::from("Lee Hong Assassination"),
                ),
                (String::from("C09-1"), String::from("Hunter and Hunted")),
            ]),
        }
    }

    fn load_stats(&self) -> Option<[u32; 8]> {
        let mut stats = [0; 8];

        // Shots has no map dependent pointer
        stats[0] = decode_to_u32(read_memory(
            BASE_ADDRESS + SHOTS_ADDRESS,
            self.pid,
            4,
            SHOTS_FIRED.to_vec(),
        ))?;

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
            stats[i + 1] = decode_to_u32(read_memory(
                BASE_ADDRESS + DATA_ADDRESS,
                self.pid,
                4,
                stat.to_vec(),
            ))?;
        }
        return Some(stats);
    }
}

impl GameData for HmC {
    fn update(&mut self) -> Option<(&str, u32, Option<([u32; 8], bool)>)> {
        // Get map bytes and decode
        let map_bytes = match read_memory(BASE_ADDRESS + MAP_ADDRESS, self.pid, 5, MAP.to_vec()) {
            Ok(bytes) => bytes,
            Err(_) => return None,
        };

        let (map_name, enable_timer) = match self.map_decoder.get(&decode_to_string(map_bytes)?) {
            Some(map) => (map.as_str(), true),
            None => ("Hitman Contracts", false),
        };

        if enable_timer {
            // Get mission timer
            let mission_time = match decode_to_f32(read_memory(
                BASE_ADDRESS + TIMER_ADDRESS,
                self.pid,
                4,
                TIMER.to_vec(),
            )) {
                Some(time) => (60.0 * time) as u32,
                None => 0,
            };

            // Only get ratings if they are active on current map
            if mission_time > 0 {
                // Get game stats
                let stats = self.load_stats()?;

                // Check silent assasin rating
                let mut silent_assasin = false;

                for combination in SA_COMBINATIONS {
                    if stats
                        .iter()
                        .zip(&combination)
                        .filter(|&(stats, combination)| stats <= combination)
                        .count()
                        == 8
                    {
                        silent_assasin = true;
                        break;
                    }
                }

                return Some((map_name, mission_time, Some((stats, silent_assasin))));
            }
        }
        return Some((map_name, 0, None));
    }
}
