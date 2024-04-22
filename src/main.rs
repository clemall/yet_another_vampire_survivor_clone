use bevy::prelude::*;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::input::common_conditions::input_toggle_active;
use bevy::window::WindowMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pixel_camera::PixelCameraPlugin;
use bevy_rapier2d::prelude::*;

use yet_another_vampire_survivor_clone::animations::animation::AnimationSimplePlugin;
use yet_another_vampire_survivor_clone::cameras::camera::PlayerCameraPlugin;
use yet_another_vampire_survivor_clone::components::*;
use yet_another_vampire_survivor_clone::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use yet_another_vampire_survivor_clone::enemies::enemy::EnemyPlugin;
use yet_another_vampire_survivor_clone::gems::boss_gem::GemsBossPlugin;
use yet_another_vampire_survivor_clone::gems::gem::GemsPlugin;
use yet_another_vampire_survivor_clone::items::item::ItemsPlugin;
use yet_another_vampire_survivor_clone::math_utils::get_random_position_in_screen;
use yet_another_vampire_survivor_clone::players::player::PlayerPlugin;
use yet_another_vampire_survivor_clone::ui::ui_enemy::UiEnemyPlugin;
use yet_another_vampire_survivor_clone::ui::ui_fps::UiFPSPlugin;
use yet_another_vampire_survivor_clone::ui::ui_global_timer::UiGlobalTimerPlugin;
use yet_another_vampire_survivor_clone::ui::ui_level_up::UiLevelUpPlugin;
use yet_another_vampire_survivor_clone::ui::ui_player::UiPlayerPlugin;
use yet_another_vampire_survivor_clone::ui::ui_update_weapon_up::UiUpdateWeaponPlugin;
use yet_another_vampire_survivor_clone::waves::waves::WavesPlugin;
use yet_another_vampire_survivor_clone::waves::waves_map_1::WavesMap1Plugin;
use yet_another_vampire_survivor_clone::weapons::arcane_missile::ArcaneMissilePlugin;
use yet_another_vampire_survivor_clone::weapons::bouncing_ball::BouncingBallPlugin;
use yet_another_vampire_survivor_clone::weapons::chain_lightning::ChainLightningPlugin;
use yet_another_vampire_survivor_clone::weapons::claw::WeaponClawPlugin;
use yet_another_vampire_survivor_clone::weapons::fire_area::WeaponFireAreaPlugin;
use yet_another_vampire_survivor_clone::weapons::fire_boots::FireBootsPlugin;
use yet_another_vampire_survivor_clone::weapons::generic_systems::GenericWeaponPlugin;
use yet_another_vampire_survivor_clone::weapons::light_sword::LightSwordsPlugin;
use yet_another_vampire_survivor_clone::weapons::shuriken::ShurikenPlugin;
use yet_another_vampire_survivor_clone::weapons::slow_dome::SlowDomePlugin;

fn main() {
    App::new()
        // bevy plugin
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "yet another vampire".into(),
                        resolution: (SCREEN_WIDTH as f32 * 3.0, SCREEN_HEIGHT as f32 * 3.0).into(),
                        // resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        // States
        .init_state::<GameState>()
        // Events
        .add_event::<EnemyDied>()
        .add_event::<EnemyBossDied>()
        .add_event::<CollectExperience>()
        .add_event::<EnemyReceivedDamage>()
        .add_event::<PlayerReceivedDamage>()
        .add_event::<SpawnEnemy>()
        .add_event::<ItemPickup>()
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
        // items
        .add_plugins(ItemsPlugin)
        // Waves
        .add_plugins(WavesPlugin)
        .add_plugins(WavesMap1Plugin) // Temp
        // Enemies plugin
        .add_plugins(EnemyPlugin)
        // UI
        .add_plugins(UiEnemyPlugin)
        .add_plugins(UiPlayerPlugin)
        .add_plugins(UiLevelUpPlugin)
        .add_plugins(UiGlobalTimerPlugin)
        .add_plugins(UiUpdateWeaponPlugin)
        // animation
        .add_plugins(AnimationSimplePlugin)
        // gems
        .add_plugins(GemsPlugin)
        .add_plugins(GemsBossPlugin)
        // Weapons
        .add_plugins(GenericWeaponPlugin)
        .add_plugins(WeaponClawPlugin)
        .add_plugins(WeaponFireAreaPlugin)
        .add_plugins(ArcaneMissilePlugin)
        .add_plugins(ShurikenPlugin)
        .add_plugins(ChainLightningPlugin)
        .add_plugins(SlowDomePlugin)
        .add_plugins(BouncingBallPlugin)
        .add_plugins(FireBootsPlugin)
        .add_plugins(LightSwordsPlugin)
        // Setup
        // .add_systems(Startup, setup)
        // test
        .insert_resource(Time::<Fixed>::from_hz(64.0))
        .add_systems(Startup, background)
        .add_systems(Update, debug)
        // .add_plugins(
        //     SteppingPlugin::default()
        //         .add_schedule(Update)
        //         .add_schedule(FixedUpdate)
        //         .at(Val::Percent(35.0), Val::Percent(50.0)),
        // )
        .run();
}

fn background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("map_1/png/background_level_1.png");

    let parent = commands
        .spawn((
            TransformBundle {
                local: Default::default(),
                global: Default::default(),
            },
            InheritedVisibility::default(),
            Name::new("Background parent"),
        ))
        .id();

    for i in -3..3 {
        for y in -3..3 {
            let child = commands
                .spawn((
                    SpriteBundle {
                        texture: texture.clone(),
                        transform: Transform::from_xyz(i as f32 * 1024.0, y as f32 * 1024.0, -1.0),
                        ..default()
                    },
                    Name::new("Background"),
                ))
                .id();
            commands.entity(parent).push_children(&[child]);
        }
    }
}

fn debug(
    mut windows: Query<&mut Window>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_weapons: ResMut<PlayerWeapons>,
    mut enemy_died: EventWriter<EnemyDied>,
    mut spawn_enemy: EventWriter<SpawnEnemy>,
    mut item_pickup: EventWriter<ItemPickup>,
    mut next_state: ResMut<NextState<GameState>>,
    // mut gizmos: Gizmos,
) {
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
    if keyboard_input.just_pressed(KeyCode::Digit8) {
        player_weapons.weapons.push(WeaponsTypes::FireBoots);
    }
    if keyboard_input.just_pressed(KeyCode::Digit9) {
        player_weapons.weapons.push(WeaponsTypes::LightSwords);
    }

    if keyboard_input.just_pressed(KeyCode::KeyP) {
        spawn_enemy.send(SpawnEnemy {
            enemy_types: EnemyTypes::Bat,
        });
    }
    if keyboard_input.just_pressed(KeyCode::KeyO) {
        spawn_enemy.send(SpawnEnemy {
            enemy_types: EnemyTypes::Bee,
        });
    }
    if keyboard_input.just_pressed(KeyCode::KeyI) {
        spawn_enemy.send(SpawnEnemy {
            enemy_types: EnemyTypes::Golem,
        });
    }
    if keyboard_input.just_pressed(KeyCode::KeyU) {
        spawn_enemy.send(SpawnEnemy {
            enemy_types: EnemyTypes::Rabbit,
        });
    }
    if keyboard_input.just_pressed(KeyCode::KeyY) {
        spawn_enemy.send(SpawnEnemy {
            enemy_types: EnemyTypes::Skull,
        });
    }
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        spawn_enemy.send(SpawnEnemy {
            enemy_types: EnemyTypes::BossWolf,
        });
    }

    let mut window = windows.single_mut();

    if keyboard_input.just_pressed(KeyCode::F1) {
        window.mode = WindowMode::BorderlessFullscreen;
    }
    if keyboard_input.just_pressed(KeyCode::F2) {
        window.mode = WindowMode::Windowed;
    }

    if keyboard_input.pressed(KeyCode::KeyG) {
        enemy_died.send(EnemyDied {
            position: get_random_position_in_screen().extend(0.0),
            experience: 1,
        });
    }

    if keyboard_input.just_pressed(KeyCode::KeyX) {
        item_pickup.send(ItemPickup {
            item_key: "HEALTHY_GEM_STONE".to_string(),
            rarity: Rarity::Legendary,
        });
    }

    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        next_state.set(GameState::PlayerLevelUp);
    }
}
