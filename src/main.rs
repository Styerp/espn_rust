use bench_king::api::client::EspnClient;
static LEAGUE_ID: i32 = 111368805;
const SEASON_ID: i16 = 2023;
static SWID: &str = "{8A5D0331-9FAB-4A5F-BA5F-1F52DECD4646}";
static ESPN_S2: &str = "AEAE3uggnFrxe%2Fulf%2FN3Y5DZpkDwrPnG2zpRTd3z3m%2B7YJ2FmuTVIu2fqIODO1Tt9t7ypPIcvyiN%2FPIqoQVHqDvR9TaBoywLQOeCet2Wi2N1NpLWAz14%2B1CRHq32OQ4dBwh%2FRbS%2FoNGcTJBbw3uyNaHFpa5SR9dya8kHsQBtZJGvx7tyEcf%2B00nMqct3h9UyFCGtNu4FZdA6sx1zciveFSXyV7iNz2w6KvF%2FyoYrsz8eAryWKbn7g2dz74soOXwqN0WPYxxXQtQjzYLgUp4Xx%2BvV";

#[tokio::main]
async fn main() {
    let client = EspnClient::build(SWID, ESPN_S2, LEAGUE_ID);
    let data = client.get_league_settings(SEASON_ID).await;
    dbg!(data);
}
