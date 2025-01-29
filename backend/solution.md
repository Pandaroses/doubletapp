```rust

  struct GameState {
    active_players: Vec<(uuid,grid,cursorpositions, score),
    inactive_player: Vec<(uuid,final_round,position),
    current_quota: u32
  }
  // tx is given in all newgames to connect
  pub async fn boobies(channel: (tx,rx), game_id: uuid ) { // only listen to uuid that matches?
    let mut sockets: Vec<Websocket>;

    while sockets.len() <= 40 {
      // do nothing of actual value and just periodically send "still alives"
    }

    sockets.iter_send("game full, starting soon"); // this part when returns should also give each uuid

    let mut state = GameState::new();
    let mut loop = interval(Duration::from_secs(5));
    loop {
      tokio::select! {
        // should handle any inputs from websockets, but only for this game
        sockets.iter().something_happened => (e) => process_socket(e)
        _ = loop.tick() => {
          cull_players();
          state.current_quota += 1
        }
       }
    }
  }



```
