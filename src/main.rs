use bevy::{input::mouse::MouseWheel, prelude::*, transform::commands};
use bevy_ecs_tilemap::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, TilemapPlugin))
        .add_event::<MouseWheel>()
        .init_resource::<CursorPos>()
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (movement, zoom, cursor_pos, mouse_paint, color_tile),
        )
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

fn mouse_paint(
    cursor_pos: Res<CursorPos>,
    tilemap: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,
    mut states: Query<&mut TileState>,
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let (map_size, grid_size, map_type, tile_storage, map_transform) = tilemap.single();

    // Grab the cursor position from the `Res<CursorPos>`
    let cursor_pos: Vec2 = cursor_pos.0;

    // We need to make sure that the cursor's world position is correct relative to the map
    // due to any map transformation.
    let cursor_in_map_pos: Vec2 = {
        // Extend the cursor_pos vec3 by 0.0 and 1.0
        let cursor_pos = Vec4::from((cursor_pos, 0.0, 1.0));
        let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
        cursor_in_map_pos.xy()
    };

    // Once we have a world position we can transform it into a possible tile position.
    let Some(tile_pos) = TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
    else {
        return;
    };

    // Highlight the relevant tile's label
    let Some(tile_entity) = tile_storage.get(&tile_pos) else {
        return;
    };

    let mut state = states.get_mut(tile_entity).unwrap();

    if mouse.pressed(MouseButton::Left) {
        *state = TileState::Wall;
    };

    if mouse.pressed(MouseButton::Right) {
        *state = TileState::Empty;
    }

    if keyboard.just_pressed(KeyCode::KeyS) {
        *state = TileState::Start;
    }

    if keyboard.just_pressed(KeyCode::KeyE) {
        *state = TileState::End;
    }
}

#[derive(Resource)]
pub struct CursorPos(Vec2);
impl Default for CursorPos {
    fn default() -> Self {
        // Initialize the cursor pos at some far away place. It will get updated
        // correctly when the cursor moves.
        Self(Vec2::new(-1000.0, -1000.0))
    }
}

// We need to keep the cursor position updated based on any `CursorMoved` events.
pub fn cursor_pos(
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_pos: ResMut<CursorPos>,
) {
    for cursor_moved in cursor_moved_events.read() {
        // To get the mouse's world position, we have to transform its window position by
        // any transforms on the camera. This is done by projecting the cursor position into
        // camera space (world space).
        for (cam_t, cam) in camera_q.iter() {
            if let Some(pos) = cam.viewport_to_world_2d(cam_t, cursor_moved.position) {
                *cursor_pos = CursorPos(pos);
            }
        }
    }
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
