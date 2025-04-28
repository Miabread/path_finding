use bevy_ecs_tilemap::tiles::TilePos;
use std::collections::{HashSet, VecDeque};

use super::Algorithm;

#[derive(Debug, Default)]
pub struct BreadthFirst {
    queue: VecDeque<TilePos>,
}

impl Algorithm for BreadthFirst {
    fn insert(&mut self, tile: TilePos, _goals: &HashSet<TilePos>) {
        self.queue.push_back(tile);
    }

    fn next(&mut self) -> Option<TilePos> {
        self.queue.pop_front()
    }
}
