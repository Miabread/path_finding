use bevy::prelude::*;

use crate::TileState;

pub fn generate_flat(tiles: &mut Query<'_, '_, &mut TileState>, fill: TileState) {
    for mut tile in tiles.iter_mut() {
        *tile = fill;
    }
}
