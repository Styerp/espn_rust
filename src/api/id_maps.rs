use std::collections::HashMap;

use phf::phf_map;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct PositionId(u64);
impl PositionId {
    fn to_string(&self) -> &'static str {
        match POSITION_ID_MAP.get(&self.0.to_string()) {
            Some(f) => f,
            None => "Unknown?",
        }
    }
}
// Adapted from https://github.com/mkreiser/ESPN-Fantasy-Football-API/blob/main/src/constants.js;
static POSITION_ID_MAP: phf::Map<&'static str, &'static str> = phf_map! {
  "0" => "QB",
  "1" => "TQB",
  "2" => "RB",
  "3" =>"RB/WR",
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
  "22" => "Unknown?", // TODO: Figure out what this is
  "23" => "RB/WR/TE",
  "24" => "Unknown?" // TODO: Figure out what this is
};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ProTeamId(u32);
impl ProTeamId {
    fn to_string(&self) -> &'static str {
        match PRO_TEAM_ID_MAP.get(&self.0.to_string()) {
            Some(f) => f,
            None => "Unknown?",
        }
    }
}

static PRO_TEAM_ID_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "-1" => "Bye",
    "1" => "Atlanta Falcons",
    "2" => "Buffalo Bills",
    "3"=> "Chicago Bears",
    "4" => "Cincinnati Bengals",
    "5" => "Cleveland Browns",
    "6" => "Dallas Cowboys",
    "7" => "Denver Broncos",
    "8" => "Detroit Lions",
    "9" => "Green Bay Packers",
    "10"=> "Tennessee Titans",
    "11"=> "Indianapolis Colts",
    "12"=> "Kansas City Chiefs",
    "13"=> "Las Vegas Raiders",
    "14"=> "Los Angeles Rams",
    "15"=> "Miami Dolphins",
    "16"=> "Minnesota Vikings",
    "17"=> "New England Patriots",
    "18"=> "New Orleans Saints",
    "19"=> "New York Giants",
    "20"=> "New York Jets",
    "21"=> "Philadelphia Eagles",
    "22"=> "Arizona Cardinals",
    "23"=> "Pittsburgh Steelers",
    "24"=> "Los Angeles Chargers",
    "25"=> "San Francisco 49ers",
    "26"=> "Seattle Seahawks",
    "27"=> "Tampa Bay Buccaneers",
    "28"=> "Washington Commanders",
    "29"=> "Carolina Panthers",
    "30"=> "Jacksonville Jaguars",
    "33"=> "Baltimore Ravens",
    "34"=> "Houston Texans"
  };


#[cfg(test)]
mod test {
    use crate::api::id_maps::{PositionId, ProTeamId};
    #[test]
    fn position_id_converts_to_string() {
        assert_eq!(PositionId(0u64).to_string(), "QB");
        assert_eq!(PositionId(16u64).to_string(), "D/ST");
        assert_eq!(PositionId(24u64).to_string(), "Unknown?");
    }
    #[test]
    fn pro_team_id_converts_to_string(){
        assert_eq!(ProTeamId(0u32).to_string(), "Unknown?");
        assert_eq!(ProTeamId(16u32).to_string(), "Minnesota Vikings");
        assert_eq!(ProTeamId(24u32).to_string(), "Los Angeles Chargers");
    }
}

