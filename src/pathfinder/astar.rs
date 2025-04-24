use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    ops::ControlFlow,
};

use crate::TileState;

use super::{neighbors, Algorithm};

#[derive(Debug, Clone, Copy)]
struct AStarTilePos {
    pos: TilePos,
    distance: u32,
}

impl AStarTilePos {
    fn new(pos: TilePos, goals: &HashSet<TilePos>) -> Self {
        let distance = goals
            .iter()
            .map(|goal| {
                ((pos.x as i32 - goal.x as i32).pow(2) + (pos.y as i32 - goal.y as i32).pow(2))
                    .isqrt()
                    .abs() as u32
            })
            .min()
            .unwrap_or(0);

        AStarTilePos { pos, distance }
    }
}

impl PartialEq for AStarTilePos {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for AStarTilePos {}

impl PartialOrd for AStarTilePos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Reverse(self.distance).partial_cmp(&Reverse(other.distance))
    }
}

impl Ord for AStarTilePos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Reverse(self.distance).cmp(&Reverse(other.distance))
    }
}

#[derive(Debug, Default)]
pub struct AStar {
    queue: BinaryHeap<AStarTilePos>,
    visited: HashSet<TilePos>,
}

impl Algorithm for AStar {
    fn start(&mut self, start: TilePos) {
        self.queue.push(AStarTilePos {
            pos: start,
            distance: 0,
        });
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

        if goals.contains(&tile.pos) {
            return ControlFlow::Break(());
        }

        for neighbor in neighbors(tile.pos) {
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

            self.queue.push(AStarTilePos::new(neighbor, goals));

            neighbor_state.change_from(TileState::Empty, TileState::Queued);
        }

        tiles
            .get_mut(storage.checked_get(&tile.pos).unwrap())
            .unwrap()
            .change_from(TileState::Queued, TileState::Visited);

        ControlFlow::Continue(())
    }
}
