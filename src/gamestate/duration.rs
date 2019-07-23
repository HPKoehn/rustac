#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Duration {
    Infinite,
    Steps(i32),
    Updates(i32)
}