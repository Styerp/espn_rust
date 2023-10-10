use espn_fantasy_football::api::{
    client::EspnClient,
    matchup::Roster,
    team::{Team, TeamId, TeamResponse},
};
use std::env;

const SEASON_ID: i16 = 2023;

#[tokio::main]
async fn main() {
    let LEAGUE_ID: i32 = env::var("ESPN_LEAGUE_ID")
        .unwrap()
        .parse::<i32>()
        .expect("League to be numeric");
    let SWID: &str = &env::var("SWID").unwrap();
    let ESPN_S2: &str = &env::var("ESPN_S2").unwrap();

    let client = EspnClient::build(SWID, ESPN_S2, LEAGUE_ID);
    let teams = client.get_team_data(SEASON_ID).await;

    for week in 1..=5 {
        let data = client.get_matchups_for_week(SEASON_ID, week, week).await;

        println!("============== WEEK {week} ==================");
        for box_score in data {
            get_unknowns(
                box_score.away.roster_for_current_scoring_period.unwrap(),
                box_score.away.team_id,
                &teams,
            );
            get_unknowns(
                box_score.home.roster_for_current_scoring_period.unwrap(),
                box_score.home.team_id,
                &teams,
            );
        }
    }
}
fn get_unknowns(roster: Roster, team: TeamId, team_data: &Vec<Team>) {
    let filt = team_data
        .iter()
        .filter(|x| team == x.id)
        .collect::<Vec<_>>();
    let team_detail = filt.get(0).unwrap();
    for player in roster.entries {
        let position = player.player_pool_entry.player.default_position_id;
        if position.to_string() == "Unknown" {
            println!(
                "Team {:?} Player {} Position {:?}",
                team_detail.name, player.player_pool_entry.player.full_name, position
            )
        }
    }
}
