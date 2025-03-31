use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use bevy_egui::egui::{self, RadioButton};
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

#[derive(Debug, Resource, Default)]
pub struct Pathfinder {
    pub algorithm: Algorithm,
    start: HashSet<TilePos>,
    goal: HashSet<TilePos>,
}

impl Pathfinder {
    pub fn step(&mut self) {
        let endpoints = Option::zip(
            self.start.iter().choose(&mut rand::rng()),
            self.goal.iter().choose(&mut rand::rng()),
        );

        if let Some((start, goal)) = endpoints {
            self.algorithm.step(*start, *goal);
        }
    }
}

#[derive(Debug, Default)]
struct BreadthFirst {
    queue: VecDeque<bool>,
}

impl BreadthFirst {
    pub fn step(&mut self, start: TilePos, goal: TilePos) {}
}

#[derive(Debug, Default)]
struct AStar {
    queue: BinaryHeap<bool>,
}

impl AStar {
    pub fn step(&mut self, start: TilePos, goal: TilePos) {}
}

#[derive(Debug, Default)]
struct DepthFirst {
    queue: Vec<bool>,
}

impl DepthFirst {
    pub fn step(&mut self, start: TilePos, goal: TilePos) {}
}

#[derive(Debug, Default)]
struct Random {
    queue: Vec<bool>,
}

impl Random {
    pub fn step(&mut self, start: TilePos, goal: TilePos) {}
}

pub fn show_algorithm_selection(ui: &mut egui::Ui, algorithm: &mut Algorithm) {
    if ui
        .add(RadioButton::new(
            algorithm.is_breadth(),
            "Dijkstra (Breadth First)",
        ))
        .clicked()
    {
        *algorithm = BreadthFirst::default().into();
    }

    if ui
        .add(egui::RadioButton::new(algorithm.is_astar(), "A*"))
        .clicked()
    {
        *algorithm = AStar::default().into();
    }

    if ui
        .add(egui::RadioButton::new(algorithm.is_depth(), "Depth First"))
        .clicked()
    {
        *algorithm = DepthFirst::default().into();
    }

    if ui
        .add(egui::RadioButton::new(algorithm.is_random(), "Random"))
        .clicked()
    {
        *algorithm = Random::default().into();
    }
}

#[derive(Debug)]
pub enum Algorithm {
    BreadthFirst(BreadthFirst),
    AStar(AStar),
    DepthFirst(DepthFirst),
    Random(Random),
}

impl Algorithm {
    pub fn reset(&mut self) {
        *self = match self {
            Algorithm::BreadthFirst(_) => BreadthFirst::default().into(),
            Algorithm::AStar(_) => AStar::default().into(),
            Algorithm::DepthFirst(_) => DepthFirst::default().into(),
            Algorithm::Random(_) => Random::default().into(),
        }
    }

    pub fn step(&mut self, start: TilePos, goal: TilePos) {
        match self {
            Algorithm::BreadthFirst(algo) => algo.step(start, goal),
            Algorithm::AStar(algo) => algo.step(start, goal),
            Algorithm::DepthFirst(algo) => algo.step(start, goal),
            Algorithm::Random(algo) => algo.step(start, goal),
        }
    }

    /// Returns `true` if the algorithm is [`BreadthFirst`].
    ///
    /// [`BreadthFirst`]: Algorithm::BreadthFirst
    #[must_use]
    pub fn is_breadth(&self) -> bool {
        matches!(self, Self::BreadthFirst(..))
    }

    /// Returns `true` if the algorithm is [`AStar`].
    ///
    /// [`AStar`]: Algorithm::AStar
    #[must_use]
    pub fn is_astar(&self) -> bool {
        matches!(self, Self::AStar(..))
    }

    /// Returns `true` if the algorithm is [`DepthFirst`].
    ///
    /// [`DepthFirst`]: Algorithm::DepthFirst
    #[must_use]
    pub fn is_depth(&self) -> bool {
        matches!(self, Self::DepthFirst(..))
    }

    /// Returns `true` if the algorithm is [`Random`].
    ///
    /// [`Random`]: Algorithm::Random
    #[must_use]
    pub fn is_random(&self) -> bool {
        matches!(self, Self::Random(..))
    }
}

impl Default for Algorithm {
    fn default() -> Self {
        BreadthFirst::default().into()
    }
}

impl From<BreadthFirst> for Algorithm {
    fn from(value: BreadthFirst) -> Self {
        Self::BreadthFirst(value)
    }
}

impl From<AStar> for Algorithm {
    fn from(value: AStar) -> Self {
        Self::AStar(value)
    }
}

impl From<DepthFirst> for Algorithm {
    fn from(value: DepthFirst) -> Self {
        Self::DepthFirst(value)
    }
}

impl From<Random> for Algorithm {
    fn from(value: Random) -> Self {
        Self::Random(value)
    }
}
