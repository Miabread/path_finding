use bevy_ecs_tilemap::tiles::TilePos;
use std::collections::HashSet;

use super::Algorithm;

#[derive(Debug, Default)]
pub struct DepthFirst {
    queue: Vec<TilePos>,
}

impl Algorithm for DepthFirst {
    fn insert(&mut self, tile: TilePos, _goals: &HashSet<TilePos>) {
        self.queue.push(tile);
    }

    fn next(&mut self) -> Option<TilePos> {
        self.queue.pop()
    }
}
