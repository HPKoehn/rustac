extern crate serde;

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActorState {
    WaitingForTurn,
    Acting,
    DoneActing
}