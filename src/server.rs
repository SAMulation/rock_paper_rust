use crate::game;
use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};

pub fn handle_client(mut stream: TcpStream, game_state: Arc<Mutex<game::GameState>>) {
    loop {
        // Report game state at the start of each game
        {
            let state = game_state.lock().unwrap();
            let message = format!(
                "Current Score: Server {} - Client {} - Draws {}\n",
                state.server_wins, state.client_wins, state.draws,
            );
            let _ = stream.write(message.as_bytes());
        }

        // Handle client input
        let mut data = [0 as u8; 50]; // using 50 byte buffer
        match stream.read(&mut data) {
            Ok(size) => {
                let input = std::str::from_utf8(&data[..size]).unwrap();
                let trimmed = input.trim();

                match trimmed {
                    "Rock" => game::play_game(game::Move::Rock, &stream, &game_state),
                    "Paper" => game::play_game(game::Move::Paper, &stream, &game_state),
                    "Rust" => game::play_game(game::Move::Rust, &stream, &game_state),
                    _ => {
                        eprintln!("Invalid move");
                        let _ = write!(
                            stream,
                            "Invalid move. Please select Rock, Paper, or Rust.\n"
                        );
                    }
                };
            }
            Err(err) => {
                eprintln!("An error occurred while reading data: {}", err);
                break;
            }
        }
    }
}
