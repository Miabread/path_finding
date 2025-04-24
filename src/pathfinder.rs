use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use rand::{rng, seq::IteratorRandom};
use std::{
    collections::{BinaryHeap, HashSet, VecDeque},
    ops::ControlFlow,
};

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

#[derive(Debug, Default)]
struct BreadthFirst {
    queue: VecDeque<TilePos>,
    visited: HashSet<TilePos>,
}

impl Algorithm for BreadthFirst {
    fn start(&mut self, start: TilePos) {
        self.queue.push_back(start);
        self.visited.insert(start);
    }

    fn step(
        &mut self,
        goals: &HashSet<TilePos>,
        storage: &TileStorage,
        mut tiles: Query<&mut TileState>,
    ) -> ControlFlow<()> {
        let Some(tile) = self.queue.pop_front() else {
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

            let Some(entity) = storage.get(&neighbor) else {
                continue;
            };

            self.queue.push_back(neighbor);

            tiles
                .get_mut(entity)
                .unwrap()
                .change_from(TileState::Empty, TileState::Queued);
        }

        tiles
            .get_mut(storage.get(&tile).unwrap())
            .unwrap()
            .change_from(TileState::Queued, TileState::Visited);

        ControlFlow::Continue(())
    }
}

#[derive(Debug, Default)]
struct AStar {
    queue: BinaryHeap<TilePos>,
    visited: HashSet<TilePos>,
}

impl Algorithm for AStar {
    fn start(&mut self, start: TilePos) {
        self.queue.push(start);
        self.visited.insert(start);
    }

    fn step(
        &mut self,
        _goals: &HashSet<TilePos>,
        _storage: &TileStorage,
        _tiles: Query<&mut TileState>,
    ) -> ControlFlow<()> {
        todo!()
    }
}

#[derive(Debug, Default)]
struct DepthFirst {
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

            self.queue.push(neighbor);

            tiles
                .get_mut(entity)
                .unwrap()
                .change_from(TileState::Empty, TileState::Queued);
        }

        tiles
            .get_mut(storage.get(&tile).unwrap())
            .unwrap()
            .change_from(TileState::Queued, TileState::Visited);

        ControlFlow::Continue(())
    }
}

#[derive(Debug, Default)]
struct Random {
    queue: Vec<TilePos>,
    visited: HashSet<TilePos>,
}

impl Algorithm for Random {
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
        let Some((i, &tile)) = self.queue.iter().enumerate().choose(&mut rng()) else {
            return ControlFlow::Break(());
        };
        self.queue.remove(i);

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

            self.queue.push(neighbor);

            tiles
                .get_mut(entity)
                .unwrap()
                .change_from(TileState::Empty, TileState::Queued);
        }

        tiles
            .get_mut(storage.get(&tile).unwrap())
            .unwrap()
            .change_from(TileState::Queued, TileState::Visited);

        ControlFlow::Continue(())
    }
}
