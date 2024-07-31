use ratatui::layout::Positions;
use ratatui::style::Color;
use specs::Component;
use specs::DenseVecStorage;
use specs_derive::Component;
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Component, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Component)]
pub struct TrainHead {
    pub position: Point,
}

#[derive(Component)]
pub struct TrainParts {
    pub parts: Vec<Point>,
}

#[derive(Component)]
pub struct TrainColors {
    pub main_color: Color,
    pub head_color: Color,
}

#[derive(Component)]
pub struct TrainParameters {
    pub mass: f32,
    pub velocity: f32,
    pub acceleration: f32,
    pub force: f32,
    pub movement_direction: Direction,
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

impl From<Direction> for Point {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Point::new(0, -1),
            Direction::Right => Point::new(1, 0),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
        }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
