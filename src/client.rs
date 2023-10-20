use super::matchup::{Matchup, MatchupResponse};
use super::team::{Team, TeamResponse};
use crate::free_agent::{FreeAgent, FreeAgentResponse};
use crate::league::*;
use crate::members::MemberId;
use crate::team::TeamId;
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use reqwest::StatusCode;
use reqwest::{
    cookie::Jar,
    header::{HeaderMap, COOKIE},
    Client,
};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde_json::json;
use std::cell::RefCell;
use std::collections::HashMap;
const ESPN_FF_BASE_URL: &str = "https://fantasy.espn.com/apis/v3/games/ffl";

pub struct EspnClient {
    pub client: ClientWithMiddleware,
    pub league_id: i32,
    pub base_url: &'static str,
    teams: RefCell<HashMap<u16, HashMap<TeamId, Team>>>,
    members: RefCell<HashMap<u16, HashMap<MemberId, LeagueMember>>>,
}

impl EspnClient {
    /// Build the EspnClient, with SWID and ESPN_S2 cookies and headers, if supplied.
    ///
    /// # Arguments
    ///
    /// * league_id - The id of the league
    /// * swid - The ESPN SWID Cookie Value; pass an empty string if not a private league.
    /// * espn_s2 - the ESPN ESPN_S2 Cookie Value; pass an empty string if not a private league.
    pub fn build(league_id: i32, swid: &str, espn_s2: &str) -> EspnClient {
        let mut headers = HeaderMap::new();
        let cookie_store = Jar::default();
        if swid != "" && espn_s2 != "" {
            headers.insert(COOKIE, format!("SWID={swid}").parse().unwrap());
            headers.insert(COOKIE, format!("espn_s2={espn_s2}").parse().unwrap());
            cookie_store.add_cookie_str(
                format!("SWID={swid}; espn_s2={espn_s2}").as_str(),
                &ESPN_FF_BASE_URL.parse().unwrap(),
            );
        }
        let client_builder = Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .cookie_provider(cookie_store.into())
            .build();
        let client = match client_builder {
            Ok(c) => ClientBuilder::new(c)
                .with(Cache(HttpCache {
                    mode: CacheMode::Default,
                    manager: CACacheManager::default(),
                    options: HttpCacheOptions::default(),
                }))
                .build(),
            Err(e) => panic!("Failed to construct client. Aborting. {e}"),
        };
        EspnClient {
            client,
            league_id,
            base_url: ESPN_FF_BASE_URL,
            teams: RefCell::new(HashMap::new()),
            members: RefCell::new(HashMap::new()),
        }
    }

    pub async fn get_league_members(&self, season: u16) -> Result<Vec<LeagueMember>, String> {
        let req = self
            .client
            .get(format!(
                "{}/seasons/{}/segments/0/leagues/{}",
                &self.base_url, season, &self.league_id
            ))
            .query(&[("view", "mTeam")]);
        let res = req.send().await.expect("LeagueInfoResponse");
        let result = match res.status() {
            StatusCode::UNAUTHORIZED => Err("401".to_string()),
            StatusCode::OK => {
                let data = match res.json::<LeagueResponse>().await {
                    Ok(f) => f,
                    Err(e) => {
                        return Err(format!(
                            "LeagueInfoResponse Deserialization Error; Ignoring. {}",
                            e
                        ))
                    }
                };
                match data.members {
                    Some(m) => Ok(m),
                    None => return Err("No league members, but there should be".to_string()),
                }
            }
            StatusCode::NOT_FOUND => Err("404".to_string()),
            _ => {
                dbg!(res);
                Err("Unknown Error".to_string())
            }
        };
        result
    }
    pub async fn get_league_status(&self, season: u16) -> LeagueStatus {
        let req = self
            .client
            .get(format!(
                "{}/seasons/{}/segments/0/leagues/{}",
                &self.base_url, season, &self.league_id
            ))
            .query(&[("view", "mStatus")]);
        let res = req.send().await.expect("LeagueInfoResponse");
        let data = res
            .json::<LeagueResponse>()
            .await
            .expect("LeagueInfoResponse Deserialization");
        match data.status {
            Some(s) => s,
            None => panic!("No league status, but there should be"),
        }
    }

    pub async fn get_league_settings(&self, season: u16) -> LeagueSettings {
        let req = self
            .client
            .get(format!(
                "{}/seasons/{}/segments/0/leagues/{}",
                &self.base_url, season, &self.league_id
            ))
            .query(&[("view", "mSettings")]);
        let res = req.send().await.expect("LeagueSettingsResponse");
        let data = res
            .json::<LeagueResponse>()
            .await
            .expect("LeagueSettingsResponse Deserialization");
        match data.settings {
            Some(s) => s,
            None => panic!("No league settings found, but there should be"),
        }
    }

    pub async fn get_team_data(&self, season: u16) -> Vec<Team> {
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

    pub async fn get_teams_at_week(&self, season: u16, scoring_period_id: u8) -> Vec<Team> {
        let req = self
            .client
            .get(format!(
                "{}/seasons/{}/segments/0/leagues/{}",
                &self.base_url, season, &self.league_id
            ))
            .query(&[
                ("view", "mTeam"),
                ("view", "mRoster"),
                ("scoringPeriodId", &scoring_period_id.to_string()),
            ]);
        let res = req.send().await.expect("Team ");

        let data = res
            .json::<TeamResponse>()
            .await
            .expect("TeamResponse Deserialization");
        data.teams
    }

    /// Get data about all matchups for the season.
    ///
    /// Does not include rosters.
    pub async fn get_matchups(&self, season: u16) -> Vec<Matchup> {
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
        data.schedule
    }

    /// Get data about matchups for a given scoringPeriod and matchupPeriod. Includes rosters.
    ///
    /// To see what scoringPeriod and matchupPeriods are related, try at schedule_settings.matchup_periods from get_league_settings.
    pub async fn get_matchups_for_week(
        &self,
        season: u16,
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
    //             ("view", "mTopPerformers"),
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
    ) -> Vec<FreeAgent> {
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
        let data = res.json::<FreeAgentResponse>().await.expect("JSON");
        data.players
    }

    /// Cached implementation to get overall team data for a season in the league.
    pub async fn teams_for_season(&self, season: u16) -> HashMap<TeamId, Team> {
        if self.teams.borrow().contains_key(&season) {
            match &self.teams.borrow().get(&season) {
                Some(t) => t.to_owned().clone(),
                None => panic!("No teams for season {}. Check your inputs.", season),
            }
        } else {
            let data = self.get_team_data(season).await;
            let mapped = data
                .iter()
                .map(|f| (f.id, f.clone()))
                .collect::<HashMap<TeamId, Team>>();
            self.teams
                .borrow_mut()
                .entry(season)
                .or_insert(mapped.clone());
            mapped
        }
    }

    /// Cache access to a specific team in the league for a given season.
    pub async fn team_for_season(&self, team: &TeamId, season: u16) -> Team {
        match self.teams_for_season(season).await.get(team) {
            Some(t) => t.clone(),
            None => panic!("No team {} for season {}", team, season),
        }
    }

    /// Cache implementation for members of the league for a season.
    pub async fn members_for_season(&self, season: u16) -> HashMap<MemberId, LeagueMember> {
        if self.members.borrow().contains_key(&season) {
            match &self.members.borrow().get(&season) {
                Some(t) => t.to_owned().clone(),
                None => panic!("No members for season {}. Check your inputs.", season),
            }
        } else {
            let data = match self.get_league_members(season).await {
                Ok(f) => f,
                Err(g) => panic!("{}", g),
            };
            let mapped = data
                .iter()
                .map(|f| (f.id.clone(), f.clone()))
                .collect::<HashMap<MemberId, LeagueMember>>();
            self.members
                .borrow_mut()
                .entry(season)
                .or_insert(mapped.clone());
            mapped
        }
    }
}
