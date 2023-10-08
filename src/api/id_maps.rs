use phf::phf_map;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Default, Clone, Eq, PartialEq, Hash)]
pub struct PositionId(u64);
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

#[derive(Debug, Deserialize, Serialize, Default, Clone, Eq, PartialEq)]
pub struct ProTeamId(i8);
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
  "-1" => TeamIdentifiers{ name: "Bye", abbreviation: "Bye"},
  "1" => TeamIdentifiers{ name: "Atlanta Falcons", abbreviation: "ATL"},
  "2" => TeamIdentifiers{ name: "Buffalo Bills", abbreviation:"BUF"},
  "3" => TeamIdentifiers{ name: "Chicago Bears", abbreviation: "CHI"},
  "4" => TeamIdentifiers{ name: "Cincinnati Bengals", abbreviation:"CIN"},
  "5" => TeamIdentifiers{ name: "Cleveland Browns",abbreviation: "CLE"},
  "6" => TeamIdentifiers{ name: "Dallas Cowboys",abbreviation:"DAL"},
  "7" => TeamIdentifiers{ name: "Denver Broncos",abbreviation:"DEN"},
  "8" => TeamIdentifiers{ name: "Detroit Lions", abbreviation:"DET"},
  "9" => TeamIdentifiers{ name: "Green Bay Packers", abbreviation:"GB"},
  "10"=> TeamIdentifiers{ name: "Tennessee Titans", abbreviation:"TEN"},
  "11"=> TeamIdentifiers{ name: "Indianapolis Colts", abbreviation: "IND"},
  "12"=> TeamIdentifiers{ name: "Kansas City Chiefs",abbreviation:"KC"},
  "13"=> TeamIdentifiers{ name: "Las Vegas Raiders",abbreviation:"LV"},
  "14"=> TeamIdentifiers{ name: "Los Angeles Rams",abbreviation:"LAR"},
  "15"=> TeamIdentifiers{ name: "Miami Dolphins",abbreviation:"MIA"},
  "16"=> TeamIdentifiers{ name: "Minnesota Vikings",abbreviation:"MIN"},
  "17"=> TeamIdentifiers{ name: "New England Patriots",abbreviation:"NE"},
  "18"=> TeamIdentifiers{ name: "New Orleans Saints",abbreviation:"NO"},
  "19"=> TeamIdentifiers{ name: "New York Giants",abbreviation:"NYG"},
  "20"=> TeamIdentifiers{ name: "New York Jets",abbreviation:"NYJ"},
  "21"=> TeamIdentifiers{ name: "Philadelphia Eagles",abbreviation:"PHI"},
  "22"=> TeamIdentifiers{ name: "Arizona Cardinals",abbreviation:"ARI"},
  "23"=> TeamIdentifiers{ name: "Pittsburgh Steelers",abbreviation:"PIT"},
  "24"=> TeamIdentifiers{ name: "Los Angeles Chargers",abbreviation:"LAC"},
  "25"=> TeamIdentifiers{ name: "San Francisco 49ers",abbreviation:"SF"},
  "26"=> TeamIdentifiers{ name: "Seattle Seahawks",abbreviation:"SEA"},
  "27"=> TeamIdentifiers{ name: "Tampa Bay Buccaneers",abbreviation:"TB"},
  "28"=> TeamIdentifiers{ name: "Washington Commanders",abbreviation:"WSH"},
  "29"=> TeamIdentifiers{ name: "Carolina Panthers",abbreviation:"CAR"},
  "30"=> TeamIdentifiers{ name: "Jacksonville Jaguars",abbreviation:"JAX"},
  "33"=> TeamIdentifiers{ name: "Baltimore Ravens",abbreviation:"BAL"},
  "34"=> TeamIdentifiers{ name: "Houston Texans", abbreviation:"HOU"},
};

#[derive(Debug, Deserialize, Serialize, Default, Clone, Eq, PartialEq, Hash)]
pub struct StatId(u64);

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
static STAT_ID_MAP: phf::Map<&'static str, StatIdentifiers> = phf_map! {
    "3"=> StatIdentifiers{name: "Passing Yards", field_name: "passing_yards"},
    "4"=> StatIdentifiers{name: "Passing Touchdowns", field_name:"passing_touchdowns"},
    "17" => StatIdentifiers{name: "300-399 yard passing game", field_name: "passing_300_to_399_yard_game"},
    "18" => StatIdentifiers{name: "400+ yard passing game", field_name: "passing_over_400_yards"},
    "19" => StatIdentifiers{name: "Passing 2 Point Conversions", field_name: "passing_two_point_conversions"},
    "20" => StatIdentifiers{name: "Passing Interceptions", field_name: "passing_interceptions"},

    "24" => StatIdentifiers{name: "Rushing Yards", field_name: "rushing_yards"},
    "25" => StatIdentifiers{name: "Rushing Touchdowns", field_name: "rushing_touchdowns"},
    "26" => StatIdentifiers{name: "Rushing 2 Point Conversions", field_name:"rushing_two_point_conversions"},
    "37" => StatIdentifiers{name: "100-199 Yard Rushing Game", field_name: "rushing_100_to_199_yards"},
    "38" => StatIdentifiers{name: "200+ Yard Rushing Game", field_name: "rushing_over_200_yards"},
    "42" => StatIdentifiers{name: "Receiving Yards", field_name:"receiving_yards"},
    "43" => StatIdentifiers{name: "Receiving Touchdowns", field_name: "receiving_touchdowns"},
    "44" => StatIdentifiers{name: "Receiving 2 Point Conversions", field_name: "receiving_two_point_conversions"},
    "53" => StatIdentifiers{name: "Receptions", field_name: "receptions"},

    "56" => StatIdentifiers {name: "100-199 Yard Receiving Game", field_name: "receiving_100_to_199_yards"},
    "57" => StatIdentifiers {name: "200+ Yard Receiving Game", field_name: "receiving_over_200_yards"},
    "63"=> StatIdentifiers{name: "Fumble Recovered for Touchdown", field_name:"offensive_fumble_recovered_for_touchdown"},

    "72" =>StatIdentifiers {name: "Fumbles Lost", field_name: "fumbles_lost"},

    "74" =>StatIdentifiers {name: "Field Goals Made From 50+ Yards", field_name: "field_goals_made_50_plus"},
    "77" => StatIdentifiers{name: "Field Goals Made From Between 40 and 49 Yards", field_name: "field_goals_made_40_to_49"},
    "80" => StatIdentifiers{name: "Field Goals Made From <40+ Yards", field_name: "field_goals_made_under_40"},
    "85" => StatIdentifiers{name: "Field Goals Missed", field_name: "field_goals_missed"},
    "86" => StatIdentifiers{name: "Extra Points Made", field_name: "extra_points_made"},
    "88" => StatIdentifiers{name: "Extra Points Missed", field_name: "extra_points_missed"},

    "89" => StatIdentifiers{ name: "Defense Allowed 0 Points", field_name: "defense_0_points_allowed"},
    "90" => StatIdentifiers{name: "Defense Allowed 1 to 6 Points", field_name: "defense_1_to_6_points_allowed"},
    "91" => StatIdentifiers{name: "Defense Allowed 7 to 13 Points", field_name: "defense_7_to_13_points_allowed"},
    "92" => StatIdentifiers{name: "Defense Allowed 14 to 17 Points", field_name: "defense_14_to_17_points_allowed"},

    "93" => StatIdentifiers {name: "Defense Blocked Kicks for Touchdowns", field_name: "defense_blocked_kick_for_touchdowns"},
    "95"=> StatIdentifiers {name: "Defensive Interceptions", field_name: "defensive_interceptions"},
    "96"=> StatIdentifiers {name: "Defensive Fumbles Recovered", field_name: "defensive_fumbles_recovered"},
    "97"=> StatIdentifiers {name: "Defensive Blocked Kicks", field_name: "defensive_blocked_kicks"},
    "98"=> StatIdentifiers {name: "Defensive Safeties", field_name: "defensive_safeties"},
    "99"=> StatIdentifiers {name: "Defensive Sacks", field_name: "defensive_sacks"},

    "101"=>StatIdentifiers{name: "Kickoffs Returned for Touchdown", field_name: "kickoff_return_touchdown"},
    "102" => StatIdentifiers{name:"Punts Returned for Touchdown", field_name: "punt_return_touchdown"},
    "103"=> StatIdentifiers{name: "Fumbles Returned for Touchdown", field_name: "fumble_return_touchdown"},
    "104"=> StatIdentifiers{name: "Interceptions Returned for Touchdown", field_name: "interception_return_touchdown"},

    "123" => StatIdentifiers{name: "Defense Allowed 28 to 34 Points", field_name: "defense_28_to_34_points_allowed"},
    "124" => StatIdentifiers{name: "Defense Allowed 35 to 45 Points", field_name: "defense_35_to_45_points_allowed"},

    "128" => StatIdentifiers{name: "Defensive Sacks", field_name: "defensive_sacks"},
    "129" => StatIdentifiers{name: "Defense Allowed 100 to 199 Yards", field_name: "defense_100_to_199_yards_allowed"},
    "130" => StatIdentifiers{name: "Defense Allowed 200 to 299 Yards", field_name: "defense_200_to_299_yards_allowed"},
    "132" => StatIdentifiers{name: "Defense Allowed 350 to 399 Yards", field_name: "defense_350_to_399_yards_allowed"},
    "133" => StatIdentifiers{name: "Defense Allowed 400 to 449 Yards", field_name: "defense_400_to_449_yards_allowed"},
    "134" => StatIdentifiers{name: "Defense Allowed 450 to 499 Yards", field_name: "defense_450_to_499_yards_allowed"},
    "135" => StatIdentifiers{name: "Defense Allowed 500 to 549 Yards", field_name: "defense_500_to_549_yards_allowed"},
    "136" => StatIdentifiers{name: "Defense Allowed More than 550 Yards", field_name: "defense_over_550_yards_allowed"},
    "140" => StatIdentifiers{name: "Punts Inside the 10", field_name: "punts_inside_10"},
    "141" => StatIdentifiers{name: "Punts Inside the 20", field_name: "punts_inside_20"},
    "148" => StatIdentifiers{name: "Punt Average 44+", field_name: "punt_average_over_44"},
    "149" => StatIdentifiers{name: "Punt Average 42.0-43.9", field_name: "punt_average_42_to_44"},
    "150" => StatIdentifiers{name: "Punt Average 40.0-41.9", field_name: "punt_average_40_to_42"},
    "151" => StatIdentifiers{name: "Punt Average 38.0-39.9", field_name: "punt_average_38_to_40"},
    "198" =>StatIdentifiers {name: "Field Goals Made From 50 to 59 Yards", field_name: "field_goals_made_50_to_59"},
    "201" =>StatIdentifiers {name: "Field Goals Made From 60+ Yards", field_name: "field_goals_made_60_plus"},

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
