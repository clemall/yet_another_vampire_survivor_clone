use bevy::{prelude::*};

use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pixel_camera::{
    PixelCameraPlugin
};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_rapier2d::prelude::*;
use yet_another_vampire_survivor_clone::components::*;
use yet_another_vampire_survivor_clone::animations::animation::AnimationSimplePlugin;
use yet_another_vampire_survivor_clone::cameras::camera::PlayerCameraPlugin;
use yet_another_vampire_survivor_clone::enemies::enemy::EnemyPlugin;
use yet_another_vampire_survivor_clone::gems::gem::GemsPlugin;
use yet_another_vampire_survivor_clone::players::player::PlayerPlugin;
use yet_another_vampire_survivor_clone::ui::ui_enemy::UiEnemyPlugin;
use yet_another_vampire_survivor_clone::ui::ui_fps::UiFPSPlugin;
use yet_another_vampire_survivor_clone::ui::ui_level_up::UiLevelUpPlugin;
use yet_another_vampire_survivor_clone::ui::ui_player::UiPlayerPlugin;
use yet_another_vampire_survivor_clone::weapons::arcane_missile::ArcaneMissilePlugin;
use yet_another_vampire_survivor_clone::weapons::claw::{WeaponClawPlugin};
use yet_another_vampire_survivor_clone::weapons::fire_area::{ WeaponFireAreaPlugin};


fn main() {
    App::new()
        // bevy plugin
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // States
        .init_state::<GameState>()
        // Events
        .add_event::<EnemyDied>()
        .add_event::<CollectExperience>()

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
        .insert_resource(PlayerExperience{
            level: 1,
            amount_experience: 0,
        })
        .add_plugins(PlayerPlugin)
        // Enemies plugin
        .add_plugins(EnemyPlugin)
        // UI
        .add_plugins(UiEnemyPlugin)
        .add_plugins(UiPlayerPlugin)
        .add_plugins(UiLevelUpPlugin)
        // animation
        .add_plugins(AnimationSimplePlugin)
        // gems
        .add_plugins(GemsPlugin)
        // Weapons
        .insert_resource(PlayerWeapons{ weapons:Vec::new() })
        .add_plugins(WeaponClawPlugin)
        // .add_plugins(WeaponFireAreaPlugin)
        // .add_plugins(ArcaneMissilePlugin)

        // Setup
        // .add_systems(Startup, setup)
        // test
        .add_systems(Startup, background)
        .add_systems(Update, debug_add_claw_attack)
        .run();
}

// fn setup(
//     mut commands: Commands,
//     mut player_weapons: ResMut<PlayerWeapons>,
// ){
//     // default weapon
//     // setup_claw_spawner(&mut commands);
//     // setup_fire_area(&mut commands);
//     // TODO: change that to use a list of enum representing each weapons
//     // player_weapons.weapons.push(WeaponsTypes::CLAW);
//     // player_weapons.weapons.push(WeaponsTypes::FireArea);
// }

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

