use crate::components::*;
use crate::math_utils::find_closest;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct SlowDomeSpawner;

#[derive(Component)]
pub struct SlowDome;
pub struct SlowDomePlugin;

impl Plugin for SlowDomePlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(
        //     Startup, setup_on_hit,
        // );
        app.add_systems(
            Update,
            setup_slow_dome_spawner.run_if(
                resource_exists_and_changed::<PlayerWeapons>.and_then(run_if_slow_dome_present),
            ),
        );
        app.add_systems(
            Update,
            (spawn_slow_dome_attack, apply_slow_aura_on_hit).run_if(in_state(GameState::Gameplay)),
        );
    }
}

fn run_if_slow_dome_present(
    player_weapons: Res<PlayerWeapons>,
    weapon: Query<(), With<SlowDomeSpawner>>,
) -> bool {
    player_weapons.weapons.contains(&WeaponsTypes::SlowDome) && weapon.is_empty()
}

fn setup_slow_dome_spawner(mut commands: Commands, player_stats: Res<PlayerInGameStats>) {
    commands.spawn((
        SlowDomeSpawner,
        DelayBetweenAttacks {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        },
        AttackAmmo {
            size: 1 + player_stats.attack_amount,
            default_size: 1,
            amount: 1,
            reload_time: 10.0 * player_stats.attack_reload,
            default_reload_time: 10.0,
        },
        Name::new("Slow Dome Spawner"),
    ));
}

fn spawn_slow_dome_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: Query<&Transform, With<Player>>,
    mut spawner: Query<
        (&mut DelayBetweenAttacks, &mut AttackAmmo),
        (With<SlowDomeSpawner>, Without<AttackReloadDuration>),
    >,
    mut enemies: Query<(Entity, &Transform), With<Enemy>>,
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

                    let texture = asset_server.load("slow-dome.png");
                    commands.spawn((
                        SpriteBundle {
                            texture,
                            transform: Transform {
                                translation: enemy_transform.translation,
                                scale: Vec3::splat(player_stats.area),
                                ..default()
                            },
                            ..default()
                        },
                        Sensor,
                        Collider::ball(95.0 / 2.0),
                        ProjectileBundleCollider::default(),
                        ProjectileLifetime {
                            timer: Timer::from_seconds(
                                8.0 * player_stats.attack_duration,
                                TimerMode::Once,
                            ),
                        },
                        ProjectileDamage(1.0),
                        ProjectileTimeBetweenDamage {
                            timer: Timer::from_seconds(0.33, TimerMode::Repeating),
                        },
                        SlowDome,
                        Projectile,
                        ProjectileFromWeapon(WeaponsTypes::SlowDome),
                        // TriggersOnHit{
                        //     auras_systems: vec![systems.slow_enemy]
                        // },
                        Name::new("Slow dome Attack"),
                    ));
                }
            }
        }
    }
}

// fn apply_slow_on_hit(
//     In(payload): In<PayloadOnHit>,
//     mut commands: Commands,
// ){
//     // commands.entity(payload.target).insert(VelocityAura {
//     //     value: 0.5,
//     //     lifetime: Timer::from_seconds(2.0, TimerMode::Once),
//     // },);
// }

fn apply_slow_aura_on_hit(
    mut commands: Commands,
    enemies: Query<Entity, With<Enemy>>,
    mut eneny_hit_event: EventReader<EnemyReceivedDamage>,
) {
    for event in eneny_hit_event.read() {
        if event.weapon_projectile_type != WeaponsTypes::SlowDome {
            continue;
        }
        if let Ok(enemy_entity) = enemies.get(event.enemy_entity) {
            commands.entity(enemy_entity).try_insert(VelocityAura {
                value: 0.5,
                lifetime: Timer::from_seconds(2.0, TimerMode::Once),
            });
        }
    }
}
