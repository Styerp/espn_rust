use super::matchup::{Matchup, MatchupResponse};
use super::team::{Team, TeamResponse};
use crate::free_agent::FreeAgentResponse;
use crate::league::*;
use reqwest::{
    cookie::Jar,
    header::{HeaderMap, COOKIE},
    Client,
};
use serde_json::json;
const ESPN_FF_BASE_URL: &str = "https://fantasy.espn.com/apis/v3/games/ffl";

#[derive(Clone)]
pub struct EspnClient {
    pub client: Client,
    pub league_id: i32,
    pub base_url: &'static str,
}

impl EspnClient {
    pub fn build(swid: &str, espn_s2: &str, league_id: i32) -> EspnClient {
        let mut headers = HeaderMap::new();
        headers.insert(COOKIE, format!("SWID={swid}").parse().unwrap());
        headers.insert(COOKIE, format!("espn_s2={espn_s2}").parse().unwrap());
        let cookie_store = Jar::default();
        cookie_store.add_cookie_str(
            format!("SWID={swid}; espn_s2={espn_s2}").as_str(),
            &ESPN_FF_BASE_URL.parse().unwrap(),
        );
        let client_builder = Client::builder()
            .default_headers(headers.clone())
            .cookie_store(true)
            .cookie_provider(cookie_store.into())
            .build();
        let client = match client_builder {
            Ok(c) => c,
            Err(e) => panic!("Failed to construct client. Aborting. {e}"),
        };
        EspnClient {
            client,
            league_id,
            base_url: ESPN_FF_BASE_URL,
        }
    }

    pub async fn get_league_data(&self, season: i16) -> LeagueInfoResponse {
        let req = self.client.get(format!(
            "{}/seasons/{}/segments/0/leagues/{}",
            &self.base_url, season, &self.league_id
        ));
        let res = req.send().await.expect("LeagueInfoResponse");
        let data = res
            .json::<LeagueInfoResponse>()
            .await
            .expect("LeagueInfoResponse Deserialization");
        data
    }

    pub async fn get_league_settings(&self, season: i16) -> LeagueSettings {
        let req = self
            .client
            .get(format!(
                "{}/seasons/{}/segments/0/leagues/{}",
                &self.base_url, season, &self.league_id
            ))
            .query(&[("view", "mSettings")]);
        let res = req.send().await.expect("LeagueSettingsResponse");
        let data = res
            .json::<LeagueSettingsResponse>()
            .await
            .expect("LeagueSettingsResponse Deserialization");
        data.settings
    }

    pub async fn get_team_data(&self, season: i16) -> Vec<Team> {
        let req = self
            .client
            .get(format!(
                "{}/seasons/{}/segments/0/leagues/{}",
                &self.base_url, season, &self.league_id
            ))
            .query(&[("view", "mTeam")]);
        let res = req.send().await.expect("Team ");

        let data = res
            .json::<TeamResponse>()
            .await
            .expect("TeamResponse Deserialization");
        data.teams
    }

    pub async fn get_matchups(&self, season: i16) -> MatchupResponse {
        let req = self
            .client
            .get(format!(
                "{}/seasons/{}/segments/0/leagues/{}",
                &self.base_url, season, &self.league_id
            ))
            .query(&[("view", "mMatchup")]);
        let res = req.send().await.expect("LeagueSettingsResponse");
        let data = res
            .json::<MatchupResponse>()
            .await
            .expect("MatchupResponse Deserialization");
        data
    }

    pub async fn get_matchups_for_week(
        &self,
        season: i16,
        matchup_period_id: u8,
        scoring_period_id: u8,
    ) -> Vec<Matchup> {
        let req = self
            .client
            .get(format!(
                "{}/seasons/{}/segments/0/leagues/{}",
                &self.base_url, season, &self.league_id
            ))
            .query(&[
                ("view", "mMatchup"),      //adds the `schedule` field
                ("view", "mMatchupScore"), //adds rosters to the teams for the current scoring period
                ("scoringPeriodId", scoring_period_id.to_string().as_str()), //required for rosters
            ]);
        let res = req.send().await.expect("LeagueSettingsResponse");
        let data = res
            .json::<MatchupResponse>()
            .await
            .expect("MatchupResponse Deserialization");
        let matchups = data
            .schedule
            .iter()
            .filter(|x| x.matchup_period_id == matchup_period_id)
            .map(|x| x.to_owned())
            .collect::<Vec<_>>();
        matchups
    }

    // pub async fn get_historical_scoreboard_for_week(
    //     self,
    //     season: i16,
    //     matchup_period_id: u8,
    //     scoring_period_id: u8,
    // ) -> Vec<Matchup> {
    //     let req = self
    //         .client
    //         .get(format!(
    //             "{}/leagueHistory/{}",
    //             &self.base_url, &self.league_id
    //         ))
    //         .query(&[
    //             ("scoringPeriodId", scoring_period_id.to_string().as_str()),
    //             ("seasonId", season.to_string().as_str()),
    //             ("view", "mMatchupScore"),
    //             ("view", "mScoreboard"),
    //             //("view", "mSettings"),
    //             ("view", "mTopPerformers"),
    //             //("view", "mTeam"),
    //         ]);
    //     let res = req.send().await.expect("Historic Data");
    //     let data = res.json::<Vec<MatchupResponse>>().await.expect("JSON");
    //     data.get(0).unwrap().schedule.iter().filter(
    //         |x| x.matchup_period_id == matchup_period_id
    //     ).map(|x| x.to_owned()).collect::<Vec<_>>()
    // }

    pub async fn get_free_agents_for_week(
        &self,
        season: i16,
        scoring_period_id: u8,
        limit: u8,
    ) -> FreeAgentResponse {
        let free_agent_header_value = json!(
        {
            "players": {
            "filterStatus": {
                "value": ["FREEAGENT","WAIVERS"]
            },
            "limit": limit,
            "sortPercOwned": {
                "sortAsc": false,
                "sortPriority": 1
            }
        }});
        let req = self
            .client
            .get(format!(
                "{}/seasons/{}/segments/0/leagues/{}",
                &self.base_url, season, &self.league_id
            ))
            .query(&[
                ("scoringPeriodId", scoring_period_id.to_string().as_str()),
                ("view", "kona_player_info"),
            ])
            .header("x-fantasy-filter", free_agent_header_value.to_string());
        dbg!(&req);
        let res = req.send().await.expect("Free Agents");
        let text_resp = res.text().await.unwrap();
        let deser = &mut serde_json::Deserializer::from_str(&text_resp);
        let data: FreeAgentResponse = serde_path_to_error::deserialize(deser).unwrap();
        //let data = res.json::<FreeAgentResponse>().await.expect("JSON");
        //dbg!(&data);
        data
    }
}
