use bevy_ecs_tilemap::tiles::TilePos;
use rand::{rng, seq::IteratorRandom};
use std::collections::HashSet;

use super::Algorithm;

#[derive(Debug, Default)]
pub struct Random {
    queue: Vec<TilePos>,
}

impl Algorithm for Random {
    fn insert(&mut self, tile: TilePos, _goals: &HashSet<TilePos>) {
        self.queue.push(tile);
    }

    fn next(&mut self) -> Option<TilePos> {
        let (i, &tile) = self.queue.iter().enumerate().choose(&mut rng())?;
        self.queue.remove(i);
        Some(tile)
    }
}
