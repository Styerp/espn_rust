use bench_king::api::{client::EspnClient, team::Record};
use std::collections::HashMap;
static LEAGUE_ID: i32 = 111368805;
const SEASON_ID: i16 = 2023;
static SWID: &str = "{8A5D0331-9FAB-4A5F-BA5F-1F52DECD4646}";
static ESPN_S2: &str = "AEAE3uggnFrxe%2Fulf%2FN3Y5DZpkDwrPnG2zpRTd3z3m%2B7YJ2FmuTVIu2fqIODO1Tt9t7ypPIcvyiN%2FPIqoQVHqDvR9TaBoywLQOeCet2Wi2N1NpLWAz14%2B1CRHq32OQ4dBwh%2FRbS%2FoNGcTJBbw3uyNaHFpa5SR9dya8kHsQBtZJGvx7tyEcf%2B00nMqct3h9UyFCGtNu4FZdA6sx1zciveFSXyV7iNz2w6KvF%2FyoYrsz8eAryWKbn7g2dz74soOXwqN0WPYxxXQtQjzYLgUp4Xx%2BvV";

pub struct SimpleRecord {
    wins: u8,
    losses: u8,
    ties: u8,
}

#[tokio::main]
async fn main() {
    let client = EspnClient::build(SWID, ESPN_S2, LEAGUE_ID);
    let teams = EspnClient::build(SWID, ESPN_S2, LEAGUE_ID)
        .get_team_data(SEASON_ID)
        .await;
    let matchups = client.get_matchups(SEASON_ID).await;
    let mut records = HashMap::new();
    for team in &teams.teams {
        records.entry(team.id.clone()).or_insert(SimpleRecord {
            wins: 0,
            losses: 0,
            ties: 0,
        });
    }
    for matchup in matchups.schedule {
        let winner = matchup.winner;
        if winner == "HOME" {
            records
                .entry(matchup.home.team_id)
                .and_modify(|x| x.wins += 1);
            records
                .entry(matchup.away.team_id)
                .and_modify(|x| x.losses += 1);
        } else if winner == "AWAY" {
            records
                .entry(matchup.away.team_id)
                .and_modify(|x| x.wins += 1);

            records
                .entry(matchup.home.team_id)
                .and_modify(|x| x.losses += 1);
        }
    }
    let mut standings = records
        .into_iter()
        .map(|(x, y)| {
            let team = teams.teams.iter().find(|t| t.id == x).unwrap();
            (team.name.clone(), y)
        })
        .collect::<Vec<_>>();
    standings.sort_by_key(|a| a.1.losses);
    // for team in teams.teams {
    //     println!("{:?}", team.record)
    // }
    for (index, (team, record)) in standings.iter().enumerate() {
        println!(
            "In {} place, team {} with a record of {}-{}-{}.",
            index + 1,
            team,
            record.wins,
            record.losses,
            record.ties
        );
    }
}
