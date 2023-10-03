use phf::phf_map;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Default, Clone, Eq, PartialEq)]
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

#[derive(Debug, Deserialize, Serialize, Default, Clone, Eq, PartialEq)]
pub struct ProTeamId(i8);
impl ProTeamId {
    fn to_name(&self) -> &'static str {
        &self.identifiers().name
    }
    fn to_abbreviation(&self) -> &'static str {
        &self.identifiers().abbreviation
    }
    fn identifiers(&self) -> &TeamIdentifiers {
        match PRO_TEAM_ID_MAP.get(&self.0.to_string()) {
            Some(f) => f,
            None => &TeamIdentifiers {
                name: "Unknown",
                abbreviation: "Unknown",
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
  "-1" => TeamIdentifiers{name: "Bye", abbreviation: "Bye"},
  "1" => TeamIdentifiers{ name: "Atlanta Falcons", abbreviation: "ATL"},
  "2" => TeamIdentifiers{ name: "Buffalo Bills",abbreviation:"BUF"},
  "3"=> TeamIdentifiers{ name: "Chicago Bears", abbreviation: "CHI"},
  "4" => TeamIdentifiers{ name: "Cincinnati Bengals", abbreviation:"CIN"},
  "5" => TeamIdentifiers{ name: "Cleveland Browns",abbreviation: "CLE"},
  "6" => TeamIdentifiers{ name: "Dallas Cowboys",abbreviation:"DAL"},
  "7" => TeamIdentifiers{ name: "Denver Broncos",abbreviation:"DEN"},
  "8" => TeamIdentifiers{ name: "Detroit Lions", abbreviation:"DET"},
  "9" => TeamIdentifiers{ name: "Green Bay Packers",abbreviation:"GB"},
  "10"=> TeamIdentifiers{ name: "Tennessee Titans",abbreviation:"TEN"},
  "11"=>TeamIdentifiers{ name:  "Indianapolis Colts",abbreviation: "IND"},
  "12"=> TeamIdentifiers{ name: "Kansas City Chiefs",abbreviation:"KC"},
  "13"=> TeamIdentifiers{ name: "Las Vegas Raiders",abbreviation:"LV"},
  "14"=> TeamIdentifiers{ name: "Los Angeles Rams",abbreviation:"LAR"},
  "15"=> TeamIdentifiers{ name: "Miami Dolphins",abbreviation:"MIA"},
  "16"=>TeamIdentifiers{ name:  "Minnesota Vikings",abbreviation:"MIN"},
  "17"=> TeamIdentifiers{ name: "New England Patriots",abbreviation:"NE"},
  "18"=> TeamIdentifiers{ name: "New Orleans Saints",abbreviation:"NO"},
  "19"=> TeamIdentifiers{ name: "New York Giants",abbreviation:"NYG"},
  "20"=> TeamIdentifiers{ name: "New York Jets",abbreviation:"NYJ"},
  "21"=> TeamIdentifiers{ name: "Philadelphia Eagles",abbreviation:"PHI"},
  "22"=> TeamIdentifiers{ name: "Arizona Cardinals",abbreviation:"ARI"},
  "23"=>TeamIdentifiers{ name:  "Pittsburgh Steelers",abbreviation:"PIT"},
  "24"=> TeamIdentifiers{ name: "Los Angeles Chargers",abbreviation:"LAC"},
  "25"=> TeamIdentifiers{ name: "San Francisco 49ers",abbreviation:"SF"},
  "26"=> TeamIdentifiers{ name: "Seattle Seahawks",abbreviation:"SEA"},
  "27"=>TeamIdentifiers{ name:  "Tampa Bay Buccaneers",abbreviation:"TB"},
  "28"=> TeamIdentifiers{ name: "Washington Commanders",abbreviation:"WSH"},
  "29"=> TeamIdentifiers{ name: "Carolina Panthers",abbreviation:"CAR"},
  "30"=>TeamIdentifiers{ name:  "Jacksonville Jaguars",abbreviation:"JAX"},
  "33"=> TeamIdentifiers{ name: "Baltimore Ravens",abbreviation:"BAL"},
  "34"=> TeamIdentifiers{ name: "Houston Texans", abbreviation:"HOU"},
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
    fn pro_team_id_converts_to_string() {
        assert_eq!(ProTeamId(0i8).to_name(), "Unknown?");
        assert_eq!(ProTeamId(16i8).to_name(), "Minnesota Vikings");
        assert_eq!(ProTeamId(24i8).to_name(), "Los Angeles Chargers");
    }
}
