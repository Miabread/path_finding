use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use std::{collections::HashSet, ops::ControlFlow};

use crate::TileState;

use super::{neighbors, Algorithm};

#[derive(Debug, Default)]
pub struct DepthFirst {
    queue: Vec<TilePos>,
    visited: HashSet<TilePos>,
}

impl Algorithm for DepthFirst {
    fn start(&mut self, start: TilePos) {
        self.queue.push(start);
        self.visited.insert(start);
    }

    fn step(
        &mut self,
        goals: &HashSet<TilePos>,
        storage: &TileStorage,
        mut tiles: Query<&mut TileState>,
    ) -> ControlFlow<()> {
        let Some(tile) = self.queue.pop() else {
            return ControlFlow::Break(());
        };

        if goals.contains(&tile) {
            return ControlFlow::Break(());
        }

        for neighbor in neighbors(tile) {
            if self.visited.contains(&neighbor) {
                continue;
            }

            self.visited.insert(neighbor);

            let Some(entity) = storage.checked_get(&neighbor) else {
                continue;
            };

            let mut neighbor_state = tiles.get_mut(entity).unwrap();

            if *neighbor_state == TileState::Wall {
                continue;
            }

            self.queue.push(neighbor);

            neighbor_state.change_from(TileState::Empty, TileState::Queued);
        }

        tiles
            .get_mut(storage.checked_get(&tile).unwrap())
            .unwrap()
            .change_from(TileState::Queued, TileState::Visited);

        ControlFlow::Continue(())
    }
}
