use rock_paper_rust::{game, server};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    let game_state = Arc::new(Mutex::new(game::GameState::new()));

    println!("Server listening on port 3333");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected!");
                let game_state = Arc::clone(&game_state);
                thread::spawn(move || {
                    server::handle_client(stream, game_state);
                });
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
}
