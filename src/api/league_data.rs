use crate::api::client::EspnClient;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct LeagueInfo {
    #[serde(rename = "gameId")]
    game_id: i32,
    id: u128,
    members: Vec<LeagueMember>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct LeagueMember {
    #[serde(rename = "displayName")]
    display_name: String,
    id: String,
    #[serde(rename = "isLeagueManager")]
    is_league_manager: bool,
}

impl<'a> EspnClient<'a> {
    pub async fn get_league_data(self, season: i32) -> LeagueInfo {
        let req = self.client
        .get(format!(
            "{}/{}/segments/0/leagues/{}",
            &self.base_url,
            season,
            &self.league_id
        ))
        .query(&[("?view", "mSettings")])
        // matchup score
        // .query(&[
        //     ("view", "mMatchup"),
        //     ("view", "mMatchupScore"),
        //     ("scoringPeriodId", "3"),
        // ])
        ;
        dbg!(&req);
        let res = req.send().await.expect("Response");
        let data = res.json::<LeagueInfo>().await.unwrap();
        dbg!(&data);
        data
    }
}
