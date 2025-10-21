use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use bevy_egui::{
    EguiContexts,
    egui::{Align, Grid, Layout, RichText, Slider, Window},
};

use crate::{
    TilePrev, TileState,
    algorithm::AlgorithmOption,
    generate::{flush_path, generate_flat, generate_maze, generate_noise},
    pathfinder::Pathfinder,
};

pub fn options_plugin(app: &mut App) {
    app.init_resource::<Options>()
        .add_systems(Update, options_menu)
        .add_systems(FixedUpdate, auto_step);
}

const MAX_AUTO_SPEED: usize = 20;

#[derive(Debug, Resource)]
struct Options {
    algorithm: AlgorithmOption,

    auto_enabled: bool,
    auto_speed: usize,
    current_tick: usize,

    noise_scale: f64,
    noise_threshold: f64,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            algorithm: AlgorithmOption::default(),
            auto_enabled: false,

            auto_speed: 20,
            current_tick: 0,

            noise_scale: 5.5,
            noise_threshold: 0.0,
        }
    }
}

fn options_menu(
    mut contexts: EguiContexts,
    mut pathfinder: ResMut<Pathfinder>,
    mut options: ResMut<Options>,
    mut states: Query<&mut TileState>,
    mut prevs: Query<&mut TilePrev>,
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
                        AlgorithmOption::ReverseAStar,
                        "Reverse A*",
                    )
                    .changed();
                ui.label("Heuristic");
                ui.label("Binary Heap");
                ui.label("Worst");
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
                pathfinder.restart(options.algorithm);
                flush_path(states.reborrow(), prevs.reborrow());
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
                pathfinder.restart(options.algorithm);
                flush_path(states.reborrow(), prevs.reborrow());
            };

            if ui.button("Step").clicked() {
                pathfinder.step(storage.single(), states.reborrow(), prevs.reborrow());
            };

            ui.checkbox(&mut options.auto_enabled, "Auto");

            ui.add(Slider::new(&mut options.auto_speed, 0..=MAX_AUTO_SPEED).text("Speed"));
        });

        ui.add_space(spacing);
        ui.heading("Map");
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("Flush").clicked() {
                flush_path(states.reborrow(), prevs.reborrow());
            }

            if ui.button("Empty").clicked() {
                pathfinder.restart(options.algorithm);
                generate_flat(states.reborrow(), TileState::Empty);
            }

            if ui.button("Wall").clicked() {
                pathfinder.restart(options.algorithm);
                generate_flat(states.reborrow(), TileState::Wall);
            }

            if ui.button("Noise").clicked() {
                pathfinder.restart(options.algorithm);
                generate_noise(
                    states.reborrow(),
                    tiles_pos.reborrow(),
                    options.noise_scale,
                    options.noise_threshold,
                );
            }

            if ui.button("Maze").clicked() {
                pathfinder.restart(options.algorithm);
                generate_maze(states.reborrow(), tiles_pos.reborrow(), storage)
            }
        });
        ui.add(Slider::new(&mut options.noise_scale, 1.0..=10.0).text("Noise Scale"));
        ui.add(Slider::new(&mut options.noise_threshold, -1.0..=1.0).text("Noise Threshold"));
    });

    Window::new("Controls").show(contexts.ctx_mut(), |ui| {
        let controls = [
            ("S", "Place Start"),
            ("E", "Place Goal"),
            ("Left", "Place Wall"),
            ("Right", "Place Empty"),
            ("Middle", "Move"),
            ("Scroll", "Zoom"),
        ];

        Grid::new("controls").show(ui, |ui| {
            for (key, label) in controls {
                ui.with_layout(Layout::right_to_left(Align::default()), |ui| {
                    ui.label(RichText::new(key).strong())
                });
                ui.label(label);
                ui.end_row();
            }
        });
    });
}

fn auto_step(
    mut pathfinder: ResMut<Pathfinder>,
    mut options: ResMut<Options>,
    tiles: Query<&mut TileState>,
    prevs: Query<&mut TilePrev>,
    storage: Query<&TileStorage>,
) {
    if !options.auto_enabled {
        options.current_tick = 0;
        return;
    }

    options.current_tick += 1;
    if options.current_tick >= (MAX_AUTO_SPEED - options.auto_speed) {
        options.current_tick = 0;
        pathfinder.step(storage.single(), tiles, prevs);
    }
}
