use bevy::prelude::*;
use bevy_egui::{egui::Window, EguiContexts};

use crate::{
    pathfinder::{show_algorithm_selection, Pathfinder},
    TileState,
};

pub fn options_plugin(app: &mut App) {
    app.init_resource::<Options>()
        .add_systems(Update, options_menu);
}

#[derive(Debug, Resource, Default)]
struct Options {}

fn options_menu(
    mut contexts: EguiContexts,
    mut pathfinder: ResMut<Pathfinder>,
    mut tiles: Query<&mut TileState>,
) {
    Window::new("Options").show(contexts.ctx_mut(), |ui| {
        ui.label("Algorithm");
        show_algorithm_selection(ui, &mut pathfinder.algorithm);

        ui.label("Simulation");
        if ui.button("Step").clicked() {
            pathfinder.step();
        };
        if ui.button("Reset").clicked() {
            pathfinder.algorithm.reset();
        };

        ui.label("Field");
        if ui.button("Clear All").clicked() {
            for mut tile in tiles.iter_mut() {
                *tile = TileState::Empty
            }
        }
        if ui.button("Fill All").clicked() {
            for mut tile in tiles.iter_mut() {
                *tile = TileState::Wall
            }
        }
    });
}
