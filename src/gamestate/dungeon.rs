
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum DungeonElement {
    Floor,
    Wall,
    Connector,
    Event,
}