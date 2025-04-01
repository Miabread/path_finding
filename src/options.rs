use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TileStorage;
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
    storage: Query<&TileStorage>,
) {
    Window::new("Options").show(contexts.ctx_mut(), |ui| {
        let spacing = 20.0;

        ui.add_space(spacing);
        ui.heading("Algorithm");
        ui.separator();
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

        ui.add_space(spacing);
        ui.heading("Pathfinder");
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("Restart").clicked() {
                pathfinder.restart(options.algorithm);
            };

            if ui.button("Step").clicked() {
                pathfinder.step(storage.single(), tiles.reborrow());
            };

            ui.checkbox(&mut options.auto_enabled, "Auto");
        });
        ui.add(Slider::new(&mut options.auto_speed, 0..=10).text("Speed"));

        ui.add_space(spacing);
        ui.heading("Map");
        ui.separator();
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

        ui.add_space(spacing);
    });
}
