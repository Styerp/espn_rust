use crate::id_maps::{PositionId, ProTeamId};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};

use super::id_maps::StatId;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize, Default, Clone, Hash, Copy)]
pub struct PlayerId(pub i64);

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Player {
    pub active: bool,
    #[serde(rename = "defaultPositionId")]
    pub default_position_id: PositionId,
    #[serde(rename = "draftRanksByRankType")]
    pub draft_ranks_by_rank_type: Option<HashMap<String, Ranking>>,
    pub droppable: bool,
    /// The positions a player is eligible to play in.
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
    pub jersey: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "lastNewsDate")]
    pub last_news_date: Option<u64>,
    #[serde(rename = "lastVideoDate")]
    pub last_video_date: Option<u64>,
    pub outlooks: Option<Outlooks>,
    pub ownership: Option<Ownership>,
    #[serde(rename = "proTeamId")]
    pub pro_team_id: ProTeamId,
    pub rankings: Option<HashMap<u8, Vec<Ranking>>>,
    #[serde(rename = "seasonOutlook")]
    pub season_outlook: Option<String>,
    pub stats: Option<Vec<Stats>>,
    #[serde(rename = "universeId")]
    pub universe_id: Option<u32>,
}
#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Stats {
    #[serde(rename = "appliedAverage")]
    pub applied_average: Option<f32>,
    #[serde(rename = "appliedStats")]
    pub applied_stats: Option<HashMap<StatId, f32>>,
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
    pub slot_id: PositionId,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]

pub struct Outlooks {
    #[serde(rename = "outlooksByWeek")]
    pub outlooks_by_week: HashMap<u8, String>,
}
#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Ownership {
    #[serde(rename = "activityLevel")]
    pub activity_level: Option<f32>,
    #[serde(rename = "auctionValueAverage")]
    pub auction_value_average: f32,
    #[serde(rename = "auctionValueAverageChange")]
    pub auction_value_average_change: f32,
    #[serde(rename = "averageDraftPosition")]
    pub average_draft_position: f32,
    #[serde(rename = "averageDraftPositionPercentChange")]
    pub average_draft_position_percent_change: f32,
    pub date: u64,
    #[serde(rename = "leagueType")]
    pub league_type: i8,
    #[serde(rename = "percentChange")]
    pub percent_change: f32,
    #[serde(rename = "percentOwned")]
    pub percent_owned: f32,
    #[serde(rename = "percentStarted")]
    pub percent_started: f32,
}
