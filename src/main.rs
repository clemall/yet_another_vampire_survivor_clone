use bevy::{prelude::*};

use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pixel_camera::{
    PixelCameraPlugin
};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
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
        // .add_systems(Update, display_events)
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


pub fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.read() {
        println!("Received collision event: {collision_event:?}");
    }

    for contact_force_event in contact_force_events.read() {
        println!("Received contact force event: {contact_force_event:?}");
    }
}

// 
// 
// 
// use bevy::{
//     ecs::system::{RunSystemOnce, SystemId},
//     prelude::*,
// };
// 
// fn main() {
//     App::new()
//         .add_systems(Startup, (count_entities, setup))
//         .add_systems(PostUpdate, count_entities)
//         .add_systems(Update, evaluate_callbacks)
//         .run();
// }
// 
// // Any ordinary system can be run via commands.run_system or world.run_system.
// fn count_entities(all_entities: Query<()>) {
//     dbg!(all_entities.iter().count());
// }
// 
// #[derive(Component)]
// struct Callback(SystemId);
// 
// #[derive(Component)]
// struct Triggered;
// 
// fn setup(world: &mut World) {
//     let button_pressed_id = world.register_system(button_pressed);
//     world.spawn((Callback(button_pressed_id), Triggered));
//     // This entity does not have a Triggered component, so its callback won't run.
//     let slider_toggled_id = world.register_system(slider_toggled);
//     world.spawn(Callback(slider_toggled_id));
//     world.run_system_once(count_entities);
// }
// 
// fn button_pressed() {
//     println!("A button was pressed!");
// }
// 
// fn slider_toggled() {
//     println!("A slider was toggled!");
// }
// 
// /// Runs the systems associated with each `Callback` component if the entity also has a Triggered component.
// ///
// /// This could be done in an exclusive system rather than using `Commands` if preferred.
// fn evaluate_callbacks(query: Query<&Callback, With<Triggered>>, mut commands: Commands) {
//     for callback in query.iter() {
//         commands.run_system(callback.0);
//     }
// }
