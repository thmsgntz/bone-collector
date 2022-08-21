use crate::Vec3;
use std::f32::consts::PI;

const ENTITY_LOOKING_UP: f32 = 45.0 * PI / 180.0;
const ENTITY_LOOKING_UP_RIGHT: f32 = PI / 180.0;
const ENTITY_LOOKING_RIGHT: f32 = -45.0 * PI / 180.0;
const ENTITY_LOOKING_UP_LEFT: f32 = (2.0 * 45.0) * PI / 180.0;
const ENTITY_LOOKING_LEFT: f32 = (3.0 * 45.0) * PI / 180.0;
const ENTITY_LOOKING_DOWN: f32 = -(3.0 * 45.0) * PI / 180.0;
const ENTITY_LOOKING_DOWN_LEFT: f32 = -(4.0 * 45.0) * PI / 180.0;
const ENTITY_LOOKING_DOWN_RIGHT: f32 = -(2.0 * 45.0) * PI / 180.0;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub fn get_angle(&self) -> f32 {
        match self {
            Direction::Up => ENTITY_LOOKING_UP,
            Direction::UpRight => ENTITY_LOOKING_UP_RIGHT,
            Direction::Right => ENTITY_LOOKING_RIGHT,
            Direction::DownRight => ENTITY_LOOKING_DOWN_RIGHT,
            Direction::Down => ENTITY_LOOKING_DOWN,
            Direction::DownLeft => ENTITY_LOOKING_DOWN_LEFT,
            Direction::Left => ENTITY_LOOKING_LEFT,
            Direction::UpLeft => ENTITY_LOOKING_UP_LEFT,
        }
    }

    pub fn get_vec3(&self) -> Vec3 {
        match self {
            Direction::Up => Vec3::new(1.0, 0.0, 1.0),
            Direction::UpRight => Vec3::new(0.0, 0.0, 1.0),
            Direction::Right => Vec3::new(-1.0, 0.0, 1.0),
            Direction::DownRight => Vec3::new(-1.0, 0.0, 0.0),
            Direction::Down => Vec3::new(-1.0, 0.0, -1.0),
            Direction::DownLeft => Vec3::new(0.0, 0.0, -1.0),
            Direction::Left => Vec3::new(1.0, 0.0, -1.0),
            Direction::UpLeft => Vec3::new(1.0, 0.0, 0.0),
        }
    }
}

pub fn map_vec3_to_direction(vec: Vec3) -> Result<Direction, String> {
    match vec.x as i8 {
        0 => match vec.z as i8 {
            1 | 2 => Ok(Direction::UpRight),
            -1 | -2 => Ok(Direction::DownLeft),
            _ => Err(String::from("vector_direction not equals to 1 or 2")),
        },
        1 | 2 => match vec.z as i8 {
            1 | 2 => Ok(Direction::Up),
            -1 | -2 => Ok(Direction::Left),
            0 => Ok(Direction::UpLeft),
            _ => Err(String::from("vector_direction not equals to 1 or 2")),
        },
        -1 | -2 => match vec.z as i8 {
            1 | 2 => Ok(Direction::Right),
            -1 | -2 => Ok(Direction::Down),
            0 => Ok(Direction::DownRight),
            _ => Err(String::from("vector_direction not equals to 1 or 2")),
        },
        _ => Err(String::from("vector_direction not equals to 1 or 2")),
    }
}
