use axum::extract::ws::WebSocket;
use axum::extract::WebSocketUpgrade;
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::{any, get};
use axum::{extract::State, routing::post, Json, Router};
use axum::{middleware, Extension};
use error::AppError;
use game::GameManager;
use misc::Queue;
use models::UserExt;
use serde::{Deserialize, Serialize};
use sillyrng::*;
use sqlx::{Acquire, PgPool, Pool, Postgres};
use std::collections::HashMap;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use ulid::{self};

mod error;
mod game;
mod misc;
mod models;

pub struct AppState {
    games: Mutex<HashMap<ulid::Ulid, GameState>>,
    game_manager: GameManager,
    db: Pool<Postgres>,
}

/// enum representing all possible moves done by the client
#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Move {
    CursorRedUp,
    CursorRedDown,
    CursorRedLeft,
    CursorRedRight,
    CursorBlueUp,
    CursorBlueDown,
    CursorBlueLeft,
    CursorBlueRight,
    Submit,
}

//TODO use cheater mode to gather heuristics

#[tokio::main]
async fn main() {
    // basic initialization
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DB_URL must be set");

    let pool = PgPool::connect(&database_url).await.unwrap();

    tracing_subscriber::fmt::init();

    let state = Arc::new(AppState {
        games: Mutex::new(HashMap::new()),
        game_manager: GameManager {
            user_games:
                Queue::<Arc<(ulid::Ulid, tokio::sync::mpsc::Sender<WebSocket>)>>::default_sized(32),
            cheater_games:
                Queue::<Arc<(ulid::Ulid, tokio::sync::mpsc::Sender<WebSocket>)>>::default_sized(32),
            anon_games:
                Queue::<Arc<(ulid::Ulid, tokio::sync::mpsc::Sender<WebSocket>)>>::default_sized(32),
        },
        db: pool,
    });
    let app = Router::new()
        .route("/get-seed", post(create_seed))
        .route("/submit-game", post(submit_game))
        .route("/game", any(ws_upgrader))
        .route("/get_scores", post(misc::get_scores))
        .route("/user/signup", post(misc::signup))
        .route("/user/login", post(misc::login))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            misc::authorization,
        ))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, Deserialize)]
pub struct GameForm {
    dimension: u8,
    time_limit: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct GameState {
    seed: u32,
    dimension: u8,
    time_limit: Duration,
    start_time: Instant,
}

#[derive(Serialize)]
pub struct Seed {
    id: String,
    seed: u32,
}

pub async fn ws_upgrader(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<Option<UserExt>>,
) -> Response {
    // required due to state not implementing copy
    let cloned_state = state.clone();
    ws.on_upgrade(move |socket| ws_handler(socket, cloned_state, user))
}

pub async fn ws_handler(ws: WebSocket, state: Arc<AppState>, user: Option<UserExt>) {
    state.game_manager.clone().assign_game(ws, user).await
}

/// creates a new seed using the implemented splitmix and xoshiro256+ algorithms from sillyrng
#[axum::debug_handler]
pub async fn create_seed(
    State(state): State<Arc<AppState>>,
    Json(form): Json<GameForm>,
) -> (StatusCode, Json<Seed>) {
    let game_id = ulid::Ulid::new();
    let seed = rand::random::<u32>();
    let game_state = GameState {
        seed,
        dimension: form.dimension,
        time_limit: Duration::from_secs(form.time_limit.into()),
        start_time: Instant::now(),
    };

    println!(
        "Creating game {} with dimension {} and time limit {}s",
        game_id, form.dimension, form.time_limit
    );

    state.games.lock().await.insert(game_id, game_state);

    let res = Json(Seed {
        id: game_id.to_string(),
        seed,
    });

    (StatusCode::OK, res)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameEnd {
    id: String,
    score: u32,
    //u32 is time difference in ms
    moves: Vec<(Move, u32)>,
}
#[axum::debug_handler]
pub async fn submit_game(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<Option<UserExt>>,
    Json(game): Json<GameEnd>,
) -> Result<(StatusCode, Json<u32>), AppError> {
    println!(
        "Received submission for game {} with {} moves",
        game.id,
        game.moves.len()
    );
    let id = ulid::Ulid::from_string(&game.id).unwrap();
    let lock = state.games.lock().await;
    let mut conn = state.db.acquire().await?;
    let details = lock.get(&id).unwrap();
    let time = verify_timings(game.moves.iter().map(|(_, m)| *m).collect(), state.clone()).await;

    if !time.0 {
        println!("Rejected game {} due to suspicious timings", game.id);
        return Ok((StatusCode::NOT_ACCEPTABLE, Json(0)));
    }
    let score = match verify_moves(
        game.moves.iter().map(|(m, _)| *m).collect(),
        details.dimension,
        details.seed,
    )
    .await
    {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", e);
            // TODO anomalous game pushing
            return Ok((StatusCode::NOT_ACCEPTABLE, Json(0)));
        }
    };
    if score == game.score {
        if let Some(u) = user {
            println!(
                "Game {} submitted with score {}, user exists : {}",
                game.id,
                score,
                u.clone().username
            );
            sqlx::query!("INSERT INTO \"game\" (game_id,score,average_time,dimension,time_limit,user_id) VALUES ($1,$2,$3,$4,$5,$6)",uuid::Uuid::new_v4(),score as i32,time.1, details.dimension as i32,30,u.id).execute(&mut *conn).await?;
        }
        Ok((StatusCode::OK, Json(score)))
    } else {
        Ok((StatusCode::NOT_ACCEPTABLE, Json(0)))
    }
}

pub async fn verify_timings(timings: Vec<u32>, state: Arc<AppState>) -> (bool, f32) {
    let mean = timings.iter().sum::<u32>() as f64 / timings.len() as f64;

    let variance = timings
        .iter()
        .map(|&x| {
            let diff = mean - (x as f64);
            diff * diff
        })
        .sum::<f64>()
        / timings.len() as f64;

    let std_dev = variance.sqrt();

    // from testing, due to being able to think you can technically beat human reaction time, but anything less than 100 is already suspicious
    if std_dev < 50.0 {
        println!(
            "Timing anomaly detected: mean={:.2}ms, std_dev={:.2}ms",
            mean, std_dev
        );
        return (false, 0.0);
    }
    (true, mean as f32)

    // todo add more thingies
}

pub async fn verify_moves(moves: Vec<Move>, size: u8, seed: u32) -> Result<u32, String> {
    //this is assuming we start at 0,0 and size,size (should be a client side force, now enforced)
    let mut rng = sillyrng::Xoshiro256plus::new(Some(seed as u64));
    let mut grid: Vec<bool> = vec![false; (size * size) as usize];
    let mut blue_coords: (u8, u8) = (0, 0);
    let mut red_coords: (u8, u8) = (size - 1, size - 1);
    let mut score = 0;
    let mut distance = 0;
    let mut anomalous_distances = 0;
    let mut optimal_distance = 0;
    let mut count = 0;
    while count < size {
        let x: u8 = (rng.next() * size as f64).floor() as u8;
        let y: u8 = (rng.next() * size as f64).floor() as u8;
        if grid[(x * size + y) as usize] == false {
            grid[(x * size + y) as usize] = true;
            count += 1;
        }
    }
    for i in moves.iter() {
        match i {
            Move::CursorRedUp => {
                red_coords.1 = (red_coords.1 as i8 - 1).max(0) as u8;
                distance += 1;
            }
            Move::CursorRedDown => {
                red_coords.1 = (red_coords.1 + 1).min(size - 1);
                distance += 1;
            }
            Move::CursorRedLeft => {
                red_coords.0 = (red_coords.0 as i8 - 1).max(0) as u8;
                distance += 1;
            }
            Move::CursorRedRight => {
                red_coords.0 = (red_coords.0 + 1).min(size - 1);
                distance += 1;
            }
            Move::CursorBlueUp => {
                blue_coords.1 = (blue_coords.1 as i8 - 1).max(0) as u8;
                distance += 1;
            }
            Move::CursorBlueDown => {
                blue_coords.1 = (blue_coords.1 + 1).min(size - 1);
                distance += 1;
            }
            Move::CursorBlueLeft => {
                blue_coords.0 = (blue_coords.0 as i8 - 1).max(0) as u8;
                distance += 1;
            }
            Move::CursorBlueRight => {
                blue_coords.0 = (blue_coords.0 + 1).min(size - 1);
                distance += 1;
            }
            Move::Submit => {
                if distance <= optimal_distance {
                    anomalous_distances += 1;
                }
                distance = 0;

                if grid[(red_coords.0 * size + red_coords.1) as usize]
                    && grid[(blue_coords.0 * size + blue_coords.1) as usize]
                    && !(blue_coords == red_coords)
                {
                    score += 1;
                    let mut count = 0;
                    let r = red_coords.0 * size + red_coords.1;
                    let b = blue_coords.0 * size + blue_coords.1;
                    while count < 2 {
                        let x: u8 = (rng.next() * size as f64).floor() as u8;
                        let y: u8 = (rng.next() * size as f64).floor() as u8;
                        if !grid[(x * size + y) as usize]
                            && (x * size + y != r || x * size + y != b)
                        {
                            grid[(x * size + y) as usize] = true;
                            count += 1;
                        }
                    }
                    grid[r as usize] = false;
                    grid[b as usize] = false;
                    optimal_distance =
                        get_optimal_paths(grid.clone(), red_coords, blue_coords, size)
                            .await
                            .iter()
                            .min()
                            .unwrap_or(&0)
                            .to_owned();
                } else {
                    score = 0
                }
            }
        }
    }
    println!(
        "Game completed with score {} (anomaly ratio: {:.2})",
        score,
        anomalous_distances as f64 / score as f64
    );
    Ok(score)
}

// more "difficult algorithm" TODO
pub async fn get_optimal_paths(grid: Vec<bool>, r: (u8, u8), b: (u8, u8), size: u8) -> Vec<u32> {
    let mut paths = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i] && grid[j] && i != j {
                let r_cell = ((i / size as usize) as u8, (i % size as usize) as u8);
                let b_cell = ((j / size as usize) as u8, (j % size as usize) as u8);
                let r_dist = (r.0.abs_diff(r_cell.0) + r.1.abs_diff(r_cell.1)) as u32;
                let b_dist = (b.0.abs_diff(b_cell.0) + b.1.abs_diff(b_cell.1)) as u32;
                paths.push(r_dist + b_dist);
            }
        }
    }
    paths
}
