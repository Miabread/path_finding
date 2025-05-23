use std::{cmp::Reverse, collections::HashSet, fmt::Display, hash::Hash};

use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Debug, Clone, Copy, Default)]
pub struct Tile {
    pub pos: TilePos,
    pub distance: u32,
}

impl Tile {
    pub fn new(pos: TilePos, goals: &HashSet<Tile>) -> Self {
        let distance = goals
            .iter()
            .copied()
            .map(|goal| distance(pos, goal.pos).abs() as u32)
            .min()
            .unwrap_or(0);

        Self { pos, distance }
    }

    pub fn zero(pos: TilePos) -> Self {
        Self { pos, distance: 0 }
    }

    pub fn neighbors(&self, goals: &HashSet<Tile>) -> [Tile; 4] {
        let TilePos { x, y } = self.pos;
        [
            Tile::new(TilePos::new(x.saturating_add(1), y), goals),
            Tile::new(TilePos::new(x.saturating_sub(1), y), goals),
            Tile::new(TilePos::new(x, y.saturating_add(1)), goals),
            Tile::new(TilePos::new(x, y.saturating_sub(1)), goals),
        ]
    }
}

fn distance(a: TilePos, b: TilePos) -> i32 {
    let x_diff = b.x as i32 - a.x as i32;
    let y_diff = b.y as i32 - a.y as i32;
    (x_diff.pow(2) + y_diff.pow(2)).isqrt()
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.pos.eq(&other.pos)
    }
}

impl Eq for Tile {}

impl Hash for Tile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Reverse(self.distance).partial_cmp(&Reverse(other.distance))
    }
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Reverse(self.distance).cmp(&Reverse(other.distance))
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.pos.x, self.pos.y)
    }
}
