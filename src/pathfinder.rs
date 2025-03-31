use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use rand::seq::IteratorRandom;
use std::collections::{BinaryHeap, HashSet, VecDeque};

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
        pathfinder.goal.remove(pos);

        match state {
            TileState::Start => {
                pathfinder.start.insert(*pos);
            }

            TileState::End => {
                pathfinder.start.insert(*pos);
            }

            _ => {}
        }
    }
}

#[derive(Resource, Default)]
pub struct Pathfinder {
    algorithm: Option<Box<dyn Algorithm + Sync + Send>>,
    start: HashSet<TilePos>,
    goal: HashSet<TilePos>,
}

impl Pathfinder {
    pub fn restart(&mut self, algorithm: AlgorithmOption) {
        self.algorithm = Some(algorithm.into());

        if let Some(start) = self.start.iter().choose(&mut rand::rng()) {
            self.algorithm.as_mut().unwrap().start(*start);
        }
    }

    pub fn step(&mut self) {
        if let Some(algorithm) = &mut self.algorithm {
            algorithm.step(&self.goal);
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
    fn step(&mut self, goals: &HashSet<TilePos>);
}

#[derive(Debug, Default)]
struct BreadthFirst {
    queue: VecDeque<TilePos>,
}

impl Algorithm for BreadthFirst {
    fn start(&mut self, start: TilePos) {
        self.queue.push_back(start);
    }

    fn step(&mut self, goals: &HashSet<TilePos>) {
        todo!()
    }
}

#[derive(Debug, Default)]
struct AStar {
    queue: BinaryHeap<TilePos>,
}

impl Algorithm for AStar {
    fn start(&mut self, start: TilePos) {
        self.queue.push(start);
    }

    fn step(&mut self, goals: &HashSet<TilePos>) {
        todo!()
    }
}

#[derive(Debug, Default)]
struct DepthFirst {
    queue: Vec<TilePos>,
}

impl Algorithm for DepthFirst {
    fn start(&mut self, start: TilePos) {
        self.queue.push(start);
    }

    fn step(&mut self, goals: &HashSet<TilePos>) {
        todo!()
    }
}

#[derive(Debug, Default)]
struct Random {
    queue: Vec<TilePos>,
}

impl Algorithm for Random {
    fn start(&mut self, start: TilePos) {
        self.queue.push(start);
    }

    fn step(&mut self, goals: &HashSet<TilePos>) {
        todo!()
    }
}
