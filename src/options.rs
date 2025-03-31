use bevy::prelude::*;
use bevy_egui::{egui::Window, EguiContexts};

use crate::{
    pathfinder::{AlgorithmOption, Pathfinder},
    TileState,
};

pub fn options_plugin(app: &mut App) {
    app.init_resource::<Options>()
        .add_systems(Update, options_menu);
}

#[derive(Debug, Resource, Default)]
struct Options {
    algorithm: AlgorithmOption,
}

fn options_menu(
    mut contexts: EguiContexts,
    mut pathfinder: ResMut<Pathfinder>,
    mut options: ResMut<Options>,
    mut tiles: Query<&mut TileState>,
) {
    Window::new("Options").show(contexts.ctx_mut(), |ui| {
        ui.label("Algorithm");
        ui.radio_value(
            &mut options.algorithm,
            AlgorithmOption::BreadthFirst,
            "Dijkstra (Breadth First)",
        );
        ui.radio_value(&mut options.algorithm, AlgorithmOption::AStar, "A*");
        ui.radio_value(
            &mut options.algorithm,
            AlgorithmOption::DepthFirst,
            "Depth First",
        );
        ui.radio_value(&mut options.algorithm, AlgorithmOption::Random, "Random");

        ui.label("Pathfinder");
        if ui.button("Start").clicked() {
            pathfinder.start(options.algorithm);
        };
        if ui.button("Step").clicked() {
            pathfinder.step();
        };
        if ui.button("Stop").clicked() {
            pathfinder.stop();
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
