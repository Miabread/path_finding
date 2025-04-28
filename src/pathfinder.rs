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

use crate::TileState;

pub fn pathfinder_plugin(app: &mut App) {
    app.init_resource::<Pathfinder>()
        .add_systems(Update, update_endpoints);
}

fn update_endpoints(
    mut tile_q: Query<(&TileState, &TilePos), Changed<TileState>>,
    mut pathfinder: ResMut<Pathfinder>,
) {
    for (state, pos) in tile_q.iter_mut() {
        pathfinder.start.remove(pos);
        pathfinder.goals.remove(pos);

        match state {
            TileState::Start => {
                pathfinder.start.insert(*pos);
            }

            TileState::End => {
                pathfinder.goals.insert(*pos);
            }

            _ => {}
        }
    }
}

#[derive(Resource)]
pub struct Pathfinder {
    algorithm: Box<dyn Algorithm + Sync + Send>,
    visited: HashSet<TilePos>,

    start: HashSet<TilePos>,
    goals: HashSet<TilePos>,

    step: usize,
    complete: bool,
}

impl Pathfinder {
    pub fn restart(&mut self, algorithm: AlgorithmOption) {
        self.algorithm = algorithm.into();
        self.visited.clear();

        self.step = 0;
        self.complete = false;

        if let Some(&start) = self.start.iter().choose(&mut rand::rng()) {
            self.algorithm.insert(start, &self.goals);
            self.visited.insert(start);
        }
    }

    pub fn step(&mut self, storage: &TileStorage, tiles: Query<&mut TileState>) {
        if self.complete {
            return;
        }

        info!("pathfinder step = {}", self.step);
        self.step += 1;

        if step(
            &mut self.algorithm,
            &mut self.visited,
            &self.goals,
            storage,
            tiles,
        )
        .is_break()
        {
            self.complete = true;
        }
    }
}

impl FromWorld for Pathfinder {
    fn from_world(_world: &mut World) -> Self {
        let mut pathfinder = Self {
            algorithm: AlgorithmOption::default().into(),
            visited: HashSet::new(),

            start: HashSet::new(),
            goals: HashSet::new(),

            step: 0,
            complete: false,
        };
        pathfinder.restart(AlgorithmOption::default().into());
        pathfinder
    }
}

fn step(
    algorithm: &mut Box<dyn Algorithm + Send + Sync>,
    visited: &mut HashSet<TilePos>,
    goals: &HashSet<TilePos>,
    storage: &TileStorage,
    mut tiles: Query<&mut TileState>,
) -> ControlFlow<()> {
    let Some(tile) = algorithm.next() else {
        return ControlFlow::Break(());
    };

    if goals.contains(&tile) {
        return ControlFlow::Break(());
    }

    for neighbor in neighbors(tile) {
        if visited.contains(&neighbor) {
            continue;
        }

        visited.insert(neighbor);

        let Some(entity) = storage.checked_get(&neighbor) else {
            continue;
        };

        let mut neighbor_state = tiles.get_mut(entity).unwrap();

        if *neighbor_state == TileState::Wall {
            continue;
        }

        algorithm.insert(tile, goals);

        neighbor_state.change_from(TileState::Empty, TileState::Queued);
    }

    tiles
        .get_mut(storage.checked_get(&tile).unwrap())
        .unwrap()
        .change_from(TileState::Queued, TileState::Visited);

    ControlFlow::Continue(())
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
    fn insert(&mut self, tile: TilePos, goals: &HashSet<TilePos>);
    fn next(&mut self) -> Option<TilePos>;
}

fn neighbors(TilePos { x, y }: TilePos) -> [TilePos; 4] {
    [
        TilePos {
            x: x.saturating_add(1),
            y,
        },
        TilePos {
            x: x.saturating_sub(1),
            y,
        },
        TilePos {
            x,
            y: y.saturating_add(1),
        },
        TilePos {
            x,
            y: y.saturating_sub(1),
        },
    ]
}
