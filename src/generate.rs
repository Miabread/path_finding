use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use maze_generator::prelude::*;
use maze_generator::recursive_backtracking::RbGenerator;
use noise::{NoiseFn, Perlin};
use rand::{Rng, rng};

use crate::{MAP_SIZE, TileParent, TileState};

pub fn flush_path(mut tile_states: Query<&mut TileState>, mut parents: Query<&mut TileParent>) {
    for mut tile_state in tile_states.iter_mut() {
        if matches!(
            *tile_state,
            TileState::Queued | TileState::Visited(_) | TileState::Final(_)
        ) {
            *tile_state = TileState::Empty;
        }
    }

    for mut parent in parents.iter_mut() {
        parent.0 = None;
    }
}

pub fn generate_flat(mut tile_states: Query<&mut TileState>, fill: TileState) {
    for mut tile_state in tile_states.iter_mut() {
        *tile_state = fill;
    }
}

pub fn generate_noise(
    mut tile_states: Query<&mut TileState>,
    mut tile_positions: Query<&TilePos>,
    scale: f64,
    threshold: f64,
) {
    generate_flat(tile_states.reborrow(), TileState::Empty);

    let noise = Perlin::new(rng().random());

    let mut lens = tile_positions.join::<_, (&mut TileState, &TilePos)>(&mut tile_states);

    for (mut tile_state, &TilePos { x, y }) in lens.query().iter_mut() {
        if noise.get([x as f64 / scale + 0.5, y as f64 / scale + 0.5]) > threshold {
            *tile_state = TileState::Wall;
        }
    }
}

pub fn generate_maze(
    mut tile_states: Query<&mut TileState>,
    tile_positions: Query<&TilePos>,
    storage: &TileStorage,
) {
    generate_flat(tile_states.reborrow(), TileState::Empty);

    let maze = RbGenerator::new(None)
        .generate(MAP_SIZE as i32 / 2, MAP_SIZE as i32 / 2)
        .unwrap();

    for &TilePos { x, y } in tile_positions {
        match (x % 2 == 0, y % 2 == 0) {
            (false, false) => {
                let entity = storage.checked_get(&TilePos::new(x, y)).unwrap();
                let mut tile_state = tile_states.get_mut(entity).unwrap();
                *tile_state = TileState::Wall;
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
                    if let Some(entity) = storage.checked_get(&TilePos::new(x, y)) {
                        let mut tile_state = tile_states.get_mut(entity).unwrap();
                        if field.has_passage(&direction) {
                            *tile_state = TileState::Empty;
                        } else {
                            *tile_state = TileState::Wall;
                        }
                    }
                }
            }
        }
    }
}
