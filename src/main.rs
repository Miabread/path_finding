mod algorithm;
mod generate;
mod input;
mod options;
mod pathfinder;
mod tile;

use bevy::{asset::embedded_asset, log::LogPlugin, prelude::*, window::PrimaryWindow};
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiPlugin;
use input::input_plugin;
use options::options_plugin;
use pathfinder::pathfinder_plugin;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,path_finding=debug".into(),
            level: bevy::log::Level::DEBUG,
            ..Default::default()
        }),
        TilemapPlugin,
        EguiPlugin,
        input_plugin,
        pathfinder_plugin,
        options_plugin,
    ))
    .add_systems(Startup, startup)
    .add_systems(Update, color_tile);

    embedded_asset!(app, "../assets/tiles.png");

    app.run()
}

pub const MAP_SIZE: u32 = 32;

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
) {
    window.title = "Miabread/path_finding".to_string();

    commands.spawn(Camera2d::default());

    let texture_handle: Handle<Image> =
        asset_server.load("embedded://path_finding/../assets/tiles.png");

    let map_size = TilemapSize {
        x: MAP_SIZE,
        y: MAP_SIZE,
    };

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
                    TileParent(None),
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

#[derive(Debug, Clone, Copy, Component, Default, PartialEq, Eq)]
struct TileParent(Option<TilePos>);

#[derive(Debug, Clone, Copy, Component, Default, PartialEq, Eq)]

enum TileState {
    #[default]
    Empty,
    Wall,
    Start,
    Goal,
    Queued,
    Visited(u32),
    Final(u32),
}

fn color_tile(mut tile_q: Query<(&mut TileColor, &TileState), Changed<TileState>>) {
    for (mut color, state) in tile_q.iter_mut() {
        use bevy::color::palettes::basic;
        color.0 = match state {
            TileState::Empty => basic::GRAY,
            TileState::Wall => basic::WHITE,
            TileState::Start => basic::GREEN,
            TileState::Goal => basic::RED,
            TileState::Queued => basic::GRAY.lighter(0.2),
            TileState::Visited(distance) => {
                let ratio = *distance as f32 / MAP_SIZE as f32;
                Color::srgb(ratio, 1.0, 1.0 - ratio).into()
            }
            TileState::Final(distance) => {
                let ratio = *distance as f32 / MAP_SIZE as f32;
                basic::BLACK.lighter(ratio)
            }
        }
        .into();
    }
}
