use axum::extract::ws::WebSocket;
use futures::{SinkExt, StreamExt};
use std::{collections::HashMap, sync::Arc};
use tokio::select;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{interval, Duration};
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
    inactive_players: HashMap<Ulid, Player>,
    seed: u32,
    quota: u32,
}

#[derive(Clone, Debug)]
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
        inactive_players: HashMap::new(),
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
                    serde_json::to_string(&WsMessage::ID(meow_id)).unwrap(),
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
            serde_json::to_string(&WsMessage::Start(state.seed)).unwrap(),
        ))
        .await
        .unwrap();
    }

    println!("Game {} is now running", id);
    let mut interval = interval(Duration::from_secs(5));
    let mut senders = vec![];
    let mut recievers = vec![];
    for i in websockets {
        let (mut sender, mut reciever) = i.split();
        senders.push(sender);
        recievers.push(reciever)
    }

    loop {
        let websocket_futures = futures::future::select_all(
            recievers
                .iter_mut()
                .enumerate()
                .map(|(i, ws)| Box::pin(async move { (i, ws.next().await) })),
        );

        select! {
            (result, _, _) = websocket_futures => {
                let (idx, msg_result) = result;
                match msg_result {
                    Some(Ok(axum::extract::ws::Message::Text(text))) => {
                        dbg!(&text);
                        match serde_json::from_str::<WsMessage>(&text) {
                            Ok(WsMessage::Move(mrrp)) => {
                                println!("Received move from socket {}: {:?}", idx, mrrp);
                                dbg!(mrrp.player_id.clone());
                                let player = state.players.get_mut(&mrrp.player_id);
                                match player {
                                    Some(player) => {
                                        player.current_score += 1;
                                    }
                                    None => {
                                        println!("player doesn't exist, this is concerning");
                                    }
                                }

                            }
                            Ok(_) => {
                                println!("Received non-move message from socket {}", idx);
                            }
                            Err(e) => {
                                println!("Error parsing message from socket {}: {}", idx, e);
                            }
                        }
                    }
                    None => {
                        println!("Socket {} closed for game {}", idx, id);
                        break;
                    }
                    _ => continue,
                }
            }
            _ = interval.tick() => {
                println!("New quota for game {}", id);
                let mut culled_players = vec![];
                for (i, p) in state.players.iter_mut() {

                    if (p.current_score as u32) <= state.quota {
                        state.inactive_players.insert(i.clone(), p.clone());
                        // culled_players.push(i.clone());
                    }
                    p.current_score = 0;
                }
                for i in culled_players {
                    state.players.remove(&i);
                }
                state.quota += 1;
                for sender in &mut senders {
                    sender.send(axum::extract::ws::Message::Text(
                        serde_json::to_string(&WsMessage::Quota {
                            quota: state.quota,
                            players_left: state.players.len() as u32,
                        })
                        .unwrap(),
                    ))
                    .await
                    .unwrap();
                }
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
    Move(MMove),
    Quota { quota: u32, players_left: u32 },
    ID(Ulid),
    Start(u32),
    Ping,
}
