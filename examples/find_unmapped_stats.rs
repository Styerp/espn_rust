use espn_fantasy_football::{
    client::EspnClient,
    matchup::Roster,
    team::{TeamId, Team},
};

use clap::Parser;
#[derive(Parser)]
struct Args {
    #[arg(long, env = "SWID", long_help = "SWID Cookie value from espn.com")]
    swid: Option<String>,
    #[arg(
        long,
        env = "ESPN_S2",
        long_help = "ESPN_S2 Cookie value from espn.com"
    )]
    espn_s2: Option<String>,
    #[arg(
        short,
        long,
        env = "ESPN_LEAGUE_ID",
        long_help = "ESPN Fantasy League Identifier"
    )]
    league: i32,
    #[arg(short, long, long_help = "The year of the season.")]
    season: u16,
    #[arg(short, long, long_help = "The week of the season")]
    week: u8,
    #[arg(
        short,
        long,
        long_help = "When true, will operate on all weeks of the season to the specified week."
    )]
    comprehensive: bool,
}

#[tokio::main]
async fn main() {
    let cli_args = Args::parse();
    let week = cli_args.week;
    let swid = cli_args.swid.unwrap();
    let espn_s2 = cli_args.espn_s2.unwrap();
    let league_id = cli_args.league;
    let client = EspnClient::build(swid.as_str(), espn_s2.as_str(), league_id);
    let teams = client.get_team_data(cli_args.season).await;

    if cli_args.comprehensive {
        for week in 1..=cli_args.week {
            let data = client
                .get_matchups_for_week(cli_args.season, week, week)
                .await;

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
    } else {
        let data = client
                .get_matchups_for_week(cli_args.season, cli_args.week, cli_args.week)
                .await;

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
        let stat = player
            .player_pool_entry
            .player
            .stats
            .expect("There are stats")
            .get(0)
            .expect("At least one")
            .clone();

        for val in stat.applied_stats.unwrap() {
            if val.0.to_name() == "Unknown" && val.1 > 0.99 {
                println!(
                    "Team {:?} Player {} Stat Number {:?} Value {}",
                    team_detail.name, player.player_pool_entry.player.full_name, val.0, val.1
                )
            }
        }
    }
}
