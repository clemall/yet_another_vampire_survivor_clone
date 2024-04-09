use bevy::{prelude::*};

use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pixel_camera::{
    PixelCameraPlugin
};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::time::Stopwatch;
use bevy::window::WindowMode;
use bevy_rapier2d::prelude::*;
use yet_another_vampire_survivor_clone::components::*;
use yet_another_vampire_survivor_clone::animations::animation::AnimationSimplePlugin;
use yet_another_vampire_survivor_clone::cameras::camera::PlayerCameraPlugin;
use yet_another_vampire_survivor_clone::enemies::enemy::EnemyPlugin;
use yet_another_vampire_survivor_clone::gems::gem::GemsPlugin;
use yet_another_vampire_survivor_clone::math_utils::get_random_position_in_screen;
use yet_another_vampire_survivor_clone::players::player::PlayerPlugin;
use yet_another_vampire_survivor_clone::ui::ui_enemy::UiEnemyPlugin;
use yet_another_vampire_survivor_clone::ui::ui_fps::UiFPSPlugin;
use yet_another_vampire_survivor_clone::ui::ui_level_up::UiLevelUpPlugin;
use yet_another_vampire_survivor_clone::ui::ui_player::UiPlayerPlugin;
use yet_another_vampire_survivor_clone::weapons::arcane_missile::ArcaneMissilePlugin;
use yet_another_vampire_survivor_clone::weapons::bouncing_ball::BouncingBallPlugin;
use yet_another_vampire_survivor_clone::weapons::chain_lightning::ChainLightningPlugin;
use yet_another_vampire_survivor_clone::weapons::claw::{WeaponClawPlugin};
use yet_another_vampire_survivor_clone::weapons::generic_systems::GenericWeaponPlugin;
use yet_another_vampire_survivor_clone::weapons::fire_area::{ WeaponFireAreaPlugin};
use yet_another_vampire_survivor_clone::weapons::shuriken::ShurikenPlugin;
use yet_another_vampire_survivor_clone::weapons::slow_dome::SlowDomePlugin;


fn main() {
    App::new()
        // bevy plugin
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // States
        .init_state::<GameState>()

        // Events
        .add_event::<EnemyDied>()
        .add_event::<CollectExperience>()
        .add_event::<EnemyReceivedDamage>()
        .add_event::<PlayerReceivedDamage>()

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
        // Waves
        .insert_resource(WaveManagerGlobalTime{ global_time:Stopwatch::new() })

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
        .add_plugins(GenericWeaponPlugin)
        .add_plugins(WeaponClawPlugin)
        .add_plugins(WeaponFireAreaPlugin)
        .add_plugins(ArcaneMissilePlugin)
        .add_plugins(ShurikenPlugin)
        .add_plugins(ChainLightningPlugin)
        .add_plugins(SlowDomePlugin)
        .add_plugins(BouncingBallPlugin)

        // Setup
        // .add_systems(Startup, setup)
        // test
        .insert_resource(Time::<Fixed>::from_hz(64.0))
        .add_systems(Startup, background)
        .add_systems(Update, debug)
        .run();
}

fn background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("map_1/png/background_level_1.png");

    for i in -3..3 {
        for y in -3..3 {
            commands.spawn((
                SpriteBundle {
                    texture: texture.clone(),
                    transform: Transform::from_xyz(i as f32 * 1024.0, y as f32 * 1024.0, -1.0),
                    ..default()
                },
                Name::new("Background")
            ));
        }
    }


}




fn debug(
    mut windows: Query<&mut Window>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_weapons: ResMut<PlayerWeapons>,
    mut enemy_died: EventWriter<EnemyDied>,
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
    if keyboard_input.just_pressed(KeyCode::Digit4) {
        player_weapons.weapons.push(WeaponsTypes::Shuriken);
    }
    if keyboard_input.just_pressed(KeyCode::Digit5) {
        player_weapons.weapons.push(WeaponsTypes::ChainLightning);
    }
    if keyboard_input.just_pressed(KeyCode::Digit6) {
        player_weapons.weapons.push(WeaponsTypes::SlowDome);
    }
    if keyboard_input.just_pressed(KeyCode::Digit7) {
        player_weapons.weapons.push(WeaponsTypes::BouncingBall);
    }

    let mut window = windows.single_mut();

    if keyboard_input.just_pressed(KeyCode::F1) {
        window.mode = WindowMode::BorderlessFullscreen;
    }
    if keyboard_input.just_pressed(KeyCode::F2) {
        window.mode = WindowMode::Windowed;
    }


    if keyboard_input.pressed(KeyCode::KeyG) {
        enemy_died.send(
            EnemyDied{ 
                position: get_random_position_in_screen().extend(0.0),
                experience: 1 
            }
        );
    }
    
    
}
