use crate::components::Player;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_pixel_camera::{PixelViewport, PixelZoom};

use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::players::player::player_movement;
use bevy::input::mouse::MouseScrollUnit;
use bevy::input::mouse::MouseWheel;

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, camera_follow.after(player_movement));
        app.add_systems(Update, (debug_camera, zoom_in));
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        // PixelZoom::FitSize {
        //     width: SCREEN_WIDTH,
        //     height: SCREEN_HEIGHT,
        //     // width: 4000,
        //     // height: 3000,
        // },
        // PixelViewport,
    ));
}

fn camera_follow(
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(player) = player.get_single() {
        let mut camera = camera.single_mut();
        camera.translation.x = player.translation.x;
        camera.translation.y = player.translation.y;
    }
}

fn debug_camera(
    mut mouse_events: EventReader<MouseMotion>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    return;
    let mut camera = camera.single_mut();
    for mouse_event in mouse_events.read() {
        camera.translation.x += mouse_event.delta.x;
        camera.translation.y += mouse_event.delta.y;
    }
}

pub fn zoom_in(
    mut camera: Query<&mut OrthographicProjection, With<Camera>>,
    time: Res<Time>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    let mut projection = camera.single_mut();

    let mut delta = 0.0;

    for ev in scroll_evr.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                delta = ev.y * 30.0;
            }
            MouseScrollUnit::Pixel => {
                delta = ev.y / 2.0;
            }
        }
    }
    let mut log_scale = projection.scale.ln();
    log_scale -= delta * time.delta_seconds();
    projection.scale = log_scale.exp();
}
