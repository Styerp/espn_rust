use crate::api::league::DraftDetail;
use crate::api::id_maps::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    away: TeamMatchupPerformance,
    home: TeamMatchupPerformance,
    id: u16,
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
    pub team_id: FantasyTeamId,
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
    pub acquisition_date: Option<Timestamp>,
    #[serde(rename = "acquisitionType")]
    pub acquisition_type: Option<String>,
    #[serde(rename = "injuryStatus")]
    pub injury_status: Option<String>,
    #[serde(rename = "lineupSlotId")]
    pub lineup_slot_id: u8,
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
    #[serde(rename="rosterLocked")]
    pub roster_locked: bool,
    pub status: String,
    #[serde(rename="tradeLocked")]
    pub trade_locked: bool,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Player {
    pub active: bool,
    #[serde(rename = "defaultPositionId")]
    pub default_position_id: u8,
    pub droppable: bool,
    #[serde(rename = "eligibleSlots")]
    pub eligible_slots: Vec<PositionId>,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    pub id: PlayerId,
    pub injured: bool,
    #[serde(rename = "injuryStatus")]
    pub injury_status: Option<String>,
    #[serde(rename="lastName")]
    pub last_name: String,
    #[serde(rename="lastNewsDate")]
    pub last_news_date: Option<Timestamp>,
    #[serde(rename="proTeamId")]
    pub pro_team_id: ProTeamId,
    pub rankings: Option<HashMap<u8, Vec<Ranking>>>,
    pub stats: Vec<Stats>,
    #[serde(rename="universeId")]
    pub universe_id: u32
}
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Stats {
    #[serde(rename="appliedStats")]
    pub applied_stats: HashMap<u32, f32>,
    #[serde(rename="appliedTotal")]
    pub applied_total: f32,
    #[serde(rename="externalId")]
    pub external_id: String,
    pub id: String,
    #[serde(rename="proTeamId")]
    pub pro_team_id: ProTeamId,
    #[serde(rename="scoringPeriodId")]
    pub scoring_period_id: u8,
    #[serde(rename="seasonId")]
    pub season_id: u32,
    #[serde(rename="statSourceId")]
    pub stat_source_id: u32,
    #[serde(rename="statSplitTypeId")]
    pub stat_split_type_id: u32,
    pub stats: HashMap<u16, f32>,
    pub variance: Option<HashMap<u16, f32>>
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Ranking {
    #[serde(rename="auctionValue")]
    pub auction_value: f32,
    pub rank: u32,
    #[serde(rename="rankSourceId")]
    pub rank_source_id: u32,
    #[serde(rename="rankType")]
    pub rank_type: String,
    #[serde(rename="slotId")]
    pub slot_id: u32,
}
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct FantasyTeamId(u8);
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct PlayerId(i64);


#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Timestamp(u64);