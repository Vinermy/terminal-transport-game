use crate::components::{Direction, Point};
use crate::map_tile::{MapTile, RailShape};
use std::cmp::{max, min};

pub struct Map {
    width: usize,
    height: usize,
    tiles: Vec<MapTile>,
}

impl Map {
    pub fn empty(w: i32, h: i32) -> Self {
        Self {
            width: w as usize,
            height: h as usize,
            tiles: vec![MapTile::from(RailShape::Empty); (w * h) as usize],
        }
    }
    pub fn simple_ring(w: i32, h: i32) -> Self {
        let mut map = Self::empty(w, h);

        map.put_tile_at_xy(Point::new(0, 0), RailShape::TurnBottomRight);
        map.put_tile_at_xy(Point::new(w - 1, 0), RailShape::TurnBottomLeft);
        map.put_tile_at_xy(Point::new(0, h - 1), RailShape::TurnTopRight);
        map.put_tile_at_xy(Point::new(w - 1, h - 1), RailShape::TurnTopLeft);

        map.put_horizontal_line(1, w - 2, 0);
        map.put_horizontal_line(1, w - 2, h - 1);
        map.put_vertical_line(1, h - 2, 0);
        map.put_vertical_line(1, h - 2, w - 1);

        map
    }

    pub fn xy_idx(&self, coords: Point) -> usize {
        ((coords.y as usize) * self.width) + (coords.x as usize)
    }

    pub fn get_tile_at_xy(&self, coords: Point) -> MapTile {
        self.tiles[self.xy_idx(coords)]
    }

    fn put_tile_at_xy(&mut self, coords: Point, shape: RailShape) {
        let i = self.xy_idx(coords);
        self.tiles[i] = MapTile::from(shape);
    }

    fn put_horizontal_line(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            self.put_tile_at_xy(Point::new(x, y), RailShape::Horizontal);
        }
    }

    fn put_vertical_line(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            self.put_tile_at_xy(Point::new(x, y), RailShape::Vertical);
        }
    }

    pub fn w(&self) -> i32 {
        self.width as i32
    }

    pub fn h(&self) -> i32 {
        self.height as i32
    }

    pub fn get_next_direction(&self, pos: Point, direction: Direction) -> Direction {
        let shape = self.get_tile_at_xy(pos).shape();
        match (shape, direction) {
            (RailShape::Horizontal, Direction::Right) => Direction::Right,
            (RailShape::Horizontal, Direction::Left) => Direction::Left,

            (RailShape::Vertical, Direction::Up) => Direction::Up,
            (RailShape::Vertical, Direction::Down) => Direction::Down,

            (RailShape::TurnTopLeft, Direction::Down) => Direction::Left,
            (RailShape::TurnTopLeft, Direction::Right) => Direction::Up,

            (RailShape::TurnTopRight, Direction::Down) => Direction::Right,
            (RailShape::TurnTopRight, Direction::Left) => Direction::Up,

            (RailShape::TurnBottomRight, Direction::Up) => Direction::Right,
            (RailShape::TurnBottomRight, Direction::Left) => Direction::Down,

            (RailShape::TurnBottomLeft, Direction::Up) => Direction::Left,
            (RailShape::TurnBottomLeft, Direction::Right) => Direction::Down,

            (_, _) => panic!(
                "Incorrect travel direction and rail shape combination: {:?}, {:?}",
                shape, direction
            ),
        }
    }
}
