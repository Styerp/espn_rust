use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    id_maps::PositionId,
    league::DraftDetail,
    player::{Player, PlayerId},
    team::TeamId,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MatchupResponse {
    #[serde(rename = "draftDetail")]
    pub draft_detail: Option<DraftDetail>,
    #[serde(rename = "gameId")]
    pub game_id: i32,
    pub id: u128,
    #[serde(rename = "segmentId")]
    pub segment_id: i8,
    #[serde(rename = "scoringPeriodId")]
    pub scoring_period_id: i8,
    pub schedule: Vec<Matchup>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Matchup {
    pub away: TeamMatchupPerformance,
    pub home: TeamMatchupPerformance,
    pub id: u16,
    #[serde(rename = "matchupPeriodId")]
    pub matchup_period_id: u8,
    pub winner: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TeamMatchupPerformance {
    #[serde(rename = "cumulativeScore")]
    pub cumulative_score: CumulativeScore,
    #[serde(rename = "gamesPlayed")]
    pub games_played: i32,
    #[serde(rename = "pointsByScoringPeriod")]
    pub points_by_scoring_period: Option<HashMap<i8, f32>>,
    #[serde(rename = "rosterForCurrentScoringPeriod")]
    pub roster_for_current_scoring_period: Option<Roster>,
    #[serde(rename = "teamId")]
    pub team_id: TeamId,
    #[serde(rename = "totalPoints")]
    pub total_points: f32,
}
#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct CumulativeScore {
    pub losses: u8,
    #[serde(rename = "scoreByStat")]
    pub score_by_stat: Option<ScoreByStat>,
    pub ties: u8,
    pub wins: u8,
}

#[derive(Debug, Deserialize, Serialize, Default, Copy, Clone)]
pub struct ScoreByStat {
    ineligible: Option<bool>,
    rank: Option<u32>,
    result: Option<f32>,
    score: Option<f32>,
}
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Roster {
    #[serde(rename = "appliedStatTotal")]
    pub applied_stat_total: f32,
    pub entries: Vec<RosterSlot>,
}
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct RosterSlot {
    #[serde(rename = "acquisitionDate")]
    pub acquisition_date: Option<u64>,
    #[serde(rename = "acquisitionType")]
    pub acquisition_type: Option<String>,
    #[serde(rename = "injuryStatus")]
    pub injury_status: Option<String>,
    #[serde(rename = "lineupSlotId")]
    pub lineup_slot_id: PositionId,
    #[serde(rename = "pendingTransactionIds")]
    pub pending_transaction_ids: Option<Vec<u64>>,
    #[serde(rename = "playerId")]
    pub player_id: PlayerId,
    #[serde(rename = "playerPoolEntry")]
    pub player_pool_entry: PlayerPoolEntry,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct PlayerPoolEntry {
    #[serde(rename = "appliedStatTotal")]
    pub applied_stat_total: f32,
    pub id: PlayerId,
    #[serde(rename = "keeperValue")]
    pub keeper_value: u8,
    #[serde(rename = "keeperValueFuture")]
    pub keeper_value_future: u8,
    #[serde(rename = "lineupLocked")]
    pub lineup_locked: bool,
    #[serde(rename = "onTeamId")]
    pub on_team_id: FantasyTeamId,
    pub player: Player,
    #[serde(rename = "rosterLocked")]
    pub roster_locked: bool,
    pub status: String,
    #[serde(rename = "tradeLocked")]
    pub trade_locked: bool,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, Eq, PartialEq)]
pub struct FantasyTeamId(u8);
