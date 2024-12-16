use axum::{
    extract::{Json, Path},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

// Replace these with actual implementations from your project
use crate::moves::maximin::compute_best_move;
use crate::game::{Game, Player};

/// Request payload structure for game state
#[derive(Deserialize, Serialize)]
struct GameState {
    game_state: String,
    player: String,
}

/// Response payload structure for AI move
#[derive(Deserialize, Serialize)]
struct AIMove {
    action: String,
    details: String,
}

/// Handler for root endpoint
async fn root() -> &'static str {
    "Settlers of Catan AI Server: Ready to process game states!"
}

/// Handler to process game state and return the AI's move
async fn compute_move(Json(payload): Json<GameState>) -> Json<AIMove> {
    // Parse the incoming game state
    // let game: Game = match payload.game_state.parse::<Game>() {
    //     Ok(game) => game,
    //     Err(_) => {
    //         return Json(AIMove {
    //             action: "error".to_string(),
    //             details: "Invalid game state".to_string(),
    //         });
    //     }
    // };
    // 
    // // Determine the player
    // let player = match payload.player.as_str() {
    //     "Red" => Player::Red,
    //     "Blue" => Player::Blue,
    //     "White" => Player::White,
    //     _ => {
    //         return Json(AIMove {
    //             action: "error".to_string(),
    //             details: "Invalid player".to_string(),
    //         });
    //     }
    // };
    // 
    // // Compute the next move
    // let move_details = compute_best_move(&game, player);

    // Return the AI's suggested move
    Json(AIMove {
        action: "move".to_string(),
        details: "".to_string(),
    })
}

/// Create the Axum application
pub async fn start_server() {
    // Build the app
    let app = Router::new()
        .route("/", get(root))
        .route("/compute_move", post(compute_move));

    // Define the address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Server running at http://{}", addr);

    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
