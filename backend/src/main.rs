use axum::http::StatusCode;
use axum::{extract::State, routing::post, Json, Router};
use rc4::cipher::generic_array::{arr, GenericArray};
use rc4::{KeyInit, StreamCipher};
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
    let mut rng = PRNG::new(seed);
    let grid: Vec<bool> = vec![false; (size * size) as usize];
    let mut blue_coords: (u8, u8) = (0, 0);
    let mut red_coords: (u8, u8) = (0, 0);
    let mut score = 0;

    println!("counting");
    println!("{}", size);
    let x = rng.next_f64();
    println!("{}", x);
    // let mut count = 0;
    // while count < size {
    // count += 1;
    // let x: u8 = rng.next_f64().floor() as u8 * size;
    // dbg!(x);
    // }
    Ok(4)
}

const WIDTH: usize = 256;
const CHUNKS: usize = 6;
const DIGITS: u32 = 52;
const MASK: u8 = (WIDTH - 1) as u8;

struct ARC4 {
    i: u8,
    j: u8,
    s: [u8; WIDTH],
}

impl ARC4 {
    fn new(key: &[u8]) -> Self {
        let mut arc4 = ARC4 {
            i: 0,
            j: 0,
            s: [0; WIDTH],
        };

        for i in 0..WIDTH {
            arc4.s[i] = i as u8;
        }

        let mut j: u8 = 0;
        for i in 0..WIDTH {
            j = j.wrapping_add(arc4.s[i]).wrapping_add(key[i % key.len()]);
            arc4.s.swap(i, j as usize);
        }

        arc4
    }

    fn g(&mut self, count: usize) -> u32 {
        let mut r = 0;
        for _ in 0..count {
            self.i = self.i.wrapping_add(1);
            self.j = self.j.wrapping_add(self.s[self.i as usize]);
            self.s.swap(self.i as usize, self.j as usize);
            let t = self.s[self.i as usize].wrapping_add(self.s[self.j as usize]);
            r = (r << 8) | (self.s[t as usize] as u32);
        }
        r
    }
}

pub struct PRNG {
    arc4: ARC4,
}

impl PRNG {
    pub fn new(seed: u32) -> Self {
        let key = seed.to_le_bytes();
        PRNG {
            arc4: ARC4::new(&key),
        }
    }

    pub fn next_f64(&mut self) -> f64 {
        let significance = 2_f64.powi(DIGITS as i32);
        let overflow = significance * 2.0;
        let start_denom = (WIDTH as f64).powi(CHUNKS as i32);
        let mut x = 0;
        let mut n = self.arc4.g(CHUNKS);
        let mut d = start_denom;

        while (n as f64) < significance {
            n = (n + x) * WIDTH as u32;
            d *= WIDTH as f64;
            x = self.arc4.g(1);
        }

        while (n as f64) >= overflow {
            n /= 2;
            d /= 2.0;
            x >>= 1;
        }

        (n as f64 + x as f64) / d
    }
}
