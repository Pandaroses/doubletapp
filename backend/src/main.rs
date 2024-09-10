use axum::http::StatusCode;
use axum::{extract::State, routing::post, Json, Router};
use scc::HashMap;
use serde::{Deserialize, Serialize};
use std::mem::transmute;
use std::ops::Deref;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use ulid::{self};

pub struct AppState {
    games: HashMap<ulid::Ulid, GameState>,
}

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug)]
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

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let state = Arc::new(AppState {
        games: HashMap::new(),
    });

    let app = Router::new()
        .route("/get-seed", post(create_seed))
        .route("/submit-game", post(submit_game))
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

#[derive(Debug)]
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
    state.games.insert(game_id, game_state).unwrap();
    let x = Json(Seed {
        id: game_id.to_string(),
        seed,
    });
    println!("{}", game_id.to_string());
    (StatusCode::OK, x)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameEnd {
    id: String,
    moves: Vec<Move>,
}
#[axum::debug_handler]
pub async fn submit_game(
    State(state): State<Arc<AppState>>,
    Json(game): Json<GameEnd>,
) -> Json<u32> {
    println!("recieved");
    let id = ulid::Ulid::from_string(&game.id).unwrap();
    let deets = state.games.get(&id).unwrap();
    // state.games.remove(&id).unwrap();
    verify_moves(game.moves, deets.dimension, deets.seed)
        .await
        .unwrap();

    Json(4)
}

pub async fn verify_moves(moves: Vec<Move>, size: u8, seed: u32) -> Result<u32, String> {
    //this is assuming we start at 0,0 and size,size
    println!("verifying moves");
    let mut rng = alea::Rng::with_seed(seed as u64);
    let mut grid: Vec<bool> = vec![false; (size * size) as usize];
    let mut blue_coords: (u8, u8) = (0, 0);
    let mut red_coords: (u8, u8) = (0, 0);
    let mut score = 0;

    let mut count = 0;
    while count < size {
        let x: u8 = (rng.f64_less_than(1.0) * size as f64).floor() as u8;
        let y: u8 = (rng.f64_less_than(1.0) * size as f64).floor() as u8;
        if grid[(x * size + y) as usize] == false {
            grid[(x * size + y) as usize] = true;
            count += 1;
        }
    }
    dbg!(grid);
    Ok(4)
}
