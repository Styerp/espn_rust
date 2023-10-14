use espn_fantasy_football::client::EspnClient;
use std::env;

#[tokio::main]
async fn main() {
    let swid = env::var("SWID").unwrap();
    let espn_s2 = env::var("ESPN_S2").unwrap();
    let league_id = env::var("ESPN_LEAGUE_ID").unwrap().parse::<i32>().unwrap();
    let client = EspnClient::build( league_id, &swid, &espn_s2);

    let data = client.get_league_members(2023).await;
    dbg!(data);
}
