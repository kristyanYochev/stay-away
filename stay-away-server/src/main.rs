mod core;
mod lobby;

use serde::Deserialize;
use warp::Filter;

#[derive(Debug, Deserialize)]
struct CreateLobbyRequest {
    username: String,
}

#[tokio::main]
async fn main() {
    let create_lobby = warp::path!("create-lobby")
        .and(warp::post())
        .and(warp::body::json())
        .map(|request: CreateLobbyRequest| {
            format!("Hello, {}, I just made a lobby for you!", request.username)
        });

    let all_routes = create_lobby;

    warp::serve(all_routes).run(([127, 0, 0, 1], 3000)).await;
}
