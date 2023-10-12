use serde::{Deserialize, Serialize};

use crate::{client::EspnClient, league::LeagueMember};

/// A league member of the Fantasy Football League
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub struct MemberId(pub String);
impl MemberId {
    pub async fn to_details(&self, client: EspnClient, season: i16) -> LeagueMember {
        let data = client.get_league_members(season).await;
        match data.iter().find(|x| x.id == self.0) {
            Some(s) => s.clone(),
            None => panic!("Wrong league?"),
        }
    }
}
