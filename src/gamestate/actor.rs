
#[derive(Debug, PartialEq, Eq)]
pub enum ActorState {
    WaitingForTurn,
    Acting,
    DoneActing
}