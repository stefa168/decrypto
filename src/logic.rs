use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProvidingState {
    pub hints: [String; 3],
    pub picked_words: [String; 3],
}

#[derive(Serialize, Deserialize)]
pub struct GuessingState {
    pub hints: [String; 3],
    pub picked_words: [String; 3],
}

#[derive(Serialize, Deserialize)]
enum TurnState {
    Providing(ProvidingState),
    Guessing(GuessingState),
    Waiting,
}

#[derive(Serialize, Deserialize)]
pub struct ConnectedState {
    pub words: [String; 4],
    pub turn: TurnState,
    pub miscommunications: usize,
    pub interceptions: usize,
    pub enemy_interceptions: usize,
    pub enemy_miscommunications: usize,
    pub enemy_words: [Vec<String>; 4],
}

#[derive(Serialize, Deserialize)]
enum GameState {
    Connected(ConnectedState),
    Disconnected,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    username: String,
    game: GameState,
}

#[derive(Hash, Eq, PartialEq)]
pub struct RoomName(String);

impl Display for RoomName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
