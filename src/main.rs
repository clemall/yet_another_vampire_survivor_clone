use bevy::{prelude::*, utils::Duration};

use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pixel_camera::{
    PixelCameraPlugin
};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use yet_another_vampire_survivor_clone::components::*;
use yet_another_vampire_survivor_clone::animations::animation::AnimationSimplePlugin;
use yet_another_vampire_survivor_clone::cameras::camera::PlayerCameraPlugin;
use yet_another_vampire_survivor_clone::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use yet_another_vampire_survivor_clone::enemies::enemy::EnemyPlugin;
use yet_another_vampire_survivor_clone::players::player::PlayerPlugin;
use yet_another_vampire_survivor_clone::ui::ui_enemy::UiEnemyPlugin;
use yet_another_vampire_survivor_clone::ui::ui_fps::UiFPSPlugin;
use yet_another_vampire_survivor_clone::ui::ui_player::UiPlayerPlugin;
use yet_another_vampire_survivor_clone::weapons::arcane_missile::ArcaneMissilePlugin;
use yet_another_vampire_survivor_clone::weapons::claw::{WeaponClawPlugin};
use yet_another_vampire_survivor_clone::weapons::fire_area::{setup_fire_area, WeaponFireAreaPlugin};

#[allow(unused)]
fn main() {
    App::new()
        // bevy plugin
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // FPS plugin
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(UiFPSPlugin)
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
        // Enemies plugin
        .add_plugins(EnemyPlugin)
        // UI
        .add_plugins(UiEnemyPlugin)
        .add_plugins(UiPlayerPlugin)
        // animation
        .add_plugins(AnimationSimplePlugin)
        // Weapons
        .insert_resource(PlayerWeapons{ weapons:Vec::new() })
        .add_plugins(WeaponClawPlugin)
        .add_plugins(WeaponFireAreaPlugin)
        .add_plugins(ArcaneMissilePlugin)
        // Setup
        .add_systems(Startup, setup)
        // test
        .add_systems(Startup, background)
        .add_systems(Update, debug_spawn_enemies)
        .add_systems(Update, debug_add_claw_attack)
        .run();
}

fn setup(
    mut commands: Commands,
    mut player_weapons: ResMut<PlayerWeapons>,
){
    // default weapon
    // setup_claw_spawner(&mut commands);
    // setup_fire_area(&mut commands);
    // TODO: change that to use a list of enum representing each weapons
    // player_weapons.weapons.push(WeaponsTypes::CLAW);
    // player_weapons.weapons.push(WeaponsTypes::FireArea);
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




fn debug_add_claw_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_weapons: ResMut<PlayerWeapons>,
){
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        player_weapons.weapons.push(WeaponsTypes::Claw);
    }
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        player_weapons.weapons.push(WeaponsTypes::FireArea);
    }
    if keyboard_input.just_pressed(KeyCode::Digit3) {
        player_weapons.weapons.push(WeaponsTypes::ArcaneMissile);
    }
}

// TODO: move code elsewhere
fn debug_spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
){
    if keyboard_input.just_pressed(KeyCode::KeyO) || keyboard_input.pressed(KeyCode::KeyP) {
        let mut rng = rand::thread_rng();
        let x: i32 = rng.gen_range(-SCREEN_WIDTH/2..SCREEN_WIDTH/2);
        let y: i32 = rng.gen_range(-SCREEN_HEIGHT/2..SCREEN_HEIGHT/2);

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("enemies.png"),
                transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                ..default()
            },
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED_Z,
            Damping {
                linear_damping: 100.0,
                angular_damping: 1.0,
            },
            Collider::ball(8.0),
            ColliderMassProperties::Density(2.0),
            Enemy,
            Health(500.0),
            MaxHealth(500.0),
            EnemyVelocity(Vec2::new(0.0, 0.0)),
            EnemySpeed(30.0),
            EnemyDamageOverTime(10.0),
        ));
    }

}
