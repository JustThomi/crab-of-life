use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldCoords>();
        app.add_systems(Startup, spawn);
        app.add_systems(Update, (window_to_world_coords, controlls));
    }
}

#[derive(Resource, Default)]
struct WorldCoords(Vec2);

fn spawn(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn window_to_world_coords(
    mut mycoords: ResMut<WorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
        eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    }
}

pub fn zoom_out(
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
    time: Res<Time>
) {
    for mut projection in query.iter_mut() {
        let mut log_scale = projection.scale.ln();
        log_scale += 0.5 * time.delta_seconds();
        projection.scale = log_scale.exp();
    }
}

pub fn zoom_in(
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
    time: Res<Time>
) {
    for mut projection in query.iter_mut() {
        let mut log_scale = projection.scale.ln();
        log_scale -= 0.5 * time.delta_seconds();
        projection.scale = log_scale.exp();
    }
}

fn controlls(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    zoom_cam: Query<&mut OrthographicProjection, With<Camera>>,
    mut camera_query: Query<&mut Transform, With<Camera>>, 
) {

    // Camera movement
    let mut direction: Vec3 = Vec3::ZERO;

    if let Ok(mut camera) = camera_query.get_single_mut() {
        if keys.pressed(KeyCode::KeyW) {
            direction += camera.up().normalize();
        }
        if keys.pressed(KeyCode::KeyS) {
            direction += camera.down().normalize();
        }
        if keys.pressed(KeyCode::KeyA) {
            direction += camera.left().normalize();
        }
        if keys.pressed(KeyCode::KeyD) {
            direction += camera.right().normalize();
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
            camera.translation += direction * 300.0 * time.delta_seconds();
        }

        // Zooming
        if keys.pressed(KeyCode::KeyQ) {
            zoom_out(zoom_cam, time);
        }
        else if keys.pressed(KeyCode::KeyE) {
            zoom_in(zoom_cam, time);
        }
    }
}