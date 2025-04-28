use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use bevy_egui::{
    EguiContexts,
    egui::{Grid, RichText, Slider, Window},
};

use crate::{
    TileState,
    generate::{generate_flat, generate_maze},
    pathfinder::{AlgorithmOption, Pathfinder},
};

pub fn options_plugin(app: &mut App) {
    app.init_resource::<Options>()
        .add_systems(Update, options_menu)
        .add_systems(FixedUpdate, auto_step);
}

const MAX_AUTO_SPEED: usize = 20;

#[derive(Debug, Resource, Default)]
struct Options {
    algorithm: AlgorithmOption,
    auto_enabled: bool,
    auto_speed: usize,
    current_tick: usize,
}

fn options_menu(
    mut contexts: EguiContexts,
    mut pathfinder: ResMut<Pathfinder>,
    mut options: ResMut<Options>,
    mut tiles: Query<&mut TileState>,
    mut tiles_pos: Query<&TilePos>,
    storage: Query<&TileStorage>,
) {
    Window::new("Options").show(contexts.ctx_mut(), |ui| {
        let spacing = 10.0;

        ui.add_space(spacing);
        ui.heading(format!("Algorithm (step {})", pathfinder.step));
        ui.separator();
        {
            let mut restart = false;

            Grid::new("algorithm").show(ui, |ui| {
                for heading in [
                    "Algorithm",
                    "Also Known As",
                    "Data Structure",
                    "Prioritizes",
                ] {
                    ui.label(RichText::new(heading).underline());
                }
                ui.end_row();

                restart |= ui
                    .radio_value(
                        &mut options.algorithm,
                        AlgorithmOption::BreadthFirst,
                        "Dijkstra",
                    )
                    .changed();
                ui.label("Breadth First");
                ui.label("Queue");
                ui.label("Oldest");
                ui.end_row();

                restart |= ui
                    .radio_value(&mut options.algorithm, AlgorithmOption::AStar, "A*")
                    .changed();
                ui.label("Heuristic");
                ui.label("Binary Heap");
                ui.label("Best");
                ui.end_row();

                restart |= ui
                    .radio_value(
                        &mut options.algorithm,
                        AlgorithmOption::DepthFirst,
                        "Backtracking",
                    )
                    .changed();
                ui.label("Depth First");
                ui.label("Stack");
                ui.label("Newest");
                ui.end_row();

                restart |= ui
                    .radio_value(&mut options.algorithm, AlgorithmOption::Random, "Random")
                    .changed();
                ui.label("Bogo");
                ui.label("Array");
                ui.label("Random");
                ui.end_row();
            });

            if restart {
                restart_pathfinder(options.reborrow(), pathfinder.reborrow(), tiles.reborrow());
            }
        }

        ui.add_space(spacing);
        ui.heading(format!(
            "Pathfinder ({})",
            if pathfinder.complete {
                "complete"
            } else {
                "running"
            }
        ));
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("Restart").clicked() {
                restart_pathfinder(options.reborrow(), pathfinder.reborrow(), tiles.reborrow());
            };

            if ui.button("Step").clicked() {
                pathfinder.step(storage.single(), tiles.reborrow());
            };

            ui.checkbox(&mut options.auto_enabled, "Auto");

            ui.add(Slider::new(&mut options.auto_speed, 0..=MAX_AUTO_SPEED).text("Speed"));
        });

        ui.add_space(spacing);
        ui.heading("Map");
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("Clear All").clicked() {
                restart_pathfinder(options.reborrow(), pathfinder.reborrow(), tiles.reborrow());
                generate_flat(tiles.reborrow(), TileState::Empty);
            }

            if ui.button("Fill All").clicked() {
                restart_pathfinder(options.reborrow(), pathfinder.reborrow(), tiles.reborrow());
                generate_flat(tiles.reborrow(), TileState::Wall);
            }

            if ui.button("Maze").clicked() {
                restart_pathfinder(options.reborrow(), pathfinder.reborrow(), tiles.reborrow());
                generate_maze(tiles.reborrow(), tiles_pos.reborrow(), storage)
            }
        });

        ui.add_space(spacing);
    });
}

fn restart_pathfinder(
    mut options: Mut<Options>,
    mut pathfinder: Mut<Pathfinder>,
    mut tiles: Query<&mut TileState>,
) {
    options.current_tick = 0;
    pathfinder.restart(options.algorithm);

    for mut tile in tiles.iter_mut() {
        if matches!(*tile, TileState::Queued | TileState::Visited(_)) {
            *tile = TileState::Empty;
        }
    }
}

fn auto_step(
    mut pathfinder: ResMut<Pathfinder>,
    mut options: ResMut<Options>,
    tiles: Query<&mut TileState>,
    storage: Query<&TileStorage>,
) {
    if !options.auto_enabled {
        options.current_tick = 0;
        return;
    }

    options.current_tick += 1;
    if options.current_tick >= (MAX_AUTO_SPEED - options.auto_speed) {
        options.current_tick = 0;
        pathfinder.step(storage.single(), tiles);
    }
}
