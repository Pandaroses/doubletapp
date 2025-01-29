use std::{collections::HashMap, sync::Arc};

use axum::extract::ws::WebSocket;
use tokio::sync::{mpsc, Mutex};
use ulid::Ulid;

use crate::Move;

#[derive(Clone)]
pub struct GameManager {
    pub loading_games: Arc<Mutex<HashMap<ulid::Ulid, mpsc::Sender<WebSocket>>>>,
}

impl GameManager {
    pub async fn assign_game(&self, mut ws: WebSocket) {
        println!("Attempting to assign player to a game");
        let mut games = self.loading_games.lock().await;
        let mut dead_games = vec![];
        for (i, tx) in games.iter() {
            match tx.send(ws).await {
                Ok(()) => return,
                Err(mpsc::error::SendError(rws)) => {
                    ws = rws;
                    dead_games.push(i.clone());
                }
            }
        }

        for i in dead_games.iter() {
            games.remove(&i);
        }

        let (tx, rx) = mpsc::channel(40);
        let game_id = Ulid::new();
        tokio::spawn(game_handler(game_id.clone(), rx));

        tx.send(ws).await.unwrap();
        println!("Created new game with ID: {}", game_id);
        games.insert(game_id, tx);
    }
}

pub struct Game {
    players: HashMap<Ulid, Player>,
    seed: u32,
    quota: u32,
}

pub struct Player {
    grid: [bool; 16],
    b_coords: (u8, u8),
    r_coords: (u8, u8),
    current_score: u8,
}

async fn game_handler(id: Ulid, mut rx: mpsc::Receiver<WebSocket>) {
    println!("Game {} initialized, waiting for players", id);
    let mut state = Game {
        players: HashMap::new(),
        seed: 0,
        quota: 0,
    };
    let mut websockets = vec![];

    while state.players.len() <= 5 {
        match rx.recv().await {
            Some(mut p) => {
                let meow_id = Ulid::new();
                println!("Player {} joined game {}", meow_id, id);
                p.send(axum::extract::ws::Message::Text(
                    serde_json::to_string(&WsMessage::ID(meow_id)).unwrap()
                ))
                .await
                .unwrap();
                state.players.insert(
                    meow_id,
                    Player {
                        grid: [false; 16],
                        b_coords: (0, 0),
                        r_coords: (7, 7),
                        current_score: (0),
                    },
                );
                websockets.push(p);
            }
            None => {}
        }
    }
    
    println!("Game {} starting with {} players", id, state.players.len());
    for i in websockets.iter_mut() {
        i.send(axum::extract::ws::Message::Text(
            serde_json::to_string(&WsMessage::Start(state.seed)).unwrap()
        ))
        .await
        .unwrap();
    }
    
    println!("Game {} is now running", id);
    loop {
        let select_result = futures::future::select_all(
            websockets
                .iter_mut()
                .enumerate()
                .map(|(i, ws)| Box::pin(async move {
                    ws.recv().await.map(|m| (i, m))
                }))
        ).await;

        match select_result {
            (Some((idx, Ok(axum::extract::ws::Message::Text(text)))), _, remaining) => {
                dbg!(&text);
                match serde_json::from_str::<MMove>(&text) {
                    Ok(mrrp) => {
                        println!("Received move from socket {}: {:?}", idx, mrrp);
                        // Handle the move
                    }
                    Err(e) => {
                        println!("Error parsing move from socket {}: {}", idx, e);
                    }
                }
            }
            (None, _, _) => {
                println!("All websockets closed for game {}", id);
                break;
            }
            _ => {
                // Handle other message types or errors
                continue;
            }
        }
    }
    println!("Game {} has ended", id);
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct MMove {
    player_id: Ulid,
    action: Move,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WsMessage {
    Move(Move),
    Quota { quota: u32, players_left: u32 },
    ID(Ulid),
    Start(u32),
    Ping,
}


