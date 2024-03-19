use bevy::{prelude::*, utils::Duration};

use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pixel_camera::{
    PixelCameraPlugin, PixelZoom, PixelViewport
};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::render::camera::CameraPlugin;
use bevy_rapier2d::prelude::*;
use Farming::camera::PlayerCameraPlugin;
use Farming::components::{AnimationIndices, AnimationTimer, EnemyDamageOverTime, EnemySpeed, EnemyVelocity, Player};
use Farming::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use Farming::fps::FPSPlugin;
use Farming::player::PlayerPlugin;
use Farming::enemy::EnemyPlugin;
use Farming::weapons::claw::WeaponClawPlugin;

#[allow(unused)]
fn main() {
    App::new()
        // bevy plugin
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // FPS plugin
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(FPSPlugin)
        // Rapier2D plugin
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        // Debug plugin
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        // Camera pixel plugin
        .add_plugins(PixelCameraPlugin)
        .add_plugins(PlayerCameraPlugin)
        // Player plugin
        .add_plugins(PlayerPlugin)
        // Player plugin
        .add_plugins(EnemyPlugin)
        // animation
        .add_systems(Update, animate_sprite)
        // claw
        .add_plugins(WeaponClawPlugin)
        // test
        .add_systems(Startup, background)
        .run();
}



fn background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("background.png"),
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            ..default()
        },
    ));

}




fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if indices.is_repeating {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
            else {
                if  atlas.index < indices.last{
                    atlas.index = atlas.index + 1;
                }
            }

        }
    }
}
