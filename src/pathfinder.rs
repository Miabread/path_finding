use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use rand::seq::IteratorRandom;
use std::{collections::HashSet, ops::ControlFlow};

use crate::{
    TilePrev, TileState,
    algorithm::{Algorithm, AlgorithmOption},
    tile::Tile,
};

pub fn pathfinder_plugin(app: &mut App) {
    app.init_resource::<Pathfinder>()
        .add_systems(Update, update_endpoints);
}

fn update_endpoints(
    mut tile_q: Query<(&TileState, &TilePos), Changed<TileState>>,
    mut pathfinder: ResMut<Pathfinder>,
) {
    for (state, &pos) in tile_q.iter_mut() {
        if pathfinder.start.remove(&Tile::zero(pos)) {
            debug!("removed start tile {}", Tile::zero(pos));
        }

        if pathfinder.goals.remove(&Tile::zero(pos)) {
            debug!("removed goal tile {}", Tile::zero(pos));
        }

        match state {
            TileState::Start => {
                debug!("added start tile {}", Tile::zero(pos));
                pathfinder.start.insert(Tile::zero(pos));
            }

            TileState::Goal => {
                debug!("added goal tile {}", Tile::zero(pos));
                pathfinder.goals.insert(Tile::zero(pos));
            }

            _ => {}
        }
    }
}

#[derive(Resource)]
pub struct Pathfinder {
    algorithm: Box<dyn Algorithm + Sync + Send>,
    visited: HashSet<Tile>,

    start: HashSet<Tile>,
    goals: HashSet<Tile>,

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
        storage: &TileStorage,
        mut states: Query<&mut TileState>,
        mut prevs: Query<&mut TilePrev>,
    ) {
        if self.complete {
            return;
        }

        debug!("----- pathfinder step start = {} -----", self.step);

        if self.visited.is_empty() {
            if let Some(&start) = self.start.iter().choose(&mut rand::rng()) {
                debug!("selected start tile {}", start);
                self.algorithm.insert(start);
                self.visited.insert(start);
            } else {
                debug!("no start tiles to select from");
            }
        }

        if let ControlFlow::Break(mut last_pos) =
            self.step_internal(storage, states.reborrow(), prevs.reborrow())
        {
            self.complete = true;

            while let Some(current_last_pos) = last_pos {
                let Some(entity) = storage.checked_get(&current_last_pos) else {
                    last_pos = None;
                    continue;
                };

                let mut state = states.get_mut(entity).unwrap();

                if let TileState::Visited(distance) = *state {
                    *state = TileState::Final(distance);
                }

                last_pos = prevs.get_mut(entity).unwrap().0;
            }
        }

        debug!("----- pathfinder step done = {} -----", self.step);
        self.step += 1;
    }

    fn step_internal(
        &mut self,
        storage: &TileStorage,
        mut states: Query<&mut TileState>,
        mut prevs: Query<&mut TilePrev>,
    ) -> ControlFlow<Option<TilePos>> {
        let Some(tile) = self.algorithm.next() else {
            debug!("no more tiles in queue");
            return ControlFlow::Break(None);
        };

        debug!("stepping on tile {}", tile);

        if self.goals.contains(&tile) {
            debug!("reached goal {}", tile);
            return ControlFlow::Break(Some(tile.pos));
        }

        for neighbor in tile.neighbors(&self.goals) {
            if self.visited.contains(&neighbor) {
                debug!("neighbor skip {}", neighbor);
                continue;
            }

            self.visited.insert(neighbor);

            let Some(entity) = storage.checked_get(&neighbor.pos) else {
                debug!("neighbor bounds {}", neighbor);
                continue;
            };

            let mut neighbor_state = states.get_mut(entity).unwrap();

            if *neighbor_state == TileState::Wall {
                debug!("neighbor wall {}", neighbor);
                continue;
            }

            *prevs.get_mut(entity).unwrap() = TilePrev(Some(tile.pos));

            debug!("neighbor queue {}", neighbor);

            self.algorithm.insert(neighbor);

            if *neighbor_state == TileState::Empty {
                *neighbor_state = TileState::Queued;
            }
        }

        let mut tile_state = states
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
            start: Default::default(),
            goals: Default::default(),
            step: Default::default(),
            complete: Default::default(),
        }
    }
}
