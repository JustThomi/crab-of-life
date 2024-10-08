use bevy::prelude::*;
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

mod camera;
mod grid;

use camera::CameraPlugin;
use grid::GridPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, GridPlugin))
        .add_plugins(
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextStyle {
                        font_size: 50.0,
                        color: Color::srgb(1.0, 1.0, 1.0),
                        font: default(),
                    },
                },
            })
        .run();
}