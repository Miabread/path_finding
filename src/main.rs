mod algorithm;
mod input;

use algorithm::{algorithm_plugin, show_algorithm_selection, Algorithm};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use input::input_plugin;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TilemapPlugin,
            EguiPlugin,
            input_plugin,
            algorithm_plugin,
        ))
        .add_systems(Startup, startup)
        .add_systems(Update, (color_tile, ui_example_system))
        .run()
}

fn ui_example_system(mut contexts: EguiContexts, mut algorithm: ResMut<Algorithm>) {
    egui::Window::new("Settings").show(contexts.ctx_mut(), |ui| {
        ui.label("Algorithm");
        show_algorithm_selection(ui, &mut algorithm);
    });
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let map_size = TilemapSize { x: 32, y: 32 };

    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        color: TileColor(bevy::color::palettes::basic::BLUE.into()),
                        texture_index: TileTextureIndex(5),
                        ..Default::default()
                    },
                    TileState::Empty,
                ))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}

#[derive(Debug, Clone, Copy, Component, Default)]

enum TileState {
    #[default]
    Empty,
    Wall,
    Start,
    End,
}

fn color_tile(mut tile_q: Query<(&mut TileColor, &TileState), Changed<TileState>>) {
    for (mut color, state) in tile_q.iter_mut() {
        use bevy::color::palettes::basic;
        color.0 = match state {
            TileState::Empty => basic::GRAY,
            TileState::Wall => basic::WHITE,
            TileState::Start => basic::GREEN,
            TileState::End => basic::RED,
        }
        .into();
    }
}
