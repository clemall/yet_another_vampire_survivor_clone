use crate::components::*;
use crate::constants::PROJECTILE_Z_INDEX;
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
            spawn_weapon.run_if(
                resource_exists_and_changed::<PlayerWeapons>.and_then(run_if_weapon_not_present),
            ),
        );
        app.add_systems(
            Update,
            (spawn_attack, apply_slow_aura_on_hit).run_if(in_state(GameState::Gameplay)),
        );
    }
}

fn run_if_weapon_not_present(
    player_weapons: Res<PlayerWeapons>,
    weapon: Query<(), With<SlowDomeSpawner>>,
) -> bool {
    player_weapons.weapons.contains(&WeaponsTypes::SlowDome) && weapon.is_empty()
}

fn spawn_weapon(mut commands: Commands, player_stats: Res<PlayerInGameStats>) {
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
        CanAttack,
        Name::new("Slow Dome Spawner"),
    ));
}

fn spawn_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: Query<&Transform, With<Player>>,
    mut spawner: Query<
        (Entity, &mut AttackAmmo),
        (
            With<SlowDomeSpawner>,
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

                let texture = asset_server.load("slow-dome.png");
                commands.spawn((
                    SpriteBundle {
                        texture,
                        transform: Transform {
                            translation: Vec3::new(
                                enemy_transform.translation.x,
                                enemy_transform.translation.y,
                                PROJECTILE_Z_INDEX,
                            ),
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
                    ProjectileDamage(15.0),
                    ProjectileTimeBetweenDamage {
                        timer: Timer::from_seconds(0.33, TimerMode::Repeating),
                    },
                    SlowDome,
                    Projectile,
                    ProjectilePierce,
                    ProjectileType(ProjectileTypes::SlowDome),
                    // TriggersOnHit{
                    //     auras_systems: vec![systems.slow_enemy]
                    // },
                    Name::new("Slow dome Attack"),
                ));
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
    mut eneny_hit_event: EventReader<OnEnemyHit>,
) {
    for event in eneny_hit_event.read() {
        if event.projectile_type != ProjectileTypes::SlowDome {
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
