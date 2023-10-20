use serde::{Deserialize, Serialize};

use crate::{client::EspnClient, league::LeagueMember};

/// A league member of the Fantasy Football League
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Hash)]
pub struct MemberId(pub String);
impl MemberId {
    pub async fn to_details(&self, client: EspnClient, season: u16) -> LeagueMember {
        let data = match client.get_league_members(season).await {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        match data.iter().find(|x| &x.id == self) {
            Some(s) => s.clone(),
            None => panic!("Wrong league?"),
        }
    }
}
impl std::fmt::Display for MemberId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MemberId({})", self.0)
    }
}
