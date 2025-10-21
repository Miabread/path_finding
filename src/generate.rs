use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use maze_generator::prelude::*;
use maze_generator::recursive_backtracking::RbGenerator;
use noise::{NoiseFn, Perlin};
use rand::{Rng, rng};

use crate::{MAP_SIZE, TilePrev, TileState};

pub fn flush_path(mut states: Query<&mut TileState>, mut prevs: Query<&mut TilePrev>) {
    for mut state in states.iter_mut() {
        if matches!(
            *state,
            TileState::Queued | TileState::Visited(_) | TileState::Final
        ) {
            *state = TileState::Empty;
        }
    }

    for mut prev in prevs.iter_mut() {
        prev.0 = None;
    }
}

pub fn generate_flat(mut states: Query<&mut TileState>, fill: TileState) {
    for mut state in states.iter_mut() {
        *state = fill;
    }
}

pub fn generate_noise(
    mut tile_states: Query<&mut TileState>,
    mut tile_pos: Query<&TilePos>,
    scale: f64,
    threshold: f64,
) {
    generate_flat(tile_states.reborrow(), TileState::Empty);

    let noise = Perlin::new(rng().random());

    let mut lens = tile_pos.join::<_, (&mut TileState, &TilePos)>(&mut tile_states);

    for (mut state, &TilePos { x, y }) in lens.query().iter_mut() {
        if noise.get([x as f64 / scale + 0.5, y as f64 / scale + 0.5]) > threshold {
            *state = TileState::Wall;
        }
    }
}

pub fn generate_maze(
    mut tile_states: Query<&mut TileState>,
    mut tile_pos: Query<&TilePos>,
    storage: Query<&TileStorage>,
) {
    generate_flat(tile_states.reborrow(), TileState::Empty);

    let storage = storage.single();

    let maze = RbGenerator::new(None)
        .generate(MAP_SIZE as i32 / 2, MAP_SIZE as i32 / 2)
        .unwrap();

    let mut lens = tile_pos.join::<_, (&mut TileState, &TilePos)>(&mut tile_states);

    for (mut state, &TilePos { x, y }) in lens.query().iter_mut() {
        match (x % 2 == 0, y % 2 == 0) {
            (false, false) => {
                *state = TileState::Wall;
            }
            (true, false) | (false, true) => {}
            (true, true) => {
                let field = maze
                    .get_field(&Coordinates::new(x as i32 / 2, y as i32 / 2))
                    .unwrap();

                let directions = [
                    (Direction::North, x.saturating_add(1), y),
                    (Direction::South, x.saturating_sub(1), y),
                    (Direction::East, x, y.saturating_add(1)),
                    (Direction::West, x, y.saturating_sub(1)),
                ];

                for (direction, x, y) in directions {
                    storage.checked_get(&TilePos::new(x, y)).map(|entity| {
                        let mut tile = tile_states.get_mut(entity).unwrap();
                        if field.has_passage(&direction) {
                            *tile = TileState::Empty;
                        } else {
                            *tile = TileState::Wall;
                        }
                    });
                }
            }
        }
    }
}
