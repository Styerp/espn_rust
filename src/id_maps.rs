use phf::phf_map;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Default, Clone, Eq, PartialEq, Hash)]
pub struct PositionId(pub u64);
impl PositionId {
    pub fn to_string(&self) -> &'static str {
        match POSITION_ID_MAP.get(&self.0.to_string()) {
            Some(f) => f,
            None => "Unknown",
        }
    }
}
// Adapted from https://github.com/mkreiser/ESPN-Fantasy-Football-API/blob/main/src/constants.js;
static POSITION_ID_MAP: phf::Map<&'static str, &'static str> = phf_map! {
  "0" => "QB",
  "1" => "TQB",
  "2" => "RB",
  "3" => "RB/WR",
  "4" => "WR",
  "5" => "WR/TE",
  "6" => "TE",
  "7" => "OP",
  "8" => "DT",
  "9" => "DE",
  "10" => "LB",
  "11" => "DL",
  "12" => "CB",
  "13" => "S",
  "14" => "DB",
  "15" => "DP",
  "16" => "D/ST",
  "17" => "K",
  "18" => "P",
  "19" => "HC",
  "20" => "Bench",
  "21" => "IR",
  "22" => "Unknown", // TODO: Figure out what this is
  "23" => "RB/WR/TE",
  "24" => "Unknown" // TODO: Figure out what this is
};

#[derive(Debug, Deserialize, Serialize, Default, Clone, Eq, PartialEq, Hash)]
pub struct ProTeamId(pub i8);
impl ProTeamId {
    pub fn to_name(&self) -> &'static str {
        &self.identifiers().name
    }
    pub fn to_abbreviation(&self) -> &'static str {
        &self.identifiers().abbreviation
    }
    pub fn identifiers(&self) -> &TeamIdentifiers {
        match PRO_TEAM_ID_MAP.get(&self.0.to_string()) {
            Some(f) => f,
            None => &TeamIdentifiers {
                name: "Unknown",
                abbreviation: "UNK",
            },
        }
    }
}
#[derive(Debug, Deserialize, Serialize, Default, Clone, Eq, PartialEq)]
pub struct TeamIdentifiers {
    name: &'static str,
    abbreviation: &'static str,
}

static PRO_TEAM_ID_MAP: phf::Map<&'static str, TeamIdentifiers> = phf_map! {
  "-1" => TeamIdentifiers { name: "Bye", abbreviation: "Bye"},
  "1" => TeamIdentifiers { name: "Atlanta Falcons", abbreviation: "ATL"},
  "2" => TeamIdentifiers { name: "Buffalo Bills", abbreviation:"BUF"},
  "3" => TeamIdentifiers { name: "Chicago Bears", abbreviation: "CHI"},
  "4" => TeamIdentifiers { name: "Cincinnati Bengals", abbreviation:"CIN"},
  "5" => TeamIdentifiers { name: "Cleveland Browns",abbreviation: "CLE"},
  "6" => TeamIdentifiers { name: "Dallas Cowboys",abbreviation:"DAL"},
  "7" => TeamIdentifiers { name: "Denver Broncos",abbreviation:"DEN"},
  "8" => TeamIdentifiers { name: "Detroit Lions", abbreviation:"DET"},
  "9" => TeamIdentifiers { name: "Green Bay Packers", abbreviation:"GB"},
  "10" => TeamIdentifiers { name: "Tennessee Titans", abbreviation:"TEN"},
  "11" => TeamIdentifiers { name: "Indianapolis Colts", abbreviation: "IND"},
  "12" => TeamIdentifiers { name: "Kansas City Chiefs",abbreviation:"KC"},
  "13" => TeamIdentifiers { name: "Las Vegas Raiders",abbreviation:"LV"},
  "14" => TeamIdentifiers { name: "Los Angeles Rams",abbreviation:"LAR"},
  "15" => TeamIdentifiers { name: "Miami Dolphins",abbreviation:"MIA"},
  "16" => TeamIdentifiers { name: "Minnesota Vikings",abbreviation:"MIN"},
  "17" => TeamIdentifiers { name: "New England Patriots",abbreviation:"NE"},
  "18" => TeamIdentifiers { name: "New Orleans Saints",abbreviation:"NO"},
  "19" => TeamIdentifiers { name: "New York Giants",abbreviation:"NYG"},
  "20" => TeamIdentifiers { name: "New York Jets",abbreviation:"NYJ"},
  "21" => TeamIdentifiers { name: "Philadelphia Eagles",abbreviation:"PHI"},
  "22" => TeamIdentifiers { name: "Arizona Cardinals",abbreviation:"ARI"},
  "23" => TeamIdentifiers { name: "Pittsburgh Steelers",abbreviation:"PIT"},
  "24" => TeamIdentifiers { name: "Los Angeles Chargers",abbreviation:"LAC"},
  "25" => TeamIdentifiers { name: "San Francisco 49ers",abbreviation:"SF"},
  "26" => TeamIdentifiers { name: "Seattle Seahawks",abbreviation:"SEA"},
  "27" => TeamIdentifiers { name: "Tampa Bay Buccaneers",abbreviation:"TB"},
  "28" => TeamIdentifiers { name: "Washington Commanders",abbreviation:"WSH"},
  "29" => TeamIdentifiers { name: "Carolina Panthers",abbreviation:"CAR"},
  "30" => TeamIdentifiers { name: "Jacksonville Jaguars",abbreviation:"JAX"},
  "33" => TeamIdentifiers { name: "Baltimore Ravens",abbreviation:"BAL"},
  "34" => TeamIdentifiers { name: "Houston Texans", abbreviation:"HOU"},
};

#[derive(Debug, Deserialize, Serialize, Default, Clone, Eq, PartialEq, Hash)]
pub struct StatId(pub u64);

impl StatId {
    pub fn identifiers(&self) -> &StatIdentifiers {
        match STAT_ID_MAP.get(&self.0.to_string()) {
            Some(ids) => ids,
            None => &StatIdentifiers {
                name: "Unknown",
                field_name: "unknown", //format!("unknown_{}", &self.0.to_string()).as_str(),
            },
        }
    }
    pub fn to_name(&self) -> &'static str {
        &self.identifiers().name
    }
    pub fn to_field_name(&self) -> &'static str {
        &self.identifiers().field_name
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, Eq, PartialEq)]
pub struct StatIdentifiers {
    name: &'static str,
    field_name: &'static str,
}

// Adapted from https://github.com/mkreiser/ESPN-Fantasy-Football-API/blob/main/src/player-stats/player-stats.js
// use the `find_unmapped_stats` example to locate stats to add from your league for a given week
static STAT_ID_MAP: phf::Map<&'static str, StatIdentifiers> = phf_map! {
    "0" => StatIdentifiers {name: "Pass Attempts", field_name: "pass_attempts"},
    "1" => StatIdentifiers {name: "Completed Passes", field_name: "passing_completions"},
    "2" => StatIdentifiers {name: "Incomplete Passes", field_name: "passing_incompletions"},
    "3" => StatIdentifiers {name: "Passing Yards", field_name: "passing_yards"},
    "4" => StatIdentifiers {name: "Passing Touchdowns", field_name:"passing_touchdowns"},
    "5" => StatIdentifiers {name: "Every 5 Passing Yards", field_name: "passing_yards_each_5"},
    "6" => StatIdentifiers {name: "Every 10 Passing Yards", field_name: "passing_yards_each_10"},
    "7" => StatIdentifiers {name: "Every 20 Passing Yards", field_name: "passing_yards_each_20"},
    "8" => StatIdentifiers {name: "Every 25 Passing Yards", field_name: "passing_yards_each_25"},
    "9" => StatIdentifiers {name: "Every 50 Passing Yards", field_name: "passing_yards_each_50"},
    "10" => StatIdentifiers {name: "Every 100 Passing Yards", field_name: "passing_yards_each_100"},
    "11" => StatIdentifiers {name: "Every 5 Passing Completions", field_name: "passing_completions_each_5"},
    "12" => StatIdentifiers {name: "Every 10 Passing Completions", field_name: "passing_completions_each_10"},
    "13" => StatIdentifiers {name: "Every 5 Passing Incompletions", field_name: "passing_incompletions_each_5"},
    "14" => StatIdentifiers {name: "Every 10 Passing Incompletions", field_name: "passing_incompletions_each_10"},
    "15" => StatIdentifiers {name: "40+ Yard TD Pass Bonus", field_name: "passing_touchdown_40_plus_bonus"},
    "16" => StatIdentifiers {name: "50+ Yard TD Pass Bonus", field_name: "passing_touchdown_50_plus_bonus"},
    "17" => StatIdentifiers {name: "300-399 yard passing game", field_name: "passing_300_to_399_yard_game"},
    "18" => StatIdentifiers {name: "400+ yard passing game", field_name: "passing_over_400_yards"},
    "19" => StatIdentifiers {name: "Passing 2 Point Conversions", field_name: "passing_two_point_conversions"},
    "20" => StatIdentifiers {name: "Passing Interceptions", field_name: "passing_interceptions"},
    
    "23" => StatIdentifiers {name: "Rushing Attempts", field_name: "rushing_attempts"},
    "24" => StatIdentifiers {name: "Rushing Yards", field_name: "rushing_yards"},
    "25" => StatIdentifiers {name: "Rushing Touchdowns", field_name: "rushing_touchdowns"},
    "26" => StatIdentifiers {name: "Rushing 2 Point Conversions", field_name:"rushing_two_point_conversions"},
    "27" => StatIdentifiers {name: "Every 5 Rushing Yards", field_name: "rushing_yards_each_5"},
    "28" => StatIdentifiers {name: "Every 10 Rushing Yards", field_name: "rushing_yards_each_10"},
    "29" => StatIdentifiers {name: "Every 20 Rushing Yards", field_name: "rushing_yards_each_20"},
    "30" => StatIdentifiers {name: "Every 25 Rushing Yards", field_name: "rushing_yards_each_25"},
    "31" => StatIdentifiers {name: "Every 50 Rushing Yards", field_name: "rushing_yards_each_50"},
    "32" => StatIdentifiers {name: "Every 100 Rushing Yards", field_name: "rushing_yards_each_100"},
    "33" => StatIdentifiers {name: "Every 5 Rush Attempts", field_name: "rushing_attempts_each_5"},
    "34" => StatIdentifiers {name: "Every 10 Rush Attempts", field_name: "rushing_attempts_each_10"},
    "35" => StatIdentifiers {name: "40+ Yard TD Rush Bonus", field_name: "rushing_touchdown_40_plus_bonus"},
    "36" => StatIdentifiers {name: "50+ Yard TD Rush Bonus", field_name: "rushing_touchdown_50_plus_bonus"},
    "37" => StatIdentifiers {name: "100-199 Yard Rushing Game", field_name: "rushing_100_to_199_yards"},
    "38" => StatIdentifiers {name: "200+ Yard Rushing Game", field_name: "rushing_over_200_yards"},
    
    "42" => StatIdentifiers {name: "Receiving Yards", field_name:"receiving_yards"},
    "43" => StatIdentifiers {name: "Receiving Touchdowns", field_name: "receiving_touchdowns"},
    "44" => StatIdentifiers {name: "Receiving 2 Point Conversions", field_name: "receiving_two_point_conversions"},
    "45" => StatIdentifiers {name: "40+ Yard TD Receiving Bonus", field_name: "receiving_touchdown_40_plus_bonus"},
    "46" => StatIdentifiers {name: "50+ Yard TD Receiving Bonus", field_name: "receiving_touchdown_50_plus_bonus"},
    "47" => StatIdentifiers {name: "Every 5 Receiving Yards", field_name: "receiving_yards_each_5"},
    "48" => StatIdentifiers {name: "Every 10 Receiving Yards", field_name: "receiving_yards_each_10"},
    "49" => StatIdentifiers {name: "Every 20 Receiving Yards", field_name: "receiving_yards_each_20"},
    "50" => StatIdentifiers {name: "Every 25 Receiving Yards", field_name: "receiving_yards_each_25"},
    "51" => StatIdentifiers {name: "Every 50 Receiving Yards", field_name: "receiving_yards_each_50"},
    "52" => StatIdentifiers {name: "Every 100 Receiving Yards", field_name: "receiving_yards_each_100"},
    "53" => StatIdentifiers {name: "Receptions", field_name: "receptions"},
    "54" => StatIdentifiers {name: "Every 5 Receptions", field_name: "receptions_each_5"},
    "55" => StatIdentifiers {name: "Every 10 Receptions", field_name: "receptions_each_10"},

    "56" => StatIdentifiers {name: "100-199 Yard Receiving Game", field_name: "receiving_100_to_199_yards"},
    "57" => StatIdentifiers {name: "200+ Yard Receiving Game", field_name: "receiving_over_200_yards"},
    "58" => StatIdentifiers {name: "Receiving Targets", field_name: "receiving_targets"},
    
    "63" => StatIdentifiers {name: "Fumble Recovered for Touchdown", field_name:"offensive_fumble_recovered_for_touchdown"},
    "64" => StatIdentifiers {name: "Sacked", field_name: "sacked"},

    "68" => StatIdentifiers {name: "Total Fumbles", field_name: "total_fumbles"},
    
    "72" => StatIdentifiers {name: "Fumbles Lost", field_name: "fumbles_lost"},

    "74" => StatIdentifiers {name: "Field Goals Made From 50+ Yards", field_name: "field_goals_made_50_plus"},
    "77" => StatIdentifiers {name: "Field Goals Made From Between 40 and 49 Yards", field_name: "field_goals_made_40_to_49"},
    "78" => StatIdentifiers {name: "Field Goals Attempted From 40 to 49 Yards", field_name: "field_goals_attempted_40_to_49"},
    "79" => StatIdentifiers {name: "Field Goals Missed From 40 to 49 Yards", field_name: "field_goals_missed_40_to_49"},
    "80" => StatIdentifiers {name: "Field Goals Made From <40+ Yards", field_name: "field_goals_made_under_40"},
    "81" => StatIdentifiers {name: "Field Goals Attempted From <40+ Yards", field_name: "field_goals_attempted_under_40"},
    "82" => StatIdentifiers {name: "Field Goals Missed From <40+ Yards", field_name: "field_goals_missed_under_40"},
    "83" => StatIdentifiers {name: "Field Goals Made", field_name: "field_goals_made"},
    "84" => StatIdentifiers {name: "Field Goals Attempted", field_name: "field_goals_attempted"},
    "85" => StatIdentifiers {name: "Field Goals Missed", field_name: "field_goals_missed"},
    "86" => StatIdentifiers {name: "Extra Points Made", field_name: "extra_points_made"},
    "87" => StatIdentifiers {name: "Extra Points Attempted", field_name: "extra_points_attempted"},
    "88" => StatIdentifiers {name: "Extra Points Missed", field_name: "extra_points_missed"},

    "89" => StatIdentifiers {name: "Defense Allowed 0 Points", field_name: "defense_0_points_allowed"},
    "90" => StatIdentifiers {name: "Defense Allowed 1 to 6 Points", field_name: "defense_1_to_6_points_allowed"},
    "91" => StatIdentifiers {name: "Defense Allowed 7 to 13 Points", field_name: "defense_7_to_13_points_allowed"},
    "92" => StatIdentifiers {name: "Defense Allowed 14 to 17 Points", field_name: "defense_14_to_17_points_allowed"},
    "93" => StatIdentifiers {name: "Defense Blocked Kicks for Touchdowns", field_name: "defense_blocked_kick_for_touchdowns"},
    "94" => StatIdentifiers {name: "Fumble or INT Return for Touchdown", field_name: "defensive_fumble_or_int_return_for_touchdown"},
    "95" => StatIdentifiers {name: "Defensive Interceptions", field_name: "defensive_interceptions"},
    "96" => StatIdentifiers {name: "Defensive Fumbles Recovered", field_name: "defensive_fumbles_recovered"},
    "97" => StatIdentifiers {name: "Defensive Blocked Kicks", field_name: "defensive_blocked_kicks"},
    "98" => StatIdentifiers {name: "Defensive Safeties", field_name: "defensive_safeties"},
    "99" => StatIdentifiers {name: "Defensive Sacks", field_name: "defensive_sacks"},
    "100" => StatIdentifiers {name: "Defensive Half Sacks", field_name: "defensive_half_sacks"},

    "101" => StatIdentifiers {name: "Kickoffs Returned for Touchdown", field_name: "kickoff_return_touchdown"},
    "102" => StatIdentifiers {name: "Punts Returned for Touchdown", field_name: "punt_return_touchdown"},
    "103" => StatIdentifiers {name: "Fumbles Returned for Touchdown", field_name: "fumble_return_touchdown"},
    "104" => StatIdentifiers {name: "Interceptions Returned for Touchdown", field_name: "interception_return_touchdown"},
    "106" => StatIdentifiers {name: "Forced Fumbles", field_name: "forced_fumbles"},
    "107" => StatIdentifiers {name: "Assisted Tackles", field_name: "assisted_tackles"},
    "108" => StatIdentifiers {name: "Solo Tackles", field_name: "solo_tackles"},
    "109" => StatIdentifiers {name: "Total Tackles", field_name: "total_tackles"},
    "110" => StatIdentifiers {name: "Every 3 Tackles", field_name: "tackles_each_3"},
    "111" => StatIdentifiers {name: "Every 5 Tackles", field_name: "tackles_each_5"},
    "112" => StatIdentifiers {name: "Stuffs", field_name: "stuffs"},
    "113" => StatIdentifiers {name: "Passes Defended", field_name: "passes_defended"},
    "114" => StatIdentifiers {name: "Kickoff Return Yards", field_name: "kickoff_return_yards"},
    "115" => StatIdentifiers {name: "Punt Return Yards", field_name: "punt_return_yards"},
    "116" => StatIdentifiers {name: "Every 10 Kickoff Return Yards", field_name: "kickoff_return_yards_each_10"},
    "117" => StatIdentifiers {name: "Every 25 Kickoff Return Yards", field_name: "kickoff_return_yards_each_25"},
    "118" => StatIdentifiers {name: "Every 10 Punt Return Yards", field_name: "punt_return_yards_each_10"},
    "119" => StatIdentifiers {name: "Every 25 Punt Return Yards", field_name: "punt_return_yards_each_25"},
    "120" => StatIdentifiers {name: "Points Allowed", field_name: "points_allowed"},
    "121" => StatIdentifiers {name: "Defense Allowed 18 to 21 Points", field_name: "defense_18_to_21_points_allowed"},
    "122" => StatIdentifiers {name: "Defense Allowed 22 to 27 Points", field_name: "defense_22_to_27_points_allowed"},
    "123" => StatIdentifiers {name: "Defense Allowed 28 to 34 Points", field_name: "defense_28_to_34_points_allowed"},
    "124" => StatIdentifiers {name: "Defense Allowed 35 to 45 Points", field_name: "defense_35_to_45_points_allowed"},
    "125" => StatIdentifiers {name: "Defense Allowed 46+ Points", field_name: "defense_46_plus_points_allowed"},
    "128" => StatIdentifiers {name: "Defensive Sacks", field_name: "defensive_sacks"},
    "129" => StatIdentifiers {name: "Defense Allowed 100 to 199 Yards", field_name: "defense_100_to_199_yards_allowed"},
    "130" => StatIdentifiers {name: "Defense Allowed 200 to 299 Yards", field_name: "defense_200_to_299_yards_allowed"},
    "131" => StatIdentifiers {name: "Defense Allowed 300 to 349 Yards", field_name: "defense_300_to_349_yards_allowed"},
    "132" => StatIdentifiers {name: "Defense Allowed 350 to 399 Yards", field_name: "defense_350_to_399_yards_allowed"},
    "133" => StatIdentifiers {name: "Defense Allowed 400 to 449 Yards", field_name: "defense_400_to_449_yards_allowed"},
    "134" => StatIdentifiers {name: "Defense Allowed 450 to 499 Yards", field_name: "defense_450_to_499_yards_allowed"},
    "135" => StatIdentifiers {name: "Defense Allowed 500 to 549 Yards", field_name: "defense_500_to_549_yards_allowed"},
    "136" => StatIdentifiers {name: "Defense Allowed More than 550 Yards", field_name: "defense_over_550_yards_allowed"},
    "138" => StatIdentifiers {name: "Net Punts", field_name: "punts_net"},
    "139" => StatIdentifiers {name: "Punt Yards", field_name: "punt_yards"},
    "140" => StatIdentifiers {name: "Punts Inside the 10", field_name: "punts_inside_10"},
    "141" => StatIdentifiers {name: "Punts Inside the 20", field_name: "punts_inside_20"},
    "142" => StatIdentifiers {name: "Punts Blocked", field_name: "punts_blocked"},
    "143" => StatIdentifiers {name: "Punts Returned", field_name: "punts_returned"},
    "144" => StatIdentifiers {name: "Punt Return Yards", field_name: "punt_return_yards"},
    "145" => StatIdentifiers {name: "Punt Touchbacks", field_name: "punts_touchback"},
    "146" => StatIdentifiers {name: "Punts Fair Caught", field_name: "punts_fair_caught"},
    
    "148" => StatIdentifiers {name: "Punt Average 44+", field_name: "punt_average_over_44"},
    "149" => StatIdentifiers {name: "Punt Average 42.0-43.9", field_name: "punt_average_42_to_44"},
    "150" => StatIdentifiers {name: "Punt Average 40.0-41.9", field_name: "punt_average_40_to_42"},
    "151" => StatIdentifiers {name: "Punt Average 38.0-39.9", field_name: "punt_average_38_to_40"},
    "152" => StatIdentifiers {name: "Punt Average 36.0-37.9", field_name: "punt_average_36_to_38"},
    "153" => StatIdentifiers {name: "Punt Average 34.0-35.9", field_name: "punt_average_34_to_36"},
    "154" => StatIdentifiers {name: "Punt Average 33.9 or less", field_name: "punt_average_below_34"},
    "155" => StatIdentifiers {name: "Team Win", field_name: "team_win"},
    "156" => StatIdentifiers {name: "Team Loss", field_name: "team_loss"},
    "158" => StatIdentifiers {name: "Team Points Scored", field_name: "team_points_scored"},
    "161" => StatIdentifiers {name: "Win Margin 25+", field_name: "win_margin_25_plus"},
    "162" => StatIdentifiers {name: "Win Margin 20-24", field_name: "win_margin_20_to_24"},
    "163" => StatIdentifiers {name: "Win Margin 15-19", field_name: "win_margin_15_to_19"},
    "164" => StatIdentifiers {name: "Win Margin 10-14", field_name: "win_margin_10_to_14"},
    "165" => StatIdentifiers {name: "Win Margin 5-9", field_name: "win_margin_5_to_9"},
    "166" => StatIdentifiers {name: "Win Margin 1-4", field_name: "win_margin_1_to_4"},
    "167" => StatIdentifiers {name: "Loss Margin 1-4", field_name: "loss_margin_1_to_4"},
    "168" => StatIdentifiers {name: "Loss Margin 5-9", field_name: "loss_margin_5_to_9"},
    "169" => StatIdentifiers {name: "Loss Margin 10-14", field_name: "loss_margin_10_to_14"},
    "170" => StatIdentifiers {name: "Loss Margin 15-19", field_name: "loss_margin_15_to_19"},
    "171" => StatIdentifiers {name: "Loss Margin 20-24", field_name: "loss_margin_20_to_24"},
    "172" => StatIdentifiers {name: "Loss Margin 25+", field_name: "loss_margin_25_plus"},
    

    "198" => StatIdentifiers {name: "Field Goals Made From 50 to 59 Yards", field_name: "field_goals_made_50_to_59"},
    "199" => StatIdentifiers {name: "Field Goals Attempted From 50 to 59 Yards", field_name: "field_goals_attempted_50_to_59"},
    "200" => StatIdentifiers {name: "Field Goals Missed From 50 to 59 Yards", field_name: "field_goals_missed_50_to_59"},
    "201" => StatIdentifiers {name: "Field Goals Made From 60+ Yards", field_name: "field_goals_made_60_plus"},
    "202" => StatIdentifiers {name: "Field Goals Attempted From 60+ Yards", field_name: "field_goals_attempted_60_plus"},
    "203" => StatIdentifiers {name: "Field Goals Missed From 60+ Yards", field_name: "field_goals_missed_60_plus"},

    "211" => StatIdentifiers {name: "Passing First Downs", field_name: "passing_first_downs"},
    "212" => StatIdentifiers {name: "Rushing First Downs", field_name: "rushing_first_downs"},
    "213" => StatIdentifiers {name: "Receiving First Downs", field_name: "receiving_first_downs"},
    "214" => StatIdentifiers {name: "FG Made Yards", field_name: "field_goal_made_yards"},
    "218" => StatIdentifiers {name: "Every 10 FG Made Yards", field_name: "field_goal_made_yards_each_10"},
    "219" => StatIdentifiers {name: "Every 20 FG Made Yards", field_name: "field_goal_made_yards_each_20"},
    "221" => StatIdentifiers {name: "Every 50 FG Made Yards", field_name: "field_goal_made_yards_each_50"},
};

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn position_id_converts_to_string() {
        assert_eq!(PositionId(0u64).to_string(), "QB");
        assert_eq!(PositionId(16u64).to_string(), "D/ST");
        assert_eq!(PositionId(24u64).to_string(), "Unknown");
    }
    #[test]
    fn pro_team_id_converts_to_string() {
        assert_eq!(ProTeamId(0i8).to_name(), "Unknown?");
        assert_eq!(ProTeamId(16i8).to_name(), "Minnesota Vikings");
        assert_eq!(ProTeamId(24i8).to_name(), "Los Angeles Chargers");
    }
    #[test]
    fn stat_converts() {
        let stat = StatId(72u64);
        assert_eq!(stat.to_name(), "Fumbles Lost");
        assert_eq!(stat.to_field_name(), "fumbles_lost");
        assert_eq!(StatId(1u64).to_name(), "Unknown")
    }
}
