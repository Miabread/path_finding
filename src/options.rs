use bevy::prelude::*;
use bevy_egui::{
    egui::{Slider, Window},
    EguiContexts,
};

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
    auto_enabled: bool,
    auto_speed: usize,
}

fn options_menu(
    mut contexts: EguiContexts,
    mut pathfinder: ResMut<Pathfinder>,
    mut options: ResMut<Options>,
    mut tiles: Query<&mut TileState>,
) {
    Window::new("Options").show(contexts.ctx_mut(), |ui| {
        ui.heading("Algorithm");
        {
            let mut restart = false;

            restart |= ui
                .radio_value(
                    &mut options.algorithm,
                    AlgorithmOption::BreadthFirst,
                    "Dijkstra (Breadth First)",
                )
                .changed();

            restart |= ui
                .radio_value(&mut options.algorithm, AlgorithmOption::AStar, "A*")
                .changed();

            restart |= ui
                .radio_value(
                    &mut options.algorithm,
                    AlgorithmOption::DepthFirst,
                    "Depth First",
                )
                .changed();

            restart |= ui
                .radio_value(&mut options.algorithm, AlgorithmOption::Random, "Random")
                .changed();

            if restart {
                pathfinder.restart(options.algorithm);
            }
        }

        ui.separator();
        ui.heading("Pathfinder");
        ui.horizontal(|ui| {
            if ui.button("Restart").clicked() {
                pathfinder.restart(options.algorithm);
            };

            if ui.button("Step").clicked() {
                pathfinder.step();
            };

            ui.checkbox(&mut options.auto_enabled, "Auto");
        });
        ui.add(Slider::new(&mut options.auto_speed, 0..=10).text("Speed"));

        ui.separator();
        ui.heading("Map");
        ui.horizontal(|ui| {
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
    });
}
