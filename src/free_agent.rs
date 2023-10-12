use std::collections::HashMap;

use crate::team::TeamId;

use super::id_maps::{PositionId, ProTeamId};
use super::player::{Player, PlayerId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FreeAgentResponse {
    pub players: Vec<FreeAgent>,
    #[serde(rename = "positionAgainstOpponent")]
    pub position_against_opponent: PositionalRatings,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FreeAgent {
    #[serde(rename = "draftAuctionValue")]
    pub draft_auction_value: f32,
    pub id: PlayerId,
    #[serde(rename = "keeperValue")]
    pub keeper_value: f32,
    #[serde(rename = "keeperValueFuture")]
    pub keeper_value_future: f32,
    #[serde(rename = "lineupLocked")]
    pub lineup_locked: bool,
    #[serde(rename = "onTeamId")]
    pub on_team_id: TeamId,
    pub player: Player,
    pub ratings: HashMap<u8, FreeAgentRatings>,
    pub status: String,
    #[serde(rename = "tradeLocked")]
    pub trade_locked: bool,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PositionalRatings {
    #[serde(rename = "positionalRatings")]
    pub positional_ratings: HashMap<PositionId, Ratings>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Ratings {
    pub average: f32,
    #[serde(rename = "ratingsByOpponent")]
    pub rating_by_opponent: HashMap<ProTeamId, Rating>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Rating {
    pub average: f32,
    pub rank: u32,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FreeAgentRatings {
    #[serde(rename = "positionalRanking")]
    pub positional_ranking: u32,
    #[serde(rename = "totalRanking")]
    pub total_ranking: u32,
    #[serde(rename = "totalRating")]
    pub total_rating: f32,
}
