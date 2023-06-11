use std::{
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Move {
    Rock,
    Paper,
    Rust,
}

// We're using an Arc<Mutex<T>> to allow shared mutable state across threads
pub struct GameState {
    pub server_wins: usize,
    pub client_wins: usize,
    pub draws: usize,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            server_wins: 0,
            client_wins: 0,
            draws: 0,
        }
    }
}

pub fn play_game(player_move: Move, mut stream: &TcpStream, game_state: &Arc<Mutex<GameState>>) {
    let server_move = Move::Rock;
    let result = determine_winner(player_move, server_move);

    match result.as_str() {
        "Player 1 wins" => {
            let mut state = game_state.lock().unwrap();
            state.client_wins += 1;
        }
        "Player 2 wins" => {
            let mut state = game_state.lock().unwrap();
            state.server_wins += 1;
        }
        _ => {
            let mut state = game_state.lock().unwrap();
            state.draws += 1;
        }
    }

    let message = format!("Result: {}\n", result);
    let _ = stream.write(message.as_bytes());
}

// TODO: Later, this will be part of our game state management on the server
pub fn determine_winner(player1_move: Move, player2_move: Move) -> String {
    if player1_move == player2_move {
        "Draw".to_string()
    } else {
        match (player1_move, player2_move) {
            (Move::Rock, Move::Rust) => "Player 1 wins".to_string(),
            (Move::Rust, Move::Paper) => "Player 1 wins".to_string(),
            (Move::Paper, Move::Rock) => "Player 1 wins".to_string(),
            _ => "Player 2 wins".to_string(),
        }
    }
}
