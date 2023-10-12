use std::{collections::HashMap, fmt};

use serde::{Deserialize, Serialize};

use crate::client::EspnClient;

use super::{id_maps::StatId, league::DraftDetail, members::MemberId, player::PlayerId};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TeamResponse {
    #[serde(rename = "draftDetail")]
    pub draft_detail: Option<DraftDetail>,
    #[serde(rename = "gameId")]
    pub game_id: i32,
    pub id: u128,
    #[serde(rename = "segmentId")]
    pub segment_id: i8,
    #[serde(rename = "scoringPeriodId")]
    pub scoring_period_id: i8,
    pub teams: Vec<Team>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Team {
    pub abbrev: String,
    #[serde(rename = "currentProjectedRank")]
    pub current_projected_rank: u8,
    #[serde(rename = "divisionId")]
    pub division_id: u8,
    #[serde(rename = "draftDayProjectedRank")]
    pub draft_day_projected_rank: u8,
    #[serde(rename = "draftStrategy")]
    pub draft_strategy: DraftStrategy,
    pub id: TeamId,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    pub location: String,
    pub logo: String,
    #[serde(rename = "logoType")]
    pub logo_type: String,
    pub name: String,
    pub nickname: String,
    pub owners: Vec<MemberId>,
    #[serde(rename = "pendingTransactions")]
    pub pending_transactions: Option<Vec<PendingTransaction>>,
    #[serde(rename = "playoffSeed")]
    pub playoff_seed: u8,
    pub points: f32,
    #[serde(rename = "pointsAdjusted")]
    pub points_adjusted: f32,
    #[serde(rename = "pointsDelta")]
    pub points_delta: f32,
    #[serde(rename = "primaryOwner")]
    pub primary_owner: MemberId,
    #[serde(rename = "rankCalculatedFinal")]
    pub rank_calculated_final: u8,
    #[serde(rename = "rankFinal")]
    pub rank_final: u8,
    pub record: RecordLocales,
    #[serde(rename = "tradeBlock")]
    pub trade_block: Option<TradeBlock>,
    #[serde(rename = "transactionCounter")]
    pub transaction_counter: TransactionCounter,
    #[serde(rename = "valuesByStat")]
    pub values_by_stat: HashMap<StatId, f32>,
}

///TODO: What can't you trade?
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TradeBlock {
    pub players: Option<HashMap<PlayerId, String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PendingTransaction {
    #[serde(rename = "acceptedDate")]
    pub accepted_date: Option<u128>,
    #[serde(rename = "bidAmount")]
    pub bid_amount: i32,
    #[serde(rename = "executionType")]
    pub execution_type: String,
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<u128>,
    pub id: String,
    #[serde(rename = "isActingAsTeamOwner")]
    pub is_acting_as_team_owner: bool,
    #[serde(rename = "isLeagueManager")]
    pub is_league_manager: bool,
    #[serde(rename = "isPending")]
    pub is_pending: bool,
    pub items: Vec<TransactionItem>,
    #[serde(rename = "memberId")]
    pub member_id: MemberId,
    #[serde(rename = "processDate")]
    pub process_date: u128,
    #[serde(rename = "proposedDate")]
    pub proposed_date: u128,
    pub rating: u32,
    #[serde(rename = "scoringPeriodId")]
    pub scoring_period_id: u8,
    #[serde(rename = "skipTransactionCounters")]
    pub skip_transaction_counters: bool,
    pub status: String,
    #[serde(rename = "subOrder")]
    pub sub_order: u16,
    #[serde(rename = "teamActions")]
    pub team_actions: Option<HashMap<TeamId, String>>,
    #[serde(rename = "teamId")]
    pub team_id: TeamId,
    #[serde(rename = "type")]
    pub transaction_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransactionItem {
    #[serde(rename = "fromLineupSlotId")]
    pub from_lineup_slot_id: i8,
    #[serde(rename = "fromTeamId")]
    pub from_team_id: TeamId,
    #[serde(rename = "isKeeper")]
    pub is_keeper: bool,
    #[serde(rename = "overallPickNumber")]
    pub overall_pick_number: u16,
    #[serde(rename = "playerId")]
    pub player_id: PlayerId,
    #[serde(rename = "toLineupSlotId")]
    pub to_lineup_slot_id: i8,
    #[serde(rename = "toTeamId")]
    pub to_team_id: TeamId,
    #[serde(rename = "type")]
    pub trade_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RecordLocales {
    away: Record,
    division: Record,
    home: Record,
    overall: Record,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Record {
    #[serde(rename = "gamesBack")]
    pub games_back: f32,
    pub losses: u8,
    pub percentage: f32,
    #[serde(rename = "pointsAgainst")]
    pub points_against: f32,
    #[serde(rename = "pointsFor")]
    pub points_for: f32,
    #[serde(rename = "streakLength")]
    pub streak_length: u8,
    #[serde(rename = "streakType")]
    pub streak_type: String,
    pub ties: u8,
    pub wins: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DraftStrategy {
    #[serde(rename = "excludedPlayerIds")]
    pub excluded_player_ids: Option<Vec<PlayerId>>,
    #[serde(rename = "keeperPlayerIds")]
    pub keeper_player_ids: Option<Vec<PlayerId>>,
}

/// A fantasy football team in the league
#[derive(Debug, Deserialize, Serialize, Clone, Copy, Eq, PartialEq, Hash)]
pub struct TeamId(pub u8);
impl TeamId {
    pub async fn get_details(&self, client: &EspnClient, season: i16) -> Team {
       match client.get_team_data(season).await.iter().find(|x| &x.id == self) {
        Some(t)=> t.clone(),
        None => panic!("No data for team")
       }
    }
}
impl fmt::Display for TeamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransactionCounter {
    #[serde(rename = "acquisitionBudgetSpent")]
    pub acquisition_budget_spent: f32,
    pub acquisitions: u32,
    pub drops: u32,
    #[serde(rename = "matchupAcquisitionTotals")]
    pub matchup_acquisition_totals: HashMap<u32, u32>,
    pub misc: u32,
    #[serde(rename = "moveToActive")]
    pub move_to_active: u32,
    #[serde(rename = "moveToIR")]
    pub move_to_ir: u32,
    pub paid: f32,
    #[serde(rename = "teamCharges")]
    pub team_charges: f32,
    pub trades: u32,
}
