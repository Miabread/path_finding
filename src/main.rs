use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_ecs_tilemap::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, TilemapPlugin))
        .add_event::<MouseWheel>()
        .add_systems(Startup, startup)
        .add_systems(Update, (movement, zoom))
        .run()
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let map_size = TilemapSize { x: 32, y: 32 };

    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    color: TileColor(bevy::color::palettes::basic::BLUE.into()),
                    texture_index: TileTextureIndex(5),
                    ..Default::default()
                })
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

pub fn zoom(
    mut scroll: EventReader<MouseWheel>,
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    let mut ortho = query.single_mut();

    for event in scroll.read() {
        if event.y > 0.0 {
            ortho.scale /= 1.25;
        } else {
            ortho.scale *= 1.25;
        }
    }
}

pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let mut transform = query.single_mut();

    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyA) {
        direction -= Vec3::new(1.0, 0.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        direction -= Vec3::new(0.0, 1.0, 0.0);
    }

    let direction = direction.normalize_or_zero();

    let z = transform.translation.z;
    transform.translation += time.delta_seconds() * direction * 500.;

    // Important! We need to restore the Z values when moving the camera around.
    // Bevy has a specific camera setup and this can mess with how our layers are shown.
    transform.translation.z = z;
}
