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
use std::{collections::HashSet, fmt::Display, ops::ControlFlow};

use crate::TileState;

pub fn pathfinder_plugin(app: &mut App) {
    app.init_resource::<Pathfinder>()
        .add_systems(Update, update_endpoints);
}

fn update_endpoints(
    mut tile_q: Query<(&TileState, &TilePos), Changed<TileState>>,
    mut pathfinder: ResMut<Pathfinder>,
) {
    for (state, &pos) in tile_q.iter_mut() {
        if pathfinder.start.remove(&pos) {
            debug!("removed start tile {}", TilePosDisplay(pos));
        }

        if pathfinder.goals.remove(&pos) {
            debug!("removed goal tile {}", TilePosDisplay(pos));
        }

        match state {
            TileState::Start => {
                debug!("added start tile {}", TilePosDisplay(pos));
                pathfinder.start.insert(pos);
            }

            TileState::End => {
                debug!("added goal tile {}", TilePosDisplay(pos));
                pathfinder.goals.insert(pos);
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
    }

    pub fn step(&mut self, storage: &TileStorage, tiles: Query<&mut TileState>) {
        if self.complete {
            return;
        }

        debug!("VVVVV pathfinder step start = {} VVVVV", self.step);

        if self.visited.is_empty() {
            if let Some(&start) = self.start.iter().choose(&mut rand::rng()) {
                debug!("selected start tile {}", TilePosDisplay(start));
                self.algorithm.insert(start, &self.goals);
                self.visited.insert(start);
            } else {
                debug!("no start tiles to select from");
            }
        }

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

        debug!("^^^^^ pathfinder step done = {} ^^^^^", self.step);
        self.step += 1;
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
        debug!("no more tiles in queue");
        return ControlFlow::Break(());
    };

    debug!("stepping on tile {}", TilePosDisplay(tile));

    if goals.contains(&tile) {
        debug!("reached goal {}", TilePosDisplay(tile));
        return ControlFlow::Break(());
    }

    for neighbor in neighbors(tile) {
        if visited.contains(&neighbor) {
            debug!("neighbor skip {}", TilePosDisplay(neighbor));
            continue;
        }

        visited.insert(neighbor);

        let Some(entity) = storage.checked_get(&neighbor) else {
            debug!("neighbor bounds {}", TilePosDisplay(neighbor));
            continue;
        };

        let mut neighbor_state = tiles.get_mut(entity).unwrap();

        if *neighbor_state == TileState::Wall {
            debug!("neighbor wall {}", TilePosDisplay(neighbor));
            continue;
        }

        debug!("neighbor queue {}", TilePosDisplay(neighbor));

        algorithm.insert(neighbor, goals);

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

struct TilePosDisplay(TilePos);

impl Display for TilePosDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0.x, self.0.y)
    }
}
