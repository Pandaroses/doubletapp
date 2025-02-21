// pub async fn create_replay(state: State<Arc<AppState>>, id: String) -> impl IntoResponse {
//     todo!()
// }

use std::array;
use std::sync::Arc;

use axum::extract::Request;
use axum::http::{HeaderMap, StatusCode};
use axum::{extract::State, middleware::Next, response::Response, Form};
use axum::{Extension, Json};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::cookie::CookieJar;
use bcrypt;
use serde::{Deserialize, Serialize};
use sqlx::Acquire;
use uuid;
use validator::Validate;

use crate::models::{User, UserExt};
use crate::{error::AppError, AppState};

// pub async fn get_cheater_game(state: State<Arc<AppState>>) -> impl IntoResponse {
//     //TODO
//     todo!()
// }
//

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Score {
    username: String,
    score: Option<i16>,
}

#[derive(Serialize, Deserialize)]
pub struct GetScore {
    page: u32,
    dimension: u8,
    time_limit: u8,
    user_scores: bool,
}

#[axum::debug_handler]
pub async fn get_scores(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<Option<UserExt>>,
    Json(data): Json<GetScore>,
) -> Result<Json<Vec<(String, usize)>>, AppError> {
    let query_string = if data.user_scores && user.is_some() {
        r#"
        SELECT "game".score, "user".username
        FROM "game"
        JOIN "user" ON "game".user_id = "user".id
        WHERE dimension = $1
        AND time_limit = $2
        AND "user".id = $4
        ORDER BY score
        OFFSET ($3 - 1) * 100 
        FETCH NEXT 100 ROWS ONLY
        "#
    } else {
        r#"
        SELECT "game".score, "user".username
        FROM "game"
        JOIN "user" ON "game".user_id = "user".id
        WHERE dimension = $1
        AND time_limit = $2
        ORDER BY score
        OFFSET ($3 - 1) * 100 
        FETCH NEXT 100 ROWS ONLY
        "#
    };
    let user_id = match user.is_some() {
        true => user.unwrap().id,
        false => uuid::Uuid::new_v4(),
    };
    let res: Vec<(String, usize)> = sqlx::query_as::<_, Score>(
        query_string
    )
    .bind(data.dimension as i32)
    .bind(data.time_limit as i32)
    .bind(data.page as i32)
    .bind(user_id)
    .fetch_all(&mut *state.db.acquire().await?)
    .await?
    .iter()
    .map(|x| (x.username.clone(), x.score.unwrap() as usize))
    .collect();
    Ok(Json(res))
}
#[derive(Deserialize, Serialize, Validate)]
pub struct SignForm {
    pub(crate) username: String,
    #[validate(length(min = 8))]
    pub(crate) password: String,
}

#[axum::debug_handler]
pub async fn signup(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Form(details): Form<SignForm>,
) -> Result<CookieJar, AppError> {
    let mut conn = state.db.acquire().await?;
    let jar = CookieJar::from_headers(&headers);
    let exists: Option<(i64,)> = sqlx::query_as("SELECT 1 FROM \"user\" WHERE username = $1")
        .bind(&details.username)
        .fetch_optional(&mut *conn)
        .await?;

    if exists.is_some() {
        return Err(AppError::Status(StatusCode::CONFLICT));
    }

    let hashed = bcrypt::hash(details.password, bcrypt::DEFAULT_COST)?;
    let user_id = uuid::Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO \"user\" (id, username, password) VALUES ($1, $2, $3)",
        user_id,
        details.username,
        hashed
    )
    .execute(&mut *conn)
    .await?;

    let session_id = uuid::Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO session (ssid, user_id, expiry_date) VALUES ($1, $2, NOW() + INTERVAL '7 DAYS')",
        session_id,
        user_id
    )
    .execute(&mut *conn)
    .await?;

    Ok(jar.add(
        Cookie::build(("session", session_id.to_string()))
            .path("/")
            .build(),
    ))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Form(details): Form<SignForm>,
) -> Result<CookieJar, AppError> {
    let mut conn = state.db.acquire().await?;

    let user: Option<(uuid::Uuid, String)> =
        sqlx::query_as("SELECT id, password FROM \"user\" WHERE username = $1")
            .bind(&details.username)
            .fetch_optional(&mut *conn)
            .await?;

    let (user_id, hashed) = user.ok_or(AppError::Status(StatusCode::UNAUTHORIZED))?;

    if !bcrypt::verify(details.password, &hashed)? {
        return Err(AppError::Status(StatusCode::UNAUTHORIZED));
    }

    let session_id = uuid::Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO session (ssid, user_id, expiry_date) VALUES ($1, $2, NOW() + INTERVAL '7 DAYS')",
        session_id,
        user_id
    )
    .execute(&mut *conn)
    .await?;

    Ok(jar.add(
        Cookie::build(("session", session_id.to_string()))
            .path("/")
            .build(),
    ))
}

#[axum::debug_middleware]
pub async fn authorization(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let jar = CookieJar::from_headers(&headers);
    let user = if let Some(cookie) = jar.get("session") {
        if let Ok(session_id) = uuid::Uuid::parse_str(cookie.value()) {
            let mut conn = state.db.acquire().await?;
            sqlx::query_as!(
                crate::models::UserExt,
                r#"
                SELECT u.id, u.username, u.admin, u.cheater
                FROM "user" u
                INNER JOIN session s ON u.id = s.user_id
                WHERE s.ssid = $1 AND s.expiry_date > NOW()
                "#,
                session_id
            )
            .fetch_optional(&mut *conn)
            .await?
        } else {
            None
        }
    } else {
        None
    };

    request.extensions_mut().insert(user);
    let response = next.run(request).await;
    Ok(response)
}

// TODO logout function that deletes the session from the database

#[derive(Default, Clone, PartialEq, PartialOrd)]
pub struct Queue<T> {
    items: Vec<T>,
    pub size: usize,
    capacity: usize,
    front: usize,
}

impl<T: Clone> Queue<T> {
    /// see how i use proper naming convention
    pub fn enqueue<U>(&mut self, item: T) -> bool {
        if self.size == self.capacity {
            return false;
        }
        let rear = (self.front + self.size) % self.capacity;
        self.items[rear as usize] = item;
        return true;
    }
    /// Returns the dequeue of this [`Queue<T>`].
    pub fn dequeue(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let res = self.items[self.front].clone();
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
        Some(res)
    }

    pub(crate) fn default_sized(size: usize) -> Self {
        Self {
            items: Vec::with_capacity(size),
            size: 0,
            capacity: size,
            front: 0,
        }
    }
}
