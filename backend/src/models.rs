#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub password: String,
    pub username: String,
    pub admin: Option<bool>,
    pub cheater: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct UserExt {
    pub id: uuid::Uuid,
    pub username: String,
    pub admin: Option<bool>,
    pub cheater: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserStatistics {
    pub stat_id: uuid::Uuid,
    pub highest_score: Option<i16>,
    pub victories: Option<i16>,
    pub games_played: Option<i16>,
    pub elo: Option<i16>,
    pub user_id: uuid::Uuid,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Game {
    pub game_id: uuid::Uuid,
    pub score: Option<i16>,
    pub average_time: Option<f32>,
    pub dimension: Option<i16>,
    pub time_limit: Option<i16>,
    pub user_id: uuid::Uuid,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Statistics {
    pub stat_id: uuid::Uuid,
    pub total_timings: Option<f32>,
    pub total_score: Option<i64>,
    pub games_played: Option<i64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AnomalousGames {
    pub agame_id: uuid::Uuid,
    pub moves: serde_json::Value,
    pub user_id: uuid::Uuid,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Session {
    pub ssid: uuid::Uuid,
    pub expiry_date: chrono::NaiveDate,
    pub user_id: uuid::Uuid,
}
