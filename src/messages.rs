use crate::logic::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    State(State),
    Hello,
}

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    StatePls,
    UpdateState(State),
}
