extern crate serde;

use serde::{Serialize, Deserialize};
use crate::gamestate::LocationVec;
use crate::UPDATES_PER_SECOND;

#[derive(Eq, PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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
                let step = *speed / UPDATES_PER_SECOND as f64;

                // get the direction of axis we will move towards
                let x_direction = movement_vec.x / movement_vec.x.abs();
                let y_direction = movement_vec.y / movement_vec.y.abs();

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

                LocationVec {
                    x: location.x + x_step * x_direction,
                    y: location.y + y_step * y_direction
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