use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use maze_generator::prelude::*;
use maze_generator::recursive_backtracking::RbGenerator;

use crate::{MAP_SIZE, TileState};

pub fn generate_flat(mut tiles: Query<&mut TileState>, fill: TileState) {
    for mut tile in tiles.iter_mut() {
        *tile = fill;
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
                *state = TileState::Visited(5);
            }
            (true, false) | (false, true) => {}
            (true, true) => {
                let field = maze
                    .get_field(&Coordinates::new(x as i32 / 2, y as i32 / 2))
                    .unwrap();

                match field.field_type {
                    FieldType::Start => *state = TileState::Start,
                    FieldType::Goal => *state = TileState::Goal,
                    FieldType::Normal => {}
                }

                let directions = [
                    (Direction::North, x.saturating_add(1), y),
                    (Direction::South, x.saturating_sub(1), y),
                    (Direction::East, x, y.saturating_add(1)),
                    (Direction::West, x, y.saturating_sub(1)),
                ];

                for (direction, x, y) in directions {
                    if field.has_passage(&direction) {
                        storage.checked_get(&TilePos::new(x, y)).map(|entity| {
                            let mut tile = tile_states.get_mut(entity).unwrap();
                            // assert!(!matches!(*tile, TileState::Visited(_)), "was {:?}", *tile);
                            *tile = TileState::Visited(20);
                        });
                    }
                }
            }
        }
    }
}
