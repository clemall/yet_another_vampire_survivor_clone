use crate::components::*;
use crate::constants::PROJECTILE_Z_INDEX;
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
            spawn_weapon.run_if(
                resource_exists_and_changed::<PlayerWeapons>.and_then(run_if_weapon_not_present),
            ),
        );
        app.add_systems(
            Update,
            (spawn_attack,).run_if(in_state(GameState::Gameplay)),
        );
    }
}

fn run_if_weapon_not_present(
    player_weapons: Res<PlayerWeapons>,
    weapon: Query<(), With<LightSwordsSpawner>>,
) -> bool {
    player_weapons.weapons.contains(&WeaponsTypes::LightSwords) && weapon.is_empty()
}

fn spawn_weapon(mut commands: Commands, player_stats: Res<PlayerInGameStats>) {
    commands.spawn((
        LightSwordsSpawner,
        DelayBetweenAttacks {
            timer: Timer::from_seconds(0.3, TimerMode::Repeating),
        },
        AttackAmmo {
            size: 3 + player_stats.attack_amount,
            amount: 3,
            default_size: 3,
            reload_time: 5.0 * player_stats.attack_reload,
            default_reload_time: 5.0,
        },
        Name::new("Light Sword Spawner"),
    ));
}

fn spawn_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: Query<(&Transform, &Player)>,
    mut spawner: Query<
        (Entity, &mut AttackAmmo),
        (
            With<LightSwordsSpawner>,
            With<CanAttack>,
            Without<AttackSpawnerIsReloading>,
        ),
    >,
    player_stats: Res<PlayerInGameStats>,
) {
    let (player_transform, player) = player.single_mut();

    if let Ok((spawner_entity, mut attack_ammo)) = spawner.get_single_mut() {
        // if attack_ammo.amount == 0 {
        //     return;
        // }

        let texture = asset_server.load("sword-of-light.png");

        attack_ammo.amount -= 1;
        commands.entity(spawner_entity).remove::<CanAttack>();

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
                            PROJECTILE_Z_INDEX,
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
                ProjectileType(ProjectileTypes::LightSwords),
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
