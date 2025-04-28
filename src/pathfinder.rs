mod astar;
mod breadth_first;
mod depth_first;
mod random;

use astar::AStar;
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use breadth_first::BreadthFirst;
use depth_first::DepthFirst;
use rand::seq::IteratorRandom;
use random::Random;
use std::{collections::HashSet, ops::ControlFlow};

use crate::{TileState, tile::Tile};

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

            TileState::End => {
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

    pub fn step(&mut self, storage: &TileStorage, tiles: Query<&mut TileState>) {
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

        if self.step_internal(storage, tiles).is_break() {
            self.complete = true;
        }

        debug!("----- pathfinder step done = {} -----", self.step);
        self.step += 1;
    }

    fn step_internal(
        &mut self,
        storage: &TileStorage,
        mut tiles: Query<&mut TileState>,
    ) -> ControlFlow<()> {
        let Some(tile) = self.algorithm.next() else {
            debug!("no more tiles in queue");
            return ControlFlow::Break(());
        };

        debug!("stepping on tile {}", tile);

        if self.goals.contains(&tile) {
            debug!("reached goal {}", tile);
            return ControlFlow::Break(());
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

            let mut neighbor_state = tiles.get_mut(entity).unwrap();

            if *neighbor_state == TileState::Wall {
                debug!("neighbor wall {}", neighbor);
                continue;
            }

            debug!("neighbor queue {}", neighbor);

            self.algorithm.insert(neighbor);

            if *neighbor_state == TileState::Empty {
                *neighbor_state = TileState::Queued;
            }
        }

        let mut tile_state = tiles
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlgorithmOption {
    #[default]
    BreadthFirst,
    AStar,
    DepthFirst,
    Random,
}

impl From<AlgorithmOption> for Box<dyn Algorithm + Send + Sync> {
    fn from(value: AlgorithmOption) -> Self {
        match value {
            AlgorithmOption::BreadthFirst => Box::new(BreadthFirst::default()),
            AlgorithmOption::AStar => Box::new(AStar::default()),
            AlgorithmOption::DepthFirst => Box::new(DepthFirst::default()),
            AlgorithmOption::Random => Box::new(Random::default()),
        }
    }
}

trait Algorithm {
    fn insert(&mut self, tile: Tile);
    fn next(&mut self) -> Option<Tile>;
}
