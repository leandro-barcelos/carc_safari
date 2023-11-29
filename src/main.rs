mod tile;
mod utils;

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::window::WindowResized;
use bevy_pancam::{PanCam, PanCamPlugin};
use tile::*;

#[derive(Component)]
pub struct LockToCam;

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

fn lock_entity_to_cam(mut commands: Commands,
                      query_cam: Query<Entity, With<PanCam>>,
                      query_lock_cam: Query<Entity, With<LockToCam>>
) {
    for entity in query_lock_cam.iter() {
        commands.entity(query_cam.single()).push_children(&[entity]);
    }
}

fn lock_deck_to_br(mut resize_event: EventReader<WindowResized>, mut query: Query<&mut Transform, With<Deck>>) {
    for e in resize_event.read() {
        let mut transform = query.single_mut();
        transform.translation.x = e.width / 2.0 - 46.0;
        transform.translation.y = e.height / 2.0 + 46.0;
    }
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
        .add_systems(PostStartup, lock_entity_to_cam)
        // .add_systems(Update, lock_deck_to_br)
        .run();
}
