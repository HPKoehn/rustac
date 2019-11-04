extern crate serde;

use serde::{Serialize, Deserialize};

/// Current state regarding acting and turns
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActorState {
    WaitingForTurn,
    Acting,
    DoneActing
}