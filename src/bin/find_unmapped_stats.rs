use espn_fantasy_football::api::{
    client::EspnClient,
    matchup::Roster,
    team::{TeamId, TeamResponse},
};
use std::env;
static LEAGUE_ID: i32 = 111368805;
const SEASON_ID: i16 = 2023;

#[tokio::main]
async fn main() {
    let week = env::args()
        .collect::<Vec<_>>()
        .get(1)
        .expect("A week value to be supplied")
        .parse()
        .expect("To be a number");
    let swid = match env::var("SWID") {
        Ok(swid) => {
            println!("Got SWID from env");
            swid},
        Err(_) => "".to_string()
    };
    let espn_s2 = match env::var("ESPN_S2") {
        Ok(espn_s2) => {
            println!("Got ESPN_S2 from env");
            espn_s2},
        Err(_) => "".to_string()
    };
    let client = EspnClient::build(swid.as_str(), espn_s2.as_str(), LEAGUE_ID);
    let teams = client.get_team_data(SEASON_ID).await;
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
fn get_unknowns(roster: Roster, team: TeamId, team_data: &TeamResponse) {
    let filt = team_data
        .teams
        .iter()
        .filter(|x| team == x.id)
        .collect::<Vec<_>>();
    let team_detail = filt.get(0).unwrap();
    for player in roster.entries {
        let stat = player
            .player_pool_entry
            .player
            .stats
            .expect("There are stats")
            .get(0)
            .expect("At least one")
            .clone();

        for val in stat.applied_stats {
            if val.0.to_name() == "Unknown" && val.1 > 0.99 {
                println!(
                    "Team {:?} Player {} Stat Number {:?} Value {}",
                    team_detail.name, player.player_pool_entry.player.full_name, val.0, val.1
                )
            }
        }
    }
}
