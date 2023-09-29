use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug, Serialize)]
pub struct LeagueInfoResponse {
    #[serde(rename = "gameId")]
    pub game_id: u32,
    pub id: u128,
    pub members: Vec<LeagueMember>,
    #[serde(rename = "segmentId")]
    pub segment_id: i8,
    #[serde(rename = "scoringPeriodId")]
    pub scoring_period_id: i8,
    pub settings: LeagueInfoSettings,
    pub status: LeagueInfoStatus,
    pub teams: Vec<TeamInfo>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct TeamInfo {
    #[serde(rename = "abbrev")]
    pub abbreviation: String,
    pub id: i8,
    pub location: String,
    pub nickname: String,
    pub owners: Vec<String>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct LeagueInfoStatus {
    #[serde(rename = "currentMatchupPeriod")]
    pub current_matchup_period: i8,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "latestScoringPeriod")]
    pub latest_scoring_period: i8,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct LeagueInfoSettings {
    pub name: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct LeagueMember {
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub id: String,
    #[serde(rename = "isLeagueManager")]
    pub is_league_manager: bool,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct LeagueSettingsResponse {
    #[serde(rename = "draftDetail")]
    pub draft_detail: DraftDetail,
    #[serde(rename = "gameId")]
    pub game_id: i32,
    pub id: u128,
    #[serde(rename = "segmentId")]
    pub segment_id: i8,
    #[serde(rename = "scoringPeriodId")]
    pub scoring_period_id: i8,
    pub settings: LeagueSettings,
    pub status: LeagueSettingsStatus,
}
#[derive(Deserialize, Debug, Serialize, Copy, Clone)]
pub struct DraftDetail {
    pub drafted: bool,
    #[serde(rename = "inProgress")]
    pub in_progress: bool,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct LeagueSettings {
    #[serde(rename = "acquisitionSettings")]
    pub acquisition_settings: AcquisitionSettings,
    #[serde(rename = "draftSettings")]
    pub draft_settings: DraftSettings,
    #[serde(rename = "financeSettings")]
    pub finance_settings: FinanceSettings,
    #[serde(rename = "isCustomizable")]
    pub is_customizable: bool,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    pub name: String,
    #[serde(rename = "restrictionType")]
    pub restriction_type: String,
    #[serde(rename = "rosterSettings")]
    roster_settings: RosterSettings,
    #[serde(rename = "scheduleSettings")]
    schedule_settings: ScheduleSettings,
    #[serde(rename = "scoringSettings")]
    pub scoring_settings: ScoringSettings,
    pub size: i8,
    #[serde(rename = "tradeSettings")]
    pub trade_settings: TradeSettings,
}
#[derive(Deserialize, Debug, Serialize)]
pub struct AcquisitionSettings {
    #[serde(rename = "acquisitionBudget")]
    pub acquisition_budget: f32,
    #[serde(rename = "acquisitionLimit")]
    pub acquisition_limits: f32,
    #[serde(rename = "acquisitionType")]
    pub acquisition_type: String,
    #[serde(rename = "isUsingAcquisitionBudget")]
    pub is_using_acquisition_budget: bool,
    #[serde(rename = "matchupAcquisitionLimit")]
    pub matchup_acquisition_limit: f32,
    #[serde(rename = "matchupLimitPerScoringPeriod")]
    pub matchup_limit_per_scoring_period: bool,
    #[serde(rename = "minimumBid")]
    pub minimum_bid: f32,
    #[serde(rename = "waiverHours")]
    pub waiver_hours: i32,
    #[serde(rename = "waiverOrderReset")]
    pub waiver_order_reset: bool,
    #[serde(rename = "waiverProcessDays")]
    pub waiver_process_days: Vec<String>,
    #[serde(rename = "waiverProcessHour")]
    pub waiver_process_hour: i8,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct DraftSettings {
    #[serde(rename = "auctionBudget")]
    pub auction_budget: f32,
    #[serde(rename = "availableDate")]
    pub available_date: u64,
    pub date: u64,
    #[serde(rename = "isTradingEnabled")]
    pub is_trading_enabled: bool,
    #[serde(rename = "keeperCount")]
    pub keeper_count: u8,
    #[serde(rename = "keeperCountFuture")]
    pub keeper_count_future: u8,
    #[serde(rename = "keeperOrderType")]
    pub keeper_order_type: String,
    #[serde(rename = "leagueSubType")]
    pub league_sub_type: String,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(rename = "pickOrder")]
    pub pick_order: Vec<u8>,
    #[serde(rename = "timePerSelection")]
    pub time_per_selection: u16,
    #[serde(rename = "type")]
    pub draft_type: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct FinanceSettings {
    #[serde(rename = "entryFee")]
    pub entry_fee: f32,
    #[serde(rename = "miscFee")]
    pub misc_fee: f32,
    #[serde(rename = "perLoss")]
    pub per_loss: f32,
    #[serde(rename = "perTrade")]
    pub per_trade: f32,
    #[serde(rename = "playerAcquisition")]
    pub player_acquisition: f32,
    #[serde(rename = "playerDrop")]
    pub player_drop: f32,
    #[serde(rename = "playerMoveToActive")]
    pub player_move_to_active: f32,
    #[serde(rename = "playerMoveToIR")]
    pub player_move_to_ir: f32,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct RosterSettings {
    #[serde(rename = "isBenchUnlimited")]
    pub is_bench_unlimited: bool,
    #[serde(rename = "isUsingUndroppableList")]
    pub is_using_undroppable_list: bool,
    #[serde(rename = "lineupLocktimeType")]
    pub lineup_locktime_type: String,
    #[serde(rename = "lineupSlotCounts")]
    pub lineup_slot_counts: HashMap<u8, u8>,
    #[serde(rename = "lineupSlotStatLimits")]
    pub lineup_slot_stat_limits: HashMap<u8, u8>,
    #[serde(rename = "moveLimit")]
    pub move_limit: i32,
    #[serde(rename = "positionLimits")]
    pub position_limits: HashMap<u8, i8>,
    #[serde(rename = "rosterLocktimeType")]
    pub roster_locktime_type: String,
    #[serde(rename = "universeIds")]
    pub universe_ids: Vec<i8>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ScheduleSettings {
    pub divisions: Vec<Divison>,
    #[serde(rename = "matchupPeriodCount")]
    pub matchup_period_count: u8,
    #[serde(rename = "matchupPeriodLength")]
    pub matchup_period_length: u8,
    #[serde(rename = "matchupPeriods")]
    pub matchup_periods: HashMap<u8, Vec<u8>>,
    #[serde(rename = "periodTypeId")]
    pub period_type_id: u8,
    #[serde(rename = "playoffMatchupPeriodLength")]
    pub playoff_matchup_period_length: u8,
    #[serde(rename = "playoffSeedingRule")]
    pub playoff_seeding_rule: String,
    #[serde(rename = "playoffSeedingRuleBy")]
    pub playoff_seeding_rule_by: u8,
    #[serde(rename = "playoffTeamCount")]
    pub playoff_team_count: u8,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Divison {
    pub id: u32,
    pub name: String,
    pub size: u8,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ScoringSettings {
    #[serde(rename = "allowOutOfPositionScoring")]
    pub allow_out_of_position_scoring: bool,
    #[serde(rename = "homeTeamBonus")]
    pub home_team_bonus: f32,
    #[serde(rename = "matchupTieRule")]
    pub matchup_tie_rule: String, //TODO: Enum of possible values; Known: NONE
    #[serde(rename = "matchupTieRuleBy")]
    pub matchup_tie_rule_by: i8,
    #[serde(rename = "playerRankType")]
    pub player_rank_type: String, //TODO: Enum of possible values; Known: PPR
    #[serde(rename = "playoffHomeTeamBonus")]
    pub playoff_home_team_bonus: f32,
    #[serde(rename = "playoffMatchupTieRule")]
    pub playoff_matchup_tie_rule: String, //TODO: Enum of possible values; Known: NONE
    #[serde(rename = "playoffMatchupTieRuleBy")]
    pub playoff_matchup_tie_rule_by: i8,
    #[serde(rename = "scoringEnhancementType")]
    pub scoring_enhancement_type: String, //TODO: Enum of possible values; Known: WIN_BONUS_TOP_HALF
    #[serde(rename = "scoringItems")]
    pub scoring_items: Vec<ScoringItems>,
    #[serde(rename = "scoringType")]
    pub scoring_type: String, //TODO: Enum of possible values; Known: H2H_POINTS
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ScoringItems {
    #[serde(rename = "isReverseItem")]
    pub is_reverse_item: bool,
    #[serde(rename = "leagueRanking")]
    pub league_ranking: f32,
    #[serde(rename = "leagueTotal")]
    pub league_total: f32,
    pub points: f32,
    #[serde(rename = "statId")]
    pub stat_id: i16,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct TradeSettings {
    #[serde(rename = "allowOutOfUniverse")]
    pub allow_out_of_universe: bool,
    #[serde(rename = "deadlineDate")]
    pub deadline_date: u64,
    pub max: i32,
    #[serde(rename = "revisionHours")]
    pub revision_hours: u16,
    #[serde(rename = "vetoVotesRequired")]
    pub veto_votes_required: i8,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct LeagueSettingsStatus {
    #[serde(rename = "activatedDate")]
    pub activated_date: u64,
    #[serde(rename = "createdAsLeagueType")]
    pub created_as_league_type: u8,
    #[serde(rename = "currentLeagueType")]
    pub current_league_type: u8,
    #[serde(rename = "currentMatchupPeriod")]
    pub current_matchup_period: u8,
    #[serde(rename = "finalScoringPeriod")]
    pub final_scoring_period: u8,
    #[serde(rename = "firstScoringPeriod")]
    pub first_scoring_period: u8,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isExpired")]
    pub is_expired: bool,
    #[serde(rename = "isFull")]
    pub is_full: bool,
    #[serde(rename = "isPlayoffMatchupEdited")]
    pub is_playoff_matchup_edited: bool,
    #[serde(rename = "isToBeDeleted")]
    pub is_to_be_deleted: bool,
    #[serde(rename = "isViewable")]
    pub is_viewable: bool,
    #[serde(rename = "isWaiverOrderEdited")]
    pub is_waiver_order_edited: bool,
    #[serde(rename = "latestScoringPeriod")]
    pub latest_scoring_period: u8,
    #[serde(rename = "previousSeasons")]
    pub previous_seasons: Vec<u32>,
    #[serde(rename = "standingsUpdateDate")]
    pub standings_update_date: u64,
    #[serde(rename = "teamsJoined")]
    pub teams_joined: u8,
    #[serde(rename = "transactionScoringPeriod")]
    pub transaction_scoring_period: u8,
    #[serde(rename = "waiverLastExecutionDate")]
    pub waiver_last_execution_date: u64,
    #[serde(rename = "waiverProcessStatus")]
    pub waiver_process_status: HashMap<String, u8>,
}
