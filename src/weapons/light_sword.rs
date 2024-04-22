use crate::components::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct LightSwordsSpawner;

#[derive(Component)]
pub struct LightSwords;

pub struct LightSwordsPlugin;

impl Plugin for LightSwordsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup_light_swords_spawner.run_if(
                resource_exists_and_changed::<PlayerWeapons>.and_then(run_if_light_swords_present),
            ),
        );
        app.add_systems(
            Update,
            (spawn_light_swords_attack, light_swords_update_stats)
                .run_if(in_state(GameState::Gameplay)),
        );
    }
}

fn run_if_light_swords_present(
    player_weapons: Res<PlayerWeapons>,
    weapon: Query<(), With<LightSwordsSpawner>>,
) -> bool {
    player_weapons.weapons.contains(&WeaponsTypes::LightSwords) && weapon.is_empty()
}

fn setup_light_swords_spawner(mut commands: Commands, player_stats: Res<PlayerInGameStats>) {
    commands.spawn((
        LightSwordsSpawner,
        DelayBetweenAttacks {
            timer: Timer::from_seconds(0.3, TimerMode::Repeating),
        },
        AttackAmmo {
            size: 3,
            amount: 3,
            reload_time: 5.0 * player_stats.attack_reload,
        },
        Name::new("Light Sword Spawner"),
    ));
}

fn light_swords_update_stats(
    mut attack_ammos: Query<&mut AttackAmmo, With<LightSwordsSpawner>>,
    player_stats: Res<PlayerInGameStats>,
) {
    if !player_stats.is_changed() {
        return;
    }
    for mut attack_ammo in &mut attack_ammos {
        attack_ammo.reload_time = 5.0 * player_stats.attack_reload;
    }
}

fn spawn_light_swords_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: Query<(&Transform, &Player)>,
    mut spawner: Query<
        (&mut DelayBetweenAttacks, &mut AttackAmmo),
        (With<LightSwordsSpawner>, Without<AttackReloadDuration>),
    >,
    time: Res<Time>,
    player_stats: Res<PlayerInGameStats>,
) {
    let (player_transform, player) = player.single_mut();

    if let Ok((mut attack_timer, mut attack_ammo)) = spawner.get_single_mut() {
        attack_timer.timer.tick(time.delta());
        if attack_timer.timer.just_finished() {
            if attack_ammo.amount == 0 {
                return;
            }

            let texture = asset_server.load("sword-of-light.png");
            attack_ammo.amount -= 1;

            let direction: Vec2 = match player.facing {
                Facing::Left => Vec2::new(1.0, 0.0),
                Facing::Right => Vec2::new(-1.0, 0.0),
            };

            commands
                .spawn((
                    SpriteBundle {
                        texture: texture.clone(),
                        transform: Transform {
                            translation: Vec3::new(
                                player_transform.translation.x,
                                player_transform.translation.y
                                    + rand::thread_rng().gen_range(-10.0..10.0),
                                1.0,
                            ),
                            scale: Vec3::splat(player_stats.area),
                            ..default()
                        },
                        sprite: Sprite {
                            flip_x: direction.x > 0.,
                            ..default()
                        },
                        ..default()
                    },
                    Sensor,
                    Collider::capsule_x(12.0, 15.0 / 2.0),
                    ProjectileBundleCollider::default(),
                ))
                .insert((
                    Projectile,
                    ProjectileFromWeapon(WeaponsTypes::LightSwords),
                    LightSwords,
                    ProjectileDamage(40.0),
                    ProjectileImpulse(120.0),
                    ProjectileLifetime {
                        timer: Timer::from_seconds(
                            10.0 * player_stats.attack_duration,
                            TimerMode::Once,
                        ),
                    },
                    ProjectileSpeed(180.0),
                    ProjectileDirection(direction),
                    AlreadyHitEnemies { seen: Vec::new() },
                    ProjectileOrigin(player_transform.translation),
                    Name::new("Light swords Attack"),
                ));
        }
    }
}
