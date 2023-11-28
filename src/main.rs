mod tile;
mod utils;

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_pancam::{PanCam, PanCamPlugin};
use tile::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::ANTIQUE_WHITE),
        },
        ..default()
    }).insert(PanCam::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("tile_base.png"),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins
                    .set(ImagePlugin::default_nearest())
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "Carcassonne Safari".into(),
                            resolution: (640.0, 480.0).into(),
                            resizable: false,
                            ..default()
                        }),
                        ..default()
                    })
                    .build(),
                PanCamPlugin::default()
            ),
        )
        .add_systems(Startup,(setup, spawn_all_tiles))
        .run();
}
