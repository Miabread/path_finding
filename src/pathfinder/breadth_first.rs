use std::collections::VecDeque;

use crate::tile::Tile;

use super::Algorithm;

#[derive(Debug, Default)]
pub struct BreadthFirst {
    queue: VecDeque<Tile>,
}

impl Algorithm for BreadthFirst {
    fn insert(&mut self, tile: Tile) {
        self.queue.push_back(tile);
    }

    fn next(&mut self) -> Option<Tile> {
        self.queue.pop_front()
    }
}
