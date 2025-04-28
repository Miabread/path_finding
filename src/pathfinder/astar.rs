use bevy_ecs_tilemap::tiles::TilePos;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use super::Algorithm;

#[derive(Debug, Default)]
pub struct AStar {
    queue: BinaryHeap<AStarTile>,
}

impl Algorithm for AStar {
    fn insert(&mut self, tile: TilePos, goals: &HashSet<TilePos>) {
        let distance = goals
            .iter()
            .map(|&goal| distance(tile, goal).abs() as u32)
            .min()
            .unwrap_or(0);

        self.queue.push(AStarTile { tile, distance });
    }

    fn next(&mut self) -> Option<TilePos> {
        self.queue.pop().map(|it| it.tile)
    }
}

fn distance(a: TilePos, b: TilePos) -> i32 {
    let x_diff = b.x as i32 - a.x as i32;
    let y_diff = b.y as i32 - a.y as i32;
    (x_diff.pow(2) + y_diff.pow(2)).isqrt()
}

#[derive(Debug, Clone, Copy)]
struct AStarTile {
    tile: TilePos,
    distance: u32,
}

impl PartialEq for AStarTile {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for AStarTile {}

impl PartialOrd for AStarTile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Reverse(self.distance).partial_cmp(&Reverse(other.distance))
    }
}

impl Ord for AStarTile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Reverse(self.distance).cmp(&Reverse(other.distance))
    }
}
