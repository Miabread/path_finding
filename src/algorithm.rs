use bevy::prelude::*;
use bevy_egui::egui::{self, RadioButton};
use std::collections::{BinaryHeap, VecDeque};

pub fn algorithm_plugin(app: &mut App) {
    app.init_resource::<Algorithm>();
}

#[derive(Debug, Default)]
struct BreadthFirst {
    queue: VecDeque<bool>,
}

#[derive(Debug, Default)]
struct AStar {
    queue: BinaryHeap<bool>,
}

#[derive(Debug, Default)]
struct DepthFirst {
    queue: Vec<bool>,
}

#[derive(Debug, Default)]
struct Random {
    queue: Vec<bool>,
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

#[derive(Debug, Resource)]
pub enum Algorithm {
    BreadthFirst(BreadthFirst),
    AStar(AStar),
    DepthFirst(DepthFirst),
    Random(Random),
}

impl Algorithm {
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
