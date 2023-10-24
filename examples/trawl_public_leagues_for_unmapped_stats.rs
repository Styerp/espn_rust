use std::io::Write;

use espn_fantasy_football::{
    client::EspnClient,
    id_maps::StatId,
    matchup::Roster,
    team::{Team, TeamId},
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    find_public_league_stats(436937, 111_368_805).await;
}

fn write_data(league: i32, data: Vec<Data>) -> Result<(), std::io::Error> {
    if data.len() > 0 {
        let file = ::std::fs::File::create(format!("{league}.txt"))?;
        let mut file = std::io::BufWriter::new(file);
        for stat in data {
            serde_json::to_writer(&mut file, &stat).unwrap();
            file.write_all(b"\n").unwrap();
        }
    }
    Ok(())
}

async fn find_unknowns_for_league(league: i32) -> Vec<Data> {
    let mut final_data = Vec::new();
    //println!("============== LEAGUE {league} ==================");

    let client = EspnClient::build(league, "", "");
    let teams = client.get_team_data(2023).await;

    for week in 1..=6 {
        //  println!("==============  WEEK {week}  ==================");
        let data = client.get_matchups_for_week(2023, week, week).await;

        for box_score in data {
            let away = match box_score.away {
                Some(a) => a,
                None => break,
            };
            let home = match box_score.home {
                Some(a) => a,
                None => break,
            };
            match away.roster_for_current_scoring_period {
                Some(m) => {
                    final_data.append(&mut get_unknowns(m, away.team_id, &teams, league, week))
                }
                None => {}
            };
            match home.roster_for_current_scoring_period {
                Some(m) => {
                    final_data.append(&mut get_unknowns(m, home.team_id, &teams, league, week))
                }
                None => {}
            }
        }
    }
    //println!("============== END LEAGUE {league} ==================");
    final_data
}

#[derive(Serialize, Deserialize)]
struct Data {
    league: i32,
    week: u8,
    team_name: String,
    player_name: String,
    stat: StatId,
    value: f32,
}

fn get_unknowns(
    roster: Roster,
    team: TeamId,
    team_data: &Vec<Team>,
    league: i32,
    week: u8,
) -> Vec<Data> {
    let team_detail = team_data.iter().find(|x| team == x.id).unwrap();

    let mut return_value = Vec::new();
    for player in roster.entries {
        let stat = match player
            .player_pool_entry
            .player
            .stats
            .expect("There are stats")
            .get(0)
        {
            Some(m) => m.clone(),
            None => break,
        };

        for val in stat.applied_stats.unwrap() {
            if val.0.to_name() == "Unknown" && val.1 > 0.5 {
                // println!(
                //     "Team {:?} Player {} Stat Number {:?} Value {}",
                //     team_detail.name,
                //     player.player_pool_entry.player.full_name.clone(),
                //     val.0,
                //     val.1
                // );
                return_value.push(Data {
                    league,
                    week,
                    team_name: team_detail.name.clone(),
                    stat: val.0,
                    value: val.1,
                    player_name: player.player_pool_entry.player.full_name.clone(),
                })
            }
        }
    }
    return_value
}

async fn find_public_league_stats(start: i32, end: i32) {
    for league in start..end {
        if league % 1000 == 0 {
            println!("Tracking at league {}", league)
        }

        let client = EspnClient::build(league, "", "");

        let data = client.get_league_members(2023).await;
        match data {
            Ok(_) => {
                println!("League {} is a go!", league);
                let data = find_unknowns_for_league(league).await;
                match write_data(league, data) {
                    Ok(_) => {}
                    Err(e) => println!("{}", e),
                }
            }
            Err(_) => {
                //println!("Not public");
            }
        };
    }
}
