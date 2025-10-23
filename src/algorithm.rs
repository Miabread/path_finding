use rand::{rng, seq::IteratorRandom};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use crate::pathfinder_tile::PathfinderTile;

#[derive(Debug, Default)]
struct BreadthFirst {
    queue: VecDeque<PathfinderTile>,
}

impl Algorithm for BreadthFirst {
    fn insert(&mut self, tile: PathfinderTile) {
        self.queue.push_back(tile);
    }

    fn next(&mut self) -> Option<PathfinderTile> {
        self.queue.pop_front()
    }
}

#[derive(Debug, Default)]
struct AStar {
    queue: BinaryHeap<PathfinderTile>,
}

impl Algorithm for AStar {
    fn insert(&mut self, tile: PathfinderTile) {
        self.queue.push(tile);
    }

    fn next(&mut self) -> Option<PathfinderTile> {
        self.queue.pop()
    }
}

#[derive(Debug, Default)]
struct ReverseAStar {
    queue: BinaryHeap<Reverse<PathfinderTile>>,
}

impl Algorithm for ReverseAStar {
    fn insert(&mut self, tile: PathfinderTile) {
        self.queue.push(Reverse(tile));
    }

    fn next(&mut self) -> Option<PathfinderTile> {
        self.queue.pop().map(|title| title.0)
    }
}

#[derive(Debug, Default)]
struct DepthFirst {
    queue: Vec<PathfinderTile>,
}

impl Algorithm for DepthFirst {
    fn insert(&mut self, tile: PathfinderTile) {
        self.queue.push(tile);
    }

    fn next(&mut self) -> Option<PathfinderTile> {
        self.queue.pop()
    }
}

#[derive(Debug, Default)]
struct Random {
    queue: Vec<PathfinderTile>,
}

impl Algorithm for Random {
    fn insert(&mut self, tile: PathfinderTile) {
        self.queue.push(tile);
    }

    fn next(&mut self) -> Option<PathfinderTile> {
        let (i, &tile) = self.queue.iter().enumerate().choose(&mut rng())?;
        self.queue.remove(i);
        Some(tile)
    }
}

pub trait Algorithm {
    fn insert(&mut self, tile: PathfinderTile);
    fn next(&mut self) -> Option<PathfinderTile>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlgorithmOption {
    #[default]
    BreadthFirst,
    AStar,
    ReverseAStar,
    DepthFirst,
    Random,
}

impl From<AlgorithmOption> for Box<dyn Algorithm + Send + Sync> {
    fn from(value: AlgorithmOption) -> Self {
        match value {
            AlgorithmOption::BreadthFirst => Box::new(BreadthFirst::default()),
            AlgorithmOption::AStar => Box::new(AStar::default()),
            AlgorithmOption::ReverseAStar => Box::new(ReverseAStar::default()),
            AlgorithmOption::DepthFirst => Box::new(DepthFirst::default()),
            AlgorithmOption::Random => Box::new(Random::default()),
        }
    }
}
