use crate::api::id_maps::{PositionId, ProTeamId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::id_maps::StatId;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize, Default, Clone, Hash, Copy)]
pub struct PlayerId(i64);

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Player {
    pub active: bool,
    #[serde(rename = "defaultPositionId")]
    pub default_position_id: PositionId,
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
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "lastNewsDate")]
    pub last_news_date: Option<u64>,
    #[serde(rename = "proTeamId")]
    pub pro_team_id: ProTeamId,
    pub rankings: Option<HashMap<u8, Vec<Ranking>>>,
    pub stats: Option<Vec<Stats>>,
    #[serde(rename = "universeId")]
    pub universe_id: u32,
}
#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Stats {
    #[serde(rename = "appliedStats")]
    pub applied_stats: HashMap<StatId, f32>,
    #[serde(rename = "appliedTotal")]
    pub applied_total: f32,
    #[serde(rename = "externalId")]
    pub external_id: String,
    pub id: String,
    #[serde(rename = "proTeamId")]
    pub pro_team_id: ProTeamId,
    #[serde(rename = "scoringPeriodId")]
    pub scoring_period_id: u8,
    #[serde(rename = "seasonId")]
    pub season_id: u32,
    #[serde(rename = "statSourceId")]
    pub stat_source_id: u32,
    #[serde(rename = "statSplitTypeId")]
    pub stat_split_type_id: u32,
    pub stats: HashMap<u16, f32>,
    pub variance: Option<HashMap<u16, f32>>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Ranking {
    #[serde(rename = "auctionValue")]
    pub auction_value: f32,
    pub rank: u32,
    #[serde(rename = "rankSourceId")]
    pub rank_source_id: u32,
    #[serde(rename = "rankType")]
    pub rank_type: String,
    #[serde(rename = "slotId")]
    pub slot_id: u32,
}
