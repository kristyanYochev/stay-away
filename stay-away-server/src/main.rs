mod core;
mod lobby;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use lobby::{Lobby, LobbyHandle};
use serde::Deserialize;
use warp::Filter;

#[derive(Debug, Deserialize)]
struct CreateLobbyRequest {
    username: String,
}

type LobbyId = String;
type LobbyDb = Arc<Mutex<HashMap<LobbyId, LobbyHandle>>>;

#[tokio::main]
async fn main() {
    let lobbies = LobbyDb::default();

    let create_lobby = warp::path!("create-lobby")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_lobbies(lobbies.clone()))
        .map(|request: CreateLobbyRequest, lobbies: LobbyDb| {
            let lobby_handle = Lobby::start();
            let lobby_id = generate_lobby_id();
            {
                let mut db = lobbies.lock().unwrap();
                db.insert(lobby_id.clone(), lobby_handle);
            }
            format!("Lobby generated with id {}", lobby_id)
        });

    let all_routes = create_lobby;

    warp::serve(all_routes).run(([127, 0, 0, 1], 3000)).await;
}

fn with_lobbies(
    lobbies: LobbyDb,
) -> impl Filter<Extract = (LobbyDb,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || lobbies.clone())
}

fn generate_lobby_id() -> LobbyId {
    // TODO: Use actually random strings
    "123456".to_string()
}
