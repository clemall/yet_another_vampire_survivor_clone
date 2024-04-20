use crate::components::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::TAU;

#[derive(Component)]
pub struct ShurikenSpawner;

#[derive(Component)]
pub struct Shuriken;

pub struct ShurikenPlugin;

impl Plugin for ShurikenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup_shuriken_spawner.run_if(
                resource_exists_and_changed::<PlayerWeapons>.and_then(run_if_shuriken_present),
            ),
        );
        app.add_systems(
            Update,
            (spawn_shuriken_attack, shuriken_update_stats).run_if(in_state(GameState::Gameplay)),
        );
    }
}

fn run_if_shuriken_present(
    player_weapons: Res<PlayerWeapons>,
    weapon: Query<(), With<ShurikenSpawner>>,
) -> bool {
    player_weapons.weapons.contains(&WeaponsTypes::Shuriken) && weapon.is_empty()
}

fn setup_shuriken_spawner(mut commands: Commands, player_stats: Res<PlayerInGameStats>) {
    commands.spawn((
        ShurikenSpawner,
        AttackAmmo {
            size: 8,
            amount: 8,
            reload_time: 10.0 * player_stats.attack_reload,
        },
        Name::new("Shuriken Spawner"),
    ));
}

fn shuriken_update_stats(
    mut attack_ammos: Query<&mut AttackAmmo, With<ShurikenSpawner>>,
    player_stats: Res<PlayerInGameStats>,
) {
    if !player_stats.is_changed() {
        return;
    }
    for mut attack_ammo in &mut attack_ammos {
        attack_ammo.reload_time = 10.0 * player_stats.attack_reload;
    }
}

fn spawn_shuriken_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: Query<&Transform, With<Player>>,
    mut spawner: Query<&mut AttackAmmo, (With<ShurikenSpawner>, Without<AttackReloadDuration>)>,
    player_stats: Res<PlayerInGameStats>,
) {
    let player_transform = player.single_mut();

    if let Ok(mut attack_ammo) = spawner.get_single_mut() {
        // Protection from going below 0.
        // AttackReloadDuration can take 1 frame too much before being added to
        // the current spawner
        if attack_ammo.amount == 0 {
            return;
        }

        let texture = asset_server.load("shuriken_temp.png");

        while attack_ammo.amount > 0 {
            attack_ammo.amount -= 1;
            let incremental_angle = TAU / attack_ammo.size as f32;
            let angle = incremental_angle * attack_ammo.amount as f32;
            // let direction = Vec2::from_angle(angle);

            commands
                .spawn((
                    SpriteBundle {
                        texture: texture.clone(),
                        transform: Transform {
                            translation: Vec3::new(
                                player_transform.translation.x,
                                player_transform.translation.y,
                                1.0,
                            ),
                            scale: Vec3::splat(player_stats.area),
                            ..default()
                        },
                        ..default()
                    },
                    Sensor,
                    Collider::ball(32.0 / 2.0),
                    ProjectileBundleCollider::default(),
                ))
                .insert((
                    Projectile,
                    ProjectileType(WeaponsTypes::Shuriken),
                    Shuriken,
                    ProjectileSpeed(2.0),
                    // ProjectileDirection(direction),
                    // ProjectileRotateAroundPlayer{
                    //     angle,
                    //     distance: 40.0,
                    // },
                    ProjectileSpiralAroundPlayer {
                        angle,
                        distance: 0.0,
                        spiral_speed: 60.0,
                    },
                    ProjectileDamage(10.0),
                    ProjectileImpulse(200.0),
                    ProjectileLifetime {
                        timer: Timer::from_seconds(
                            10.0 * player_stats.attack_duration,
                            TimerMode::Once,
                        ),
                    },
                    AlreadyHitEnemies { seen: Vec::new() },
                    ProjectileRotateOnSelf,
                    ProjectileOrigin(player_transform.translation),
                    Name::new("Shuriken Attack"),
                ));
        }
    }
}
