mod tile;
mod utils;

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized};
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
    }).insert(PanCam {
        min_scale: 1.0,
        max_scale: Some(1.0),
        ..default()
    });
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

fn deck_init_pos(query_window: Query<&Window, With<PrimaryWindow>>,
                 mut set: ParamSet<(
                     Query<&mut Transform, With<Deck>>,
                     Query<&Transform, With<PanCam>>
                 )>
) {
    let pos_cam = set.p1().single_mut().translation;
    let mut query_deck = set.p0();
    let mut transform_deck = query_deck.single_mut();
    let window = query_window.single();
    let height = window.height();
    let width = window.width();

    transform_deck.translation.x = (pos_cam.x + width / 2.0) - 50.0;
    transform_deck.translation.y = (pos_cam.y - height / 2.0) + 50.0;
}

fn update_deck_pos_on_resize(mut resize_event: EventReader<WindowResized>,
                             mut set: ParamSet<(
                        Query<&mut Transform, With<Deck>>,
                        Query<&Transform, With<PanCam>>
                   )>
) {
    for e in resize_event.read() {
        let pos_cam = set.p1().single_mut().translation;
        let mut query_deck = set.p0();
        let mut transform_deck = query_deck.single_mut();

        println!("{}x{}", e.width, e.height);
        println!("Deck: ({}, {})", transform_deck.translation.x, transform_deck.translation.y);
        println!("Cam: ({}, {})", pos_cam.x, pos_cam.y);

        transform_deck.translation.x = (pos_cam.x + (e.width / 2.0)) - 50.0;
        transform_deck.translation.y = (pos_cam.y - (e.height / 2.0)) + 50.0;
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
        .add_systems(PostStartup, (lock_entity_to_cam, deck_init_pos))
        // .add_systems(Update, )
        .add_systems(Last, update_deck_pos_on_resize)
        .run();
}
