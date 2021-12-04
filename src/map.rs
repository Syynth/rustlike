use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for (i, tile) in self.tiles.iter().enumerate() {
            let x = i as i32 % SCREEN_WIDTH;
            let y = i as i32 / SCREEN_WIDTH;

            match tile {
                TileType::Floor => ctx.set(x, y, YELLOW, BLACK, to_cp437('.')),
                TileType::Wall => ctx.set(x, y, GREEN, BLACK, to_cp437('#')),
            }
        }
    }

    pub fn tick(&mut self, ctx: &mut BTerm) {}

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            return None;
        }
        Some(map_idx(point.x, point.y) as usize)
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    (y as usize * SCREEN_WIDTH as usize) + x as usize
}