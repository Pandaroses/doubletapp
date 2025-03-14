use axum::extract::ws::{Message, WebSocket};
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt, TryFutureExt};
use sillyrng::{Gen, Xoshiro256plus};
use std::{collections::HashMap, sync::Arc};
use tokio::select;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{interval, Duration};
use ulid::Ulid;

use crate::misc::Queue;
use crate::models::UserExt;
use crate::Move;

#[derive(Clone)]
pub struct GameManager {
    pub user_games: Arc<Mutex<Queue<(ulid::Ulid, mpsc::Sender<WebSocket>)>>>,
    pub anon_games: Arc<Mutex<Queue<(ulid::Ulid, mpsc::Sender<WebSocket>)>>>,
    pub cheater_games: Arc<Mutex<Queue<(ulid::Ulid, mpsc::Sender<WebSocket>)>>>,
}

impl GameManager {
    pub async fn assign_game(&self, ws: WebSocket, user: Option<UserExt>) {
        println!("Attempting to assign player to a game");
        let games = match user {
            Some(u) => {
                //innocent until proven guilty, very demure, very fashionable
                if u.cheater.unwrap_or(false) {
                    self.cheater_games.clone()
                } else {
                    self.user_games.clone()
                }
            }
            None => self.anon_games.clone(),
        };
        let mut attempts = games.lock().await.size;
        let mut ws = ws;
        while attempts > 0 {
            let mut lock = games.lock().await;
            if let Some(game) = lock.dequeue() {
                match game.1.send(ws).await {
                    Ok(()) => {
                        lock.enqueue(game.clone());
                        return;
                    }
                    Err(mpsc::error::SendError(rws)) => {
                        ws = rws;
                        attempts -= 1;
                    }
                }
            } else {
                break;
            }
        }

        let (tx, rx) = mpsc::channel(40);
        let game_id = Ulid::new();
        tokio::spawn(game_handler(game_id.clone(), rx));

        match tx.send(ws).await {
            Ok(_) => {
                println!("Created new game with ID: {}", game_id);
                games.lock().await.enqueue((game_id, tx));
            }
            Err(e) => {
                println!("failed to send to game error: {}", e);
            }
        };
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
    rng: sillyrng::Xoshiro256plus,
}

async fn game_handler(id: Ulid, mut rx: mpsc::Receiver<WebSocket>) {
    println!("Game {} initialized, waiting for players", id);
    let mut state = Game {
        players: HashMap::new(),
        inactive_players: HashMap::new(),
        seed: rand::random::<u32>(),
        quota: 0,
    };
    let mut senders: HashMap<Ulid, SplitSink<WebSocket, Message>> = HashMap::new();
    let mut receivers = vec![];
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
                    meow_id.clone(),
                    Player {
                        grid: [false; 16],
                        b_coords: (0, 0),
                        r_coords: (3, 3),
                        current_score: (0),
                        rng: Xoshiro256plus::new(Some(3)),
                    },
                );
                let (sender, receiver) = p.split();
                senders.insert(meow_id, sender);
                receivers.push(receiver);
            }
            None => {}
        }
    }

    println!("Game {} starting with {} players", id, state.players.len());
    for i in state.players.iter_mut() {
        i.1.rng = Xoshiro256plus::new(Some(state.seed.clone() as u64));
        let mut count = 0;
        while count < 4 {
            let x: u8 = (i.1.rng.next() * 4 as f64).floor() as u8;
            let y: u8 = (i.1.rng.next() * 4 as f64).floor() as u8;
            if i.1.grid[(x * 4 + y) as usize] == false {
                i.1.grid[(x * 4 + y) as usize] = true;
                count += 1;
            }
        }
    }
    for (_p, i) in senders.iter_mut() {
        i.send(axum::extract::ws::Message::Text(
            serde_json::to_string(&WsMessage::Start(state.seed)).unwrap(),
        ))
        .await
        .unwrap();
    }

    println!("Game {} is now running", id);
    let mut interval = interval(Duration::from_secs(5));

    loop {
        let websocket_futures = futures::future::select_all(
            receivers
                .iter_mut()
                .enumerate()
                .map(|(i, ws)| Box::pin(async move { (i, ws.next().await) })),
        );

        select! {
            (result, _, _) = websocket_futures => {
                let (idx, msg_result) = result;
                match msg_result {
                    Some(Ok(axum::extract::ws::Message::Text(text))) => {
                        match serde_json::from_str::<WsMessage>(&text) {
                            Ok(WsMessage::Move(mrrp)) => {
                                println!("Received move from socket {}: {:?}", idx, mrrp);
                                let player = state.players.get_mut(&mrrp.player_id);
                                match player {
                                    Some(p) => {
                                        match mrrp.action {
                                            Move::CursorRedUp => p.r_coords.1 = (p.r_coords.1 as i8 - 1).max(0) as u8,
                                            // 3 should be constant, but multilpayer is only 4x4
                                            Move::CursorRedDown => p.r_coords.1 = (p.r_coords.1 + 1).min(3),
                                            Move::CursorRedLeft => p.r_coords.0 = (p.r_coords.0 as i8 - 1).max(0) as u8,
                                            Move::CursorRedRight => p.r_coords.0 = (p.r_coords.0 + 1).min(3),
                                            Move::CursorBlueUp => p.b_coords.1 = (p.b_coords.1 as i8 - 1).max(0) as u8,
                                            Move::CursorBlueDown => p.b_coords.1 = (p.b_coords.1 + 1).min(3),
                                            Move::CursorBlueLeft => p.b_coords.0 = (p.b_coords.0 as i8 - 1).max(0) as u8,
                                            Move::CursorBlueRight => p.b_coords.0 = (p.b_coords.0 + 1).min(3),
                                            Move::Submit => {
                                                dbg!(p.grid);
                                                dbg!(p.r_coords,p.b_coords);
                                                if p.grid[(p.r_coords.0 * 4 + p.r_coords.1) as usize] && p.grid[(p.b_coords.0 * 4 + p.b_coords.1) as usize] && !(p.b_coords == p.r_coords) {
                                                    println!("successfull submission");
                                                    p.current_score += 1;
                                                    let mut count = 0;
                                                    let r = p.r_coords.0 * 4 + p.r_coords.1;
                                                    let b = p.b_coords.0 * 4 + p.b_coords.1;
                                                    while count < 2 {
                                                        let x: u8 = (p.rng.next() * 4 as f64).floor() as u8;
                                                        let y: u8 = (p.rng.next() * 4 as f64).floor() as u8;
                                                        if !p.grid[(x * 4 + y) as usize]
                                                            && (x * 4 + y != r || x * 4 + y != b)
                                                        {
                                                            p.grid[(x * 4 + y) as usize] = true;
                                                            count += 1;
                                                        }
                                                    }
                                                    p.grid[r as usize] = false;
                                                    p.grid[b as usize] = false;
                                                } else {
                                                p.current_score = 0;
                                            }
                                        },
                                    }
                                    }
                                    None => {
                                        println!("Recieved message from invalid player");
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
                        let _ = receivers.remove(idx);
                    }
                    _ => continue,
                }
            }
            _ = interval.tick() => {
                println!("New quota for game {}", id);
                let mut culled_players = vec![];
                let  player_count = state.players.len();
                for (i, p) in state.players.iter_mut() {
                    dbg!(p.current_score);
                    if (p.current_score as u32) < state.quota {
                        let position = (player_count - culled_players.len()) as u32;

                        if let Err(e) = senders.get_mut(&i.clone()).unwrap()
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(&WsMessage::Out(position))
                                    .expect("Failed to serialize Out message")
                            )).await
                        {
                            println!("Failed to send message to player, removing {}: {}", i, e);

                            continue;
                        }
                        culled_players.push(i.clone());
                        state.inactive_players.insert(i.clone(), p.clone());
                        senders.remove(&i);
                    }
                    p.current_score = 0;
                }
                for i in culled_players {
                    state.players.remove(&i);
                }
                if state.players.len() <= 1 {
                    println!("Game {} ended - {} player(s) remaining", id, state.players.len());
                    for (i,_) in state.players.iter() {
                        // add to database when that gets done
                        senders.get_mut(&i.clone()).unwrap().send(axum::extract::ws::Message::Text(serde_json::to_string(&WsMessage::Win).unwrap())).await.unwrap();
                    }
                    break;
                }

                state.quota += 1;
                for (_i,sender) in &mut senders {
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
    Out(u32),
    Win,
    Ping,
}
