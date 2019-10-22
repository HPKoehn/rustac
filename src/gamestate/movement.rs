extern crate serde;
extern crate math;

use serde::{Serialize, Deserialize};
use crate::gamestate::LocationVec;
use crate::UPDATES_PER_SECOND;


pub const DEFAULT_SPEED: f64 = 2f64;
// number of decimal digits for rounding
const PRECISION: f64 = 0.0000001;

#[derive(Eq, PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MoveIntent {
    Vector(LocationVec, f64),
    Position(LocationVec, f64)
}

impl MoveIntent {

    // move towards goal and give new Location vec, will change if Vector MoveIntent
    // param location: current location
    // return: new location
    pub fn move_from(&mut self, location: &LocationVec) -> LocationVec {
        match self {
            MoveIntent::Position(target_location, speed) => {
                // map this to MoveIntent::Vector
                let distance_vector = LocationVec {
                    x: target_location.x - location.x,
                    y: target_location.y - location.y
                };
                let mut dummy_move_intent = MoveIntent::Vector(distance_vector, *speed);
                dummy_move_intent.move_from(location)
            },
            MoveIntent::Vector(movement_vec, speed) => {
                let step = *speed / UPDATES_PER_SECOND as f64; // + 0.1f64.powf(PRECISION * 2.0);

                // get the direction of axis we will move towards
                let x_direction = if movement_vec.x == 0.0 { 0.0 } else {movement_vec.x / movement_vec.x.abs()};
                let y_direction = if movement_vec.y == 0.0 { 0.0 } else {movement_vec.y / movement_vec.y.abs()};

                // amount we will move in said direction
                let mut x_step = movement_vec.x.abs() / (movement_vec.x.abs() + movement_vec.y.abs()) * step;
                let mut y_step = movement_vec.y.abs() / (movement_vec.x.abs() + movement_vec.y.abs()) * step;

                // do not move to far, if a step is bigger than distance left
                x_step = x_step.min(movement_vec.x.abs());
                y_step = y_step.min(movement_vec.y.abs());

                *self = MoveIntent::Vector(LocationVec {
                    x: movement_vec.x - x_step * x_direction,
                    y: movement_vec.y - y_step * y_direction,
                }, *speed);


                // check if truncating is required
                let mut new_x = location.x + x_step * x_direction;
                if new_x.abs() - new_x.trunc().abs() < PRECISION && new_x > new_x.trunc() {
                    new_x = new_x.round();
                } else if new_x.abs() - (new_x + PRECISION).trunc().abs() < PRECISION{
                    new_x = new_x.round();
                }

                let mut new_y = location.y + y_step * y_direction;
                if new_y.abs() - new_y.trunc().abs() < PRECISION && new_y > new_y.trunc() {
                    new_y = new_y.round();
                } else if new_y.abs() - (new_y + PRECISION).trunc().abs() < PRECISION{
                    new_y = new_y.round();
                }

                LocationVec {
                    x: new_x,
                    y: new_y
                }
            }
        }
    }

    // if goal is reached
    pub fn has_arrived(&self, start_location: &LocationVec) -> bool {
        match self {
            MoveIntent::Position(target_location, _) => {
                start_location == target_location
            },
            MoveIntent::Vector(movement_vec, _) => {
                movement_vec.x == 0.0 && movement_vec.y == 0.0
            }
        }
    }

    // gives LocationVec of the goal
    pub fn target_goal(&self, start_location: &LocationVec) -> LocationVec {
        match self {
            MoveIntent::Position(target_location, _) => *target_location,
            MoveIntent::Vector(movement_vec, _) => LocationVec {
                x: start_location.x + movement_vec.x,
                y: start_location.y + movement_vec.y
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LocationVec, DEFAULT_SPEED, UPDATES_PER_SECOND, MoveIntent};
    const ZERO_VEC: LocationVec = LocationVec {x: 0.0, y: 0.0};

    #[test]
    fn arrived_vector() {
        assert!(MoveIntent::Vector(ZERO_VEC, 1.0).has_arrived(&ZERO_VEC));
        assert!(!MoveIntent::Vector(LocationVec{x:1.0, y:0.0}, 1.0).has_arrived(&ZERO_VEC));
    }

    #[test]
    fn arrived_position() {
        assert!(MoveIntent::Position(ZERO_VEC, 1.0).has_arrived(&ZERO_VEC));
        assert!(MoveIntent::Position(LocationVec{x:2.0, y:1.0}, 1.0).has_arrived(&LocationVec{x:2.0, y:1.0}));
        assert!(!MoveIntent::Position(LocationVec{x:1.0, y:2.0}, 1.0).has_arrived(&LocationVec{x:2.0,y:1.0}));
    }

    #[test]
    fn move_vector() {
        let mut intent = MoveIntent::Vector(LocationVec{x:1.0, y: 0.0}, 1.0);
        let mut location = ZERO_VEC;
        for _ in 0..UPDATES_PER_SECOND {
            location = intent.move_from(&location);
            print!("\n{:?}\n", &location);
            print!("{:?}\n\n", &intent);
        }
        print!("{:?}", &location);
        assert!(intent.has_arrived(&location));
    }
}