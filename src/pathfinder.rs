use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use rand::seq::IteratorRandom;
use std::{collections::HashSet, ops::ControlFlow};

use crate::{
    TileParent, TileState,
    algorithm::{Algorithm, AlgorithmOption},
    tile::Tile,
};

pub fn pathfinder_plugin(app: &mut App) {
    app.init_resource::<Pathfinder>()
        .add_systems(Update, update_endpoints);
}

fn update_endpoints(
    mut tiles_query: Query<(&TileState, &TilePos), Changed<TileState>>,
    mut pathfinder: ResMut<Pathfinder>,
) {
    for (state, &pos) in tiles_query.iter_mut() {
        if pathfinder.start_tiles.remove(&Tile::zero(pos)) {
            debug!("removed start tile {}", Tile::zero(pos));
        }

        if pathfinder.goal_tiles.remove(&Tile::zero(pos)) {
            debug!("removed goal tile {}", Tile::zero(pos));
        }

        match state {
            TileState::Start => {
                debug!("added start tile {}", Tile::zero(pos));
                pathfinder.start_tiles.insert(Tile::zero(pos));
            }

            TileState::Goal => {
                debug!("added goal tile {}", Tile::zero(pos));
                pathfinder.goal_tiles.insert(Tile::zero(pos));
            }

            _ => {}
        }
    }
}

#[derive(Resource)]
pub struct Pathfinder {
    algorithm: Box<dyn Algorithm + Sync + Send>,
    visited: HashSet<Tile>,

    start_tiles: HashSet<Tile>,
    goal_tiles: HashSet<Tile>,

    pub step: usize,
    pub complete: bool,
}

impl Pathfinder {
    pub fn restart(&mut self, algorithm: AlgorithmOption) {
        self.algorithm = algorithm.into();
        self.visited.clear();

        self.step = 0;
        self.complete = false;
    }

    pub fn stop(&mut self, algorithm: AlgorithmOption) {
        self.restart(algorithm);
        self.complete = true;
    }

    pub fn step(
        &mut self,
        tile_storage: &TileStorage,
        mut tile_states: Query<&mut TileState>,
        mut tile_parents: Query<&mut TileParent>,
    ) {
        if self.complete {
            return;
        }

        debug!("----- pathfinder step start = {} -----", self.step);

        if self.visited.is_empty() {
            if let Some(&start_tile) = self.start_tiles.iter().choose(&mut rand::rng()) {
                debug!("selected start tile {}", start_tile);
                self.algorithm.insert(start_tile);
                self.visited.insert(start_tile);
            } else {
                debug!("no start tiles to select from");
            }
        }

        if let ControlFlow::Break(mut next_pos) = self.step_internal(
            tile_storage,
            tile_states.reborrow(),
            tile_parents.reborrow(),
        ) {
            self.complete = true;

            while let Some(current_pos) = next_pos {
                let Some(entity) = tile_storage.checked_get(&current_pos) else {
                    next_pos = None;
                    continue;
                };

                let mut tile_state = tile_states.get_mut(entity).unwrap();

                if let TileState::Visited(distance) = *tile_state {
                    *tile_state = TileState::Final(distance);
                }

                next_pos = tile_parents.get_mut(entity).unwrap().0;
            }
        }

        debug!("----- pathfinder step done = {} -----", self.step);
        self.step += 1;
    }

    fn step_internal(
        &mut self,
        storage: &TileStorage,
        mut tile_states: Query<&mut TileState>,
        mut tile_parents: Query<&mut TileParent>,
    ) -> ControlFlow<Option<TilePos>> {
        let Some(tile) = self.algorithm.next() else {
            debug!("no more tiles in queue");
            return ControlFlow::Break(None);
        };

        debug!("stepping on tile {}", tile);

        if self.goal_tiles.contains(&tile) {
            debug!("reached goal {}", tile);
            return ControlFlow::Break(Some(tile.pos));
        }

        for neighbor in tile.neighbors(&self.goal_tiles) {
            if self.visited.contains(&neighbor) {
                debug!("neighbor skip {}", neighbor);
                continue;
            }

            self.visited.insert(neighbor);

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

            debug!("neighbor queue {}", neighbor);

            self.algorithm.insert(neighbor);

            if *neighbor_state == TileState::Empty {
                *neighbor_state = TileState::Queued;
            }
        }

        let mut tile_state = tile_states
            .get_mut(storage.checked_get(&tile.pos).unwrap())
            .unwrap();

        if *tile_state == TileState::Queued {
            *tile_state = TileState::Visited(tile.distance);
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
