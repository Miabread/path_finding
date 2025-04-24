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

#[derive(Resource, Default)]
pub struct Pathfinder {
    algorithm: Option<Box<dyn Algorithm + Sync + Send>>,
    start: HashSet<TilePos>,
    goals: HashSet<TilePos>,
    step: usize,
}

impl Pathfinder {
    pub fn restart(&mut self, algorithm: AlgorithmOption) {
        self.step = 0;
        self.algorithm = Some(algorithm.into());

        if let Some(start) = self.start.iter().choose(&mut rand::rng()) {
            self.algorithm.as_mut().unwrap().start(*start);
        }
    }

    pub fn step(&mut self, storage: &TileStorage, tiles: Query<&mut TileState>) {
        if let Some(algorithm) = &mut self.algorithm {
            debug!("pathfinder step = {}", self.step);
            self.step += 1;
            if algorithm.step(&self.goals, storage, tiles).is_break() {
                self.algorithm = None;
            }
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
    fn start(&mut self, start: TilePos);

    fn step(
        &mut self,
        goals: &HashSet<TilePos>,
        storage: &TileStorage,
        tiles: Query<&mut TileState>,
    ) -> ControlFlow<()>;
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
