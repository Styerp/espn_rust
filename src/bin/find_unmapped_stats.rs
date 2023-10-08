use bench_king::api::{
    client::EspnClient,
    matchup::Roster,
    team::{TeamId, TeamResponse},
};
use std::env;
static LEAGUE_ID: i32 = 111368805;
const SEASON_ID: i16 = 2023;
static SWID: &str = "{8A5D0331-9FAB-4A5F-BA5F-1F52DECD4646}";
static ESPN_S2: &str = "AEAE3uggnFrxe%2Fulf%2FN3Y5DZpkDwrPnG2zpRTd3z3m%2B7YJ2FmuTVIu2fqIODO1Tt9t7ypPIcvyiN%2FPIqoQVHqDvR9TaBoywLQOeCet2Wi2N1NpLWAz14%2B1CRHq32OQ4dBwh%2FRbS%2FoNGcTJBbw3uyNaHFpa5SR9dya8kHsQBtZJGvx7tyEcf%2B00nMqct3h9UyFCGtNu4FZdA6sx1zciveFSXyV7iNz2w6KvF%2FyoYrsz8eAryWKbn7g2dz74soOXwqN0WPYxxXQtQjzYLgUp4Xx%2BvV";

#[tokio::main]
async fn main() {
    let week = env::args()
        .collect::<Vec<_>>()
        .get(1)
        .expect("A week value to be supplied")
        .parse()
        .expect("To be a number");
    let client = EspnClient::build(SWID, ESPN_S2, LEAGUE_ID);
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
