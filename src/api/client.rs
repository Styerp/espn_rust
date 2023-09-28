use reqwest::{
    cookie::Jar,
    header::{HeaderMap, COOKIE},
    Client,
};
const ESPN_FF_BASE_URL: &str = "https://fantasy.espn.com/apis/v3/games/ffl/seasons";

pub struct EspnClient<'a> {
    pub client: Client,
    pub league_id: i32,
    pub base_url: &'a str,
}

impl<'a> EspnClient<'a> {
    pub fn build(swid: &str, espn_s2: &str, league_id: i32) -> EspnClient<'a> {
        let mut headers = HeaderMap::new();
        headers.insert(COOKIE, format!("SWID={swid}").parse().unwrap());
        headers.insert(COOKIE, format!("espn_s2={espn_s2}").parse().unwrap());
        let cookie_store: Jar = Jar::default();
        cookie_store.add_cookie_str(
            format!("SWID={swid}; espn_s2={espn_s2}").as_str(),
            &ESPN_FF_BASE_URL.parse().unwrap(),
        );
        let client_builder = Client::builder()
            .default_headers(headers.clone())
            .connection_verbose(true)
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

    pub fn main_api_string(&self, season: i32) -> String {
        format!(
            "{}/{}/segments/0/leagues/{}",
            self.base_url, season, self.league_id
        )
    }
}
