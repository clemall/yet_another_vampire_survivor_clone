use crate::components::*;
use crate::constants::PROJECTILE_Z_INDEX;
use crate::math_utils::find_closest;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct BouncingBallSpawner;

#[derive(Component)]
pub struct BouncingBall;

pub struct BouncingBallPlugin;

impl Plugin for BouncingBallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_weapon.run_if(
                resource_exists_and_changed::<PlayerWeapons>.and_then(run_if_weapon_not_present),
            ),
        );
        app.add_systems(
            Update,
            (spawn_attack, duplicate_ball_on_hit).run_if(in_state(GameState::Gameplay)),
        );
    }
}

fn run_if_weapon_not_present(
    player_weapons: Res<PlayerWeapons>,
    weapon: Query<(), With<BouncingBallSpawner>>,
) -> bool {
    player_weapons.weapons.contains(&WeaponsTypes::BouncingBall) && weapon.is_empty()
}

fn spawn_weapon(mut commands: Commands, player_stats: Res<PlayerInGameStats>) {
    commands.spawn((
        BouncingBallSpawner,
        DelayBetweenAttacks {
            timer: Timer::from_seconds(0.3, TimerMode::Repeating),
        },
        AttackAmmo {
            size: 1 + player_stats.attack_amount,
            amount: 1,
            default_size: 1,
            reload_time: 2.0 * player_stats.attack_reload,
            default_reload_time: 2.0,
        },
        CanAttack,
        Name::new("Bouncing ball Spawner"),
    ));
}

fn spawn_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: Query<&Transform, With<Player>>,
    mut spawner: Query<
        (Entity, &mut AttackAmmo),
        (
            With<BouncingBallSpawner>,
            With<CanAttack>,
            Without<AttackSpawnerIsReloading>,
        ),
    >,
    mut enemies: Query<(Entity, &Transform), With<Enemy>>,
    player_stats: Res<PlayerInGameStats>,
) {
    let player_transform = player.single_mut();

    if let Ok((spawner_entity, mut attack_ammo)) = spawner.get_single_mut() {
        let mut enemies_lens = enemies.transmute_lens::<(Entity, &Transform)>();
        let closed_enemy: Option<Entity> = find_closest(
            player_transform.translation,
            enemies_lens.query(),
            300.0,
            None,
        );

        if let Some(closed_enemy) = closed_enemy {
            if let Ok((_enemy, enemy_transform)) = enemies.get(closed_enemy) {
                attack_ammo.amount -= 1;
                commands.entity(spawner_entity).remove::<CanAttack>();

                let texture = asset_server.load("bouncing_ball.png");
                let direction = (player_transform.translation.xy()
                    - enemy_transform.translation.xy())
                .normalize();
                commands
                    .spawn((
                        SpriteBundle {
                            texture,
                            transform: Transform {
                                translation: Vec3::new(
                                    player_transform.translation.x,
                                    player_transform.translation.y,
                                    PROJECTILE_Z_INDEX,
                                ),
                                scale: Vec3::splat(player_stats.area),
                                ..default()
                            },
                            ..default()
                        },
                        Sensor,
                        Collider::ball(16.0 / 2.0),
                        ProjectileBundleCollider::default(),
                        ProjectileLifetime {
                            timer: Timer::from_seconds(
                                8.0 * player_stats.attack_duration,
                                TimerMode::Once,
                            ),
                        },
                        ProjectileDamage(50.0),
                        ProjectilePierce,
                        BouncingBall,
                    ))
                    .insert((
                        Projectile,
                        ProjectileSpeed(100.0),
                        ProjectileDirection(direction),
                        ProjectileImpulse(700.0),
                        ProjectileFromWeapon(WeaponsTypes::BouncingBall),
                        Name::new("Bouncing ball Attack"),
                    ));
            }
        }
    }
}

fn duplicate_ball_on_hit(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut eneny_hit_event: EventReader<OnEnemyHit>,
    player_stats: Res<PlayerInGameStats>,
) {
    for event in eneny_hit_event.read() {
        if event.weapon_projectile_type != WeaponsTypes::BouncingBall {
            continue;
        }
        if let Ok((_enemy_entity, enemy_transform)) = enemies.get(event.enemy_entity) {
            for _index in 0..2 + player_stats.attack_amount {
                let texture = asset_server.load("bouncing_ball.png");
                let direction =
                    (enemy_transform.translation.xy() - event.projectile_position.xy()).normalize();
                commands
                    .spawn((
                        SpriteBundle {
                            texture,
                            transform: Transform {
                                translation: event.projectile_position,
                                ..default()
                            },
                            ..default()
                        },
                        Sensor,
                        Collider::ball(16.0 / 2.0),
                        ProjectileBundleCollider::default(),
                        ProjectileLifetime {
                            timer: Timer::from_seconds(
                                8.0 * player_stats.attack_duration,
                                TimerMode::Once,
                            ),
                        },
                        ProjectileDamage(25.0),
                        ProjectilePierce,
                        BouncingBall,
                    ))
                    .insert((
                        Projectile,
                        ProjectileSpeed(100.0),
                        ProjectileDirection(direction),
                        ProjectileImpulse(700.0),
                        ProjectileFromWeapon(WeaponsTypes::BouncingBallSplit),
                        Name::new("Bouncing ball duplicate Attack"),
                    ));
            }
        }
    }
}
