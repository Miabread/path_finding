use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::input::egui_wants_input;

use crate::TileState;

pub fn input_plugin(app: &mut App) {
    app.init_resource::<CursorPos>().add_systems(
        Update,
        (movement, zoom, cursor_pos, mouse_paint).run_if(not(egui_wants_input)),
    );
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

    // We need to make sure that the cursor's world position is correct relative to the map due to any map transformation.
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
        *state = TileState::Goal;
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
            if let Ok(pos) = cam.viewport_to_world_2d(cam_t, cursor_moved.position) {
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
    mouse: Res<ButtonInput<MouseButton>>,
    mut motion: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
) {
    let (mut transform, ortho) = query.single_mut();

    if !mouse.pressed(MouseButton::Middle) {
        return;
    }

    for event in motion.read() {
        transform.translation += ortho.scale * Vec3::new(-event.delta.x, event.delta.y, 0.0);
    }
}
