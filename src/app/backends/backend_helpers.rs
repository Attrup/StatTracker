use crate::MissionStats;

/// Function to calculate 'less than' or 'equal to' between two MissionStats objects
/// to determine SA rating
pub fn sa_compare(stats: MissionStats, sa_combination: MissionStats) -> bool {
    stats.shots_fired <= sa_combination.shots_fired
        && stats.close_encounters <= sa_combination.close_encounters
        && stats.headshots <= sa_combination.headshots
        && stats.alerts <= sa_combination.alerts
        && stats.enemies_killed <= sa_combination.enemies_killed
        && stats.enemies_harmed <= sa_combination.enemies_harmed
        && stats.innocents_killed <= sa_combination.innocents_killed
        && stats.innocents_harmed <= sa_combination.innocents_harmed
}
