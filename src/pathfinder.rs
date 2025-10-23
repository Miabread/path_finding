use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use rand::seq::IteratorRandom;
use std::{collections::HashSet, ops::ControlFlow};

use crate::{
    TileParent, TileState,
    algorithm::{Algorithm, AlgorithmOption},
    pathfinder_tile::PathfinderTile,
};

pub fn pathfinder_plugin(app: &mut App) {
    app.init_resource::<Pathfinder>()
        .add_systems(Update, update_endpoints);
}

/**
 * This system watches all TileStates, and update's the pathfinder's internal endpoints list with any added/removed endpoint tiles
 */
fn update_endpoints(
    mut tiles_query: Query<(&TileState, &TilePos), Changed<TileState>>,
    mut pathfinder: ResMut<Pathfinder>,
) {
    for (state, &pos) in tiles_query.iter_mut() {
        if pathfinder.start_tiles.remove(&PathfinderTile::zero(pos)) {
            debug!("removed start tile {}", PathfinderTile::zero(pos));
        }

        if pathfinder.goal_tiles.remove(&PathfinderTile::zero(pos)) {
            debug!("removed goal tile {}", PathfinderTile::zero(pos));
        }

        match state {
            TileState::Start => {
                debug!("added start tile {}", PathfinderTile::zero(pos));
                pathfinder.start_tiles.insert(PathfinderTile::zero(pos));
            }

            TileState::Goal => {
                debug!("added goal tile {}", PathfinderTile::zero(pos));
                pathfinder.goal_tiles.insert(PathfinderTile::zero(pos));
            }

            _ => {}
        }
    }
}

#[derive(Resource)]
pub struct Pathfinder {
    // Used to do the actual path finding
    algorithm: Box<dyn Algorithm + Sync + Send>,
    visited: HashSet<PathfinderTile>,

    // Updated by update_endpoints system
    start_tiles: HashSet<PathfinderTile>,
    goal_tiles: HashSet<PathfinderTile>,

    // Bookkeeping for UI
    pub step: usize,
    pub complete: bool,
}

impl Pathfinder {
    /*
     *  Reset pathfinder with automatically starting
     */
    pub fn restart(&mut self, algorithm: AlgorithmOption) {
        self.algorithm = algorithm.into();
        self.visited.clear();

        self.step = 0;
        self.complete = false;
    }

    /**
     * Reset pathfinder without automatically starting
     */
    pub fn stop(&mut self, algorithm: AlgorithmOption) {
        self.restart(algorithm);
        self.complete = true;
    }

    /**
     * Perform a loop of the pathfinder
     */
    pub fn step(
        &mut self,
        tile_storage: &TileStorage,
        mut tile_states: Query<&mut TileState>,
        mut tile_parents: Query<&mut TileParent>,
    ) {
        // If marked as complete, don't do any more steps
        if self.complete {
            return;
        }

        debug!("----- pathfinder step start = {} -----", self.step);

        // We're not complete and have an empty queue, meaning we haven't started yet
        // So pick a random starting tile and queue it
        if self.visited.is_empty() {
            if let Some(&start_tile) = self.start_tiles.iter().choose(&mut rand::rng()) {
                debug!("selected start tile {}", start_tile);
                self.algorithm.insert(start_tile);
                self.visited.insert(start_tile);
            } else {
                debug!("no start tiles to select from");
            }
        }

        // Keep stepping until we get told to stop
        if let ControlFlow::Break(mut next_pos) = self.step_internal(
            tile_storage,
            tile_states.reborrow(),
            tile_parents.reborrow(),
        ) {
            // Don't step anymore after this
            self.complete = true;

            // If we are given a goal position back, try to follow the parent chain and mark them, filling out the full found path
            while let Some(current_pos) = next_pos {
                let Some(entity) = tile_storage.checked_get(&current_pos) else {
                    next_pos = None;
                    continue;
                };

                let mut tile_state = tile_states.get_mut(entity).unwrap();

                if let TileState::Visited(distance) = *tile_state {
                    *tile_state = TileState::Final(distance);
                }

                // Loop to next parent
                next_pos = tile_parents.get_mut(entity).unwrap().0;
            }
        }

        debug!("----- pathfinder step done = {} -----", self.step);
        self.step += 1;
    }

    /**
     * Consider this the "loop body" of the pathfinder code
     * It does the bulk of the computation and then decides to break or continue
     */
    fn step_internal(
        &mut self,
        storage: &TileStorage,
        mut tile_states: Query<&mut TileState>,
        mut tile_parents: Query<&mut TileParent>,
    ) -> ControlFlow<Option<TilePos>> {
        // Ran out of tiles in the queue, break without a found path
        let Some(tile) = self.algorithm.next() else {
            debug!("no more tiles in queue");
            return ControlFlow::Break(None);
        };

        debug!("stepping on tile {}", tile);

        // Hit a goal tile, break with a found path
        if self.goal_tiles.contains(&tile) {
            debug!("reached goal {}", tile);
            return ControlFlow::Break(Some(tile.pos));
        }

        for neighbor in tile.neighbors(&self.goal_tiles) {
            // Don't requeue tiles we've already visited
            if self.visited.contains(&neighbor) {
                debug!("neighbor skip {}", neighbor);
                continue;
            }
            self.visited.insert(neighbor);

            // Get corresponding tile entity to do bookkeeping
            let Some(entity) = storage.checked_get(&neighbor.pos) else {
                debug!("neighbor bounds {}", neighbor);
                continue;
            };

            let mut neighbor_state = tile_states.get_mut(entity).unwrap();

            if *neighbor_state == TileState::Wall {
                debug!("neighbor wall {}", neighbor);
                continue;
            }

            *tile_parents.get_mut(entity).unwrap() = TileParent(Some(tile.pos));

            // Finally enqueue the neighbor tile
            debug!("neighbor queue {}", neighbor);
            self.algorithm.insert(neighbor);

            if *neighbor_state == TileState::Empty {
                *neighbor_state = TileState::Queued(neighbor.distance);
            }
        }

        // Finally finish bookkeeping on now visited tile
        let mut tile_state = tile_states
            .get_mut(storage.checked_get(&tile.pos).unwrap())
            .unwrap();

        if let TileState::Queued(distance) = *tile_state {
            *tile_state = TileState::Visited(distance);
        }

        ControlFlow::Continue(())
    }
}

impl Default for Pathfinder {
    fn default() -> Self {
        Self {
            algorithm: AlgorithmOption::default().into(),
            visited: Default::default(),
            start_tiles: Default::default(),
            goal_tiles: Default::default(),
            step: Default::default(),
            complete: Default::default(),
        }
    }
}
