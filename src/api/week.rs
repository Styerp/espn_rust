use crate::api::client::EspnClient;
use serde::{Deserialize, Serialize};

impl<'a> EspnClient<'a> {
    pub async fn get_box_score_for_week(self, season: i32, week: i8) {
        let request = self.client.get(self.main_api_string(season)).query(&[
            ("?view", "mMatchup"),
            ("view", "mMatchupScore"),
            ("scoringPeriodId", week.to_string().as_str()),
        ]);

        let resp = request.send().await.unwrap();
        dbg!(resp.text().await.unwrap());
    }
}

#[derive(Deserialize, Debug, Serialize)]
struct BoxScore {
    #[serde(rename = "draftDetail")]
    draft_detail: DraftDetail,
    #[serde(rename = "gameId")]
    game_id: i32,
    id: i64,
    schedule: Vec<Schedule>,
}

#[derive(Deserialize, Debug, Serialize)]
struct DraftDetail {
    drafted: bool,
    #[serde(rename = "inProgress")]
    in_progress: bool,
}

#[derive(Deserialize, Debug, Serialize)]
struct Schedule {
    away: ScheduleDetail,
    home: ScheduleDetail,
}
#[derive(Deserialize, Debug, Serialize)]
struct ScheduleDetail {
    adjustment: f32,
    #[serde(rename = "teamId")]
    team_id: i32,
    #[serde(rename = "cumulativeScore")]
    cumulative_score: CumulativeScore,
}
#[derive(Deserialize, Debug, Serialize)]
struct CumulativeScore {
    losses: u8,
    #[serde(rename = "statBySlot")]
    stat_by_slot: bool,
    ties: u8,
    wins: u8,
}
