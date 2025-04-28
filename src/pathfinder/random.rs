use rand::{rng, seq::IteratorRandom};

use crate::tile::Tile;

use super::Algorithm;

#[derive(Debug, Default)]
pub struct Random {
    queue: Vec<Tile>,
}

impl Algorithm for Random {
    fn insert(&mut self, tile: Tile) {
        self.queue.push(tile);
    }

    fn next(&mut self) -> Option<Tile> {
        let (i, &tile) = self.queue.iter().enumerate().choose(&mut rng())?;
        self.queue.remove(i);
        Some(tile)
    }
}
