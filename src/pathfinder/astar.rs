use std::collections::BinaryHeap;

use crate::tile::Tile;

use super::Algorithm;

#[derive(Debug, Default)]
pub struct AStar {
    queue: BinaryHeap<Tile>,
}

impl Algorithm for AStar {
    fn insert(&mut self, tile: Tile) {
        self.queue.push(tile);
    }

    fn next(&mut self) -> Option<Tile> {
        self.queue.pop()
    }
}
