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
use yet_another_vampire_survivor_clone::ui::ui_choose_weapon::UiChooseWeaponPlugin;
use yet_another_vampire_survivor_clone::ui::ui_enemy::UiEnemyPlugin;
use yet_another_vampire_survivor_clone::ui::ui_fps::UiFPSPlugin;
use yet_another_vampire_survivor_clone::ui::ui_global_timer::UiGlobalTimerPlugin;
use yet_another_vampire_survivor_clone::ui::ui_level_up::UiLevelUpPlugin;
use yet_another_vampire_survivor_clone::ui::ui_main_menu::UiMainMenuPlugin;
use yet_another_vampire_survivor_clone::ui::ui_player::UiPlayerPlugin;
use yet_another_vampire_survivor_clone::ui::ui_update_weapon_up::UiUpdateWeaponPlugin;
use yet_another_vampire_survivor_clone::waves::waves::WavesPlugin;
use yet_another_vampire_survivor_clone::waves::waves_map_1::WavesMap1Plugin;
use yet_another_vampire_survivor_clone::weapons::weapons::WeaponsPlugin;

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
        .add_event::<OnEnemyDied>()
        .add_event::<OnEnemyBossDied>()
        .add_event::<OnCollectExperience>()
        .add_event::<OnEnemyHit>()
        .add_event::<OnPlayerReceivedDamage>()
        .add_event::<OnSpawnEnemy>()
        .add_event::<OnItemPickup>()
        .add_event::<OnUpgradePickup>()
        .add_event::<OnWeaponPickup>()
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
        .add_plugins(UiMainMenuPlugin)
        .add_plugins(UiPlayerPlugin)
        .add_plugins(UiLevelUpPlugin)
        .add_plugins(UiGlobalTimerPlugin)
        .add_plugins(UiUpdateWeaponPlugin)
        .add_plugins(UiChooseWeaponPlugin)
        // animation
        .add_plugins(AnimationSimplePlugin)
        // gems
        .add_plugins(GemsPlugin)
        .add_plugins(GemsBossPlugin)
        // Weapons
        .add_plugins(WeaponsPlugin)
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
    mut weapon_upgrades: ResMut<PlayerUpgradeWeapons>,
    mut enemy_died: EventWriter<OnEnemyDied>,
    mut spawn_enemy: EventWriter<OnSpawnEnemy>,
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
    if keyboard_input.just_pressed(KeyCode::Digit5) && keyboard_input.pressed(KeyCode::ShiftLeft) {
        weapon_upgrades
            .upgrades
            .push(WeaponsUpgradesTypes::ChainLightningExtraAmmo);

        weapon_upgrades
            .upgrades
            .push(WeaponsUpgradesTypes::ChainLightningTriple);
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
        spawn_enemy.send(OnSpawnEnemy {
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
        enemy_died.send(OnEnemyDied {
            position: get_random_position_in_screen().extend(0.0),
            experience: 1,
        });
    }

    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        next_state.set(GameState::PlayerLevelUp);
    }

    if keyboard_input.just_pressed(KeyCode::KeyW) {
        next_state.set(GameState::PlayerUpdateWeapon);
    }

    if keyboard_input.just_pressed(KeyCode::KeyE) {
        next_state.set(GameState::PlayerChooseWeapon);
    }
}
