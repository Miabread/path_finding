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
                restart_pathfinder(&mut options, &mut pathfinder, &mut tiles, None);
            }
        }

        ui.add_space(spacing);
        ui.heading("Pathfinder");
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("Restart").clicked() {
                restart_pathfinder(&mut options, &mut pathfinder, &mut tiles, None);
            };

            if ui.button("Step").clicked() {
                pathfinder.step(storage.single(), tiles.reborrow());
            };

            ui.checkbox(&mut options.auto_enabled, "Auto");
        });
        ui.add(Slider::new(&mut options.auto_speed, 0..=MAX_AUTO_SPEED).text("Speed"));

        ui.add_space(spacing);
        ui.heading("Map");
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("Clear All").clicked() {
                restart_pathfinder(
                    &mut options,
                    &mut pathfinder,
                    &mut tiles,
                    Some(TileState::Empty),
                );
            }

            if ui.button("Fill All").clicked() {
                restart_pathfinder(
                    &mut options,
                    &mut pathfinder,
                    &mut tiles,
                    Some(TileState::Wall),
                );
            }
        });

        ui.add_space(spacing);
    });
}

fn restart_pathfinder(
    options: &mut ResMut<'_, Options>,
    pathfinder: &mut ResMut<'_, Pathfinder>,
    tiles: &mut Query<'_, '_, &mut TileState>,
    fill: Option<TileState>,
) {
    options.current_tick = 0;
    pathfinder.restart(options.algorithm);

    for mut tile in tiles.iter_mut() {
        if let Some(fill) = fill {
            *tile = fill;
        } else {
            tile.change_from(TileState::Queued, TileState::Empty);
            tile.change_from(TileState::Visited, TileState::Empty);
        }
    }

    if fill.is_some() {
        options.auto_enabled = false;
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
