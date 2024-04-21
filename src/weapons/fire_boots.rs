use crate::components::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::TAU;

#[derive(Component)]
pub struct FireBootSpawner;

#[derive(Component)]
pub struct FireBoot;

pub struct FireBootsPlugin;

impl Plugin for FireBootsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup_fire_boot_spawner.run_if(
                resource_exists_and_changed::<PlayerWeapons>.and_then(run_if_fire_boots_present),
            ),
        );
        app.add_systems(
            Update,
            (spawn_fire_boots_attack, fire_boots_update_stats)
                .run_if(in_state(GameState::Gameplay)),
        );
    }
}

fn run_if_fire_boots_present(
    player_weapons: Res<PlayerWeapons>,
    weapon: Query<(), With<FireBootSpawner>>,
) -> bool {
    player_weapons.weapons.contains(&WeaponsTypes::FireBoots) && weapon.is_empty()
}

fn setup_fire_boot_spawner(mut commands: Commands, player_stats: Res<PlayerInGameStats>) {
    commands.spawn((
        FireBootSpawner,
        DelayBetweenAttacks {
            timer: Timer::from_seconds(0.42, TimerMode::Repeating),
        },
        AttackAmmo {
            size: 10,
            amount: 10,
            reload_time: 5.0 * player_stats.attack_reload,
        },
        Name::new("Shuriken Spawner"),
    ));
}

fn fire_boots_update_stats(
    mut attack_ammos: Query<&mut AttackAmmo, With<FireBootSpawner>>,
    player_stats: Res<PlayerInGameStats>,
) {
    if !player_stats.is_changed() {
        return;
    }
    for mut attack_ammo in &mut attack_ammos {
        attack_ammo.reload_time = 5.0 * player_stats.attack_reload;
    }
}

fn spawn_fire_boots_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: Query<&Transform, With<Player>>,
    mut spawner: Query<
        (&mut DelayBetweenAttacks, &mut AttackAmmo),
        (With<FireBootSpawner>, Without<AttackReloadDuration>),
    >,
    time: Res<Time>,
    player_stats: Res<PlayerInGameStats>,
) {
    let player_transform = player.single_mut();

    if let Ok((mut attack_timer, mut attack_ammo)) = spawner.get_single_mut() {
        attack_timer.timer.tick(time.delta());
        if attack_timer.timer.just_finished() {
            if attack_ammo.amount == 0 {
                return;
            }
            let texture = asset_server.load("shuriken_temp.png");

            attack_ammo.amount -= 1;

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
                    ProjectileFromWeapon(WeaponsTypes::FireBoots),
                    FireBoot,
                    ProjectileDamage(10.0),
                    ProjectileImpulse(20.0),
                    ProjectileLifetime {
                        timer: Timer::from_seconds(
                            5.0 * player_stats.attack_duration,
                            TimerMode::Once,
                        ),
                    },
                    ProjectileTimeBetweenDamage {
                        timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                    },
                    ProjectileOrigin(player_transform.translation),
                    Name::new("Fire boots Attack"),
                ));
        }
    }
}
