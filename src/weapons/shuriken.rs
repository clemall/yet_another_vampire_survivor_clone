use crate::components::*;
use crate::constants::PROJECTILE_Z_INDEX;
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
                resource_exists_and_changed::<PlayerWeapons>.and_then(run_if_shuriken_not_present),
            ),
        );
        app.add_systems(
            Update,
            (
                spawn_shuriken_attack,
                handle_mini_shuriken_on_hit.run_if(run_if_upgrade_mini_shuriken_on_hit),
            )
                .run_if(in_state(GameState::Gameplay)),
        );

        app.add_systems(
            Update,
            handle_shuriken_extra_ammo_update.run_if(
                in_state(GameState::Gameplay)
                    .and_then(run_if_upgrade_extra_ammo)
                    .and_then(run_once()),
            ),
        );
    }
}

fn run_if_shuriken_not_present(
    player_weapons: Res<PlayerWeapons>,
    weapon: Query<(), With<ShurikenSpawner>>,
) -> bool {
    player_weapons.weapons.contains(&WeaponsTypes::Shuriken) && weapon.is_empty()
}

fn run_if_upgrade_extra_ammo(weapon_upgrades: Res<PlayerUpgradeWeapons>) -> bool {
    weapon_upgrades
        .upgrades
        .contains(&WeaponsUpgradesTypes::ShurikenExtraAmmo)
}

fn run_if_upgrade_mini_shuriken_on_hit(weapon_upgrades: Res<PlayerUpgradeWeapons>) -> bool {
    weapon_upgrades
        .upgrades
        .contains(&WeaponsUpgradesTypes::ShurikenSpawnMiniShuriken)
}

fn setup_shuriken_spawner(mut commands: Commands, player_stats: Res<PlayerInGameStats>) {
    commands.spawn((
        ShurikenSpawner,
        AttackAmmo {
            capacity: 4 + player_stats.attack_amount,
            amount: 4,
            initial_capacity: 4,
            reload_time: 7.0 * player_stats.attack_reload,
            initial_reload_time: 7.0,
        },
        Name::new("Shuriken Spawner"),
    ));
}

fn handle_shuriken_extra_ammo_update(mut spawner: Query<&mut AttackAmmo, With<ShurikenSpawner>>) {
    if let Ok(mut attack_ammo) = spawner.get_single_mut() {
        attack_ammo.initial_capacity += 4;
        attack_ammo.capacity += 4;
    }
}

fn spawn_shuriken_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: Query<&Transform, With<Player>>,
    mut spawner: Query<&mut AttackAmmo, (With<ShurikenSpawner>, Without<AttackSpawnerIsReloading>)>,
    player_stats: Res<PlayerInGameStats>,
    weapon_upgrades: Res<PlayerUpgradeWeapons>,
) {
    let player_transform = player.single_mut();

    if let Ok(mut attack_ammo) = spawner.get_single_mut() {
        // Protection from going below 0.
        // AttackReloadDuration can take 1 frame too much before being added to
        // the current spawner
        // if attack_ammo.amount == 0 {
        //     return;
        // }

        let texture = asset_server.load("shuriken_temp.png");

        while attack_ammo.amount > 0 {
            attack_ammo.amount -= 1;
            let incremental_angle = TAU / attack_ammo.capacity as f32;
            let angle = incremental_angle * attack_ammo.amount as f32;
            let direction = Vec2::from_angle(angle);

            let mut scale = Vec3::splat(player_stats.area);
            if weapon_upgrades
                .upgrades
                .contains(&WeaponsUpgradesTypes::ShurikenExtraLarge)
            {
                scale *= 1.5;
            }

            let projectile_id = commands
                .spawn((
                    SpriteBundle {
                        texture: texture.clone(),
                        transform: Transform {
                            translation: Vec3::new(
                                player_transform.translation.x,
                                player_transform.translation.y,
                                PROJECTILE_Z_INDEX,
                            ),
                            scale: scale,
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
                    ProjectileType(ProjectileTypes::Shuriken),
                    Shuriken,
                    ProjectileSpeed(150.0),
                    ProjectileDirection(direction),
                    // ProjectileRotateAroundPlayer {
                    //     angle,
                    //     distance: 40.0,
                    // },
                    ProjectileDamage(80.0),
                    ProjectileImpulse(3000.0),
                    ProjectilePierce,
                    ProjectileLifetime {
                        timer: Timer::from_seconds(
                            10.0 * player_stats.attack_duration,
                            TimerMode::Once,
                        ),
                    },
                    AlreadyHitEnemies { seen: Vec::new() },
                    ProjectileRotateOnSelf(2.0),
                    ProjectileOrigin(player_transform.translation),
                    Name::new("Shuriken Attack"),
                ))
                .id();

            if weapon_upgrades
                .upgrades
                .contains(&WeaponsUpgradesTypes::ShurikenSpiralAroundPlayer)
            {
                commands.entity(projectile_id).insert((
                    ProjectileSpiralAroundPlayer {
                        angle,
                        distance: 0.0,
                        spiral_speed: 70.0,
                    },
                    ProjectileSpeed(1.5),
                ));
            }

            // if weapon_upgrades
            //     .upgrades
            //     .contains(&WeaponsUpgradesTypes::ShurikenSpawnMiniShuriken)
            // {
            //     commands.entity(projectile_id).remove::<ProjectilePierce>();
            // }
        }
    }
}

fn handle_mini_shuriken_on_hit(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut eneny_hit_event: EventReader<OnEnemyHit>,
    player_stats: Res<PlayerInGameStats>,
    weapon_upgrades: Res<PlayerUpgradeWeapons>,
) {
    for event in eneny_hit_event.read() {
        if event.projectile_type != ProjectileTypes::Shuriken {
            continue;
        }

        let texture = asset_server.load("mini-shuriken.png");

        let mut scale = Vec3::splat(player_stats.area);
        if weapon_upgrades
            .upgrades
            .contains(&WeaponsUpgradesTypes::ShurikenExtraLarge)
        {
            scale *= 1.5;
        }

        let _projectile_id = commands
            .spawn((
                SpriteBundle {
                    texture,
                    transform: Transform {
                        translation: Vec3::new(
                            event.projectile_position.x,
                            event.projectile_position.y,
                            PROJECTILE_Z_INDEX,
                        ),
                        scale: scale,
                        ..default()
                    },
                    ..default()
                },
                Sensor,
                Collider::ball(32.0 / 2.0),
            ))
            .insert((
                Projectile,
                ProjectileBundleCollider::default(),
                ProjectileType(ProjectileTypes::ShurikenMini),
                Shuriken,
                ProjectileDamage(10.0),
                ProjectileImpulse(3000.0),
                ProjectilePierce,
                ProjectileLifetime {
                    timer: Timer::from_seconds(2.0 * player_stats.attack_duration, TimerMode::Once),
                },
                ProjectileTimeBetweenDamage {
                    timer: Timer::from_seconds(0.6, TimerMode::Repeating),
                },
                ProjectileRotateOnSelf(3.0),
                Name::new("Mini Shuriken Attack"),
            ))
            .id();
    }
}
