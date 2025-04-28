use crate::tile::Tile;

use super::Algorithm;

#[derive(Debug, Default)]
pub struct DepthFirst {
    queue: Vec<Tile>,
}

impl Algorithm for DepthFirst {
    fn insert(&mut self, tile: Tile) {
        self.queue.push(tile);
    }

    fn next(&mut self) -> Option<Tile> {
        self.queue.pop()
    }
}
