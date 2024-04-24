use crate::components::*;
use crate::math_utils::{find_circle_circle_intersections, find_closest, simple_bezier};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
pub struct ArcaneMissileSpawner;

#[derive(Component)]
pub struct ArcaneMissile;
pub struct ArcaneMissilePlugin;

impl Plugin for ArcaneMissilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup_arcane_missile_spawner.run_if(
                resource_exists_and_changed::<PlayerWeapons>
                    .and_then(run_if_arcane_missile_present),
            ),
        );
        app.add_systems(
            Update,
            (spawn_arcane_missile_attack, move_arcane_missile)
                .run_if(in_state(GameState::Gameplay)),
        );
    }
}

fn run_if_arcane_missile_present(
    player_weapons: Res<PlayerWeapons>,
    weapon: Query<(), With<ArcaneMissileSpawner>>,
) -> bool {
    player_weapons
        .weapons
        .contains(&WeaponsTypes::ArcaneMissile)
        && weapon.is_empty()
}

fn setup_arcane_missile_spawner(mut commands: Commands, player_stats: Res<PlayerInGameStats>) {
    commands.spawn((
        ArcaneMissileSpawner,
        DelayBetweenAttacks {
            timer: Timer::from_seconds(0.4, TimerMode::Repeating),
        },
        AttackAmmo {
            size: 3 + player_stats.attack_amount,
            amount: 3,
            default_size: 3,
            reload_time: 2.0 * player_stats.attack_reload,
            default_reload_time: 2.0,
        },
        CanAttack,
        ProjectileBendLeftOrRight(true),
        Name::new("Arcane missile Spawner"),
    ));
}

fn spawn_arcane_missile_attack(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    mut player: Query<&Transform, With<Player>>,
    mut spawner: Query<
        (   Entity,
            &mut AttackAmmo,
            &mut ProjectileBendLeftOrRight,
        ),
        (
            With<ArcaneMissileSpawner>,
            With<CanAttack>,
            Without<AttackSpawnerIsReloading>,
        ),
    >,
    mut enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    let player_transform = player.single_mut();

    if let Ok((spawner_entity, mut attack_ammo, mut projectile_orientation)) =
        spawner.get_single_mut()
    {
            // get closed enemy
            let mut enemies_lens = enemies.transmute_lens::<(Entity, &Transform)>();
            let closed_enemy: Option<Entity> = find_closest(
                player_transform.translation,
                enemies_lens.query(),
                300.0,
                None,
            );

            if let Some(closed_enemy) = closed_enemy {
                let texture = asset_server.load("arcane-missile.png");
                let layout =
                    TextureAtlasLayout::from_grid(Vec2::new(100.0, 100.0), 30, 1, None, None);
                let texture_atlas_layout = texture_atlas_layouts.add(layout);

                if let Ok((entity, enemy_transform)) = enemies.get(closed_enemy) {
                    let distance_enemy_player = enemy_transform
                        .translation
                        .distance(player_transform.translation);

                    let (control_point_1, control_point_2) = find_circle_circle_intersections(
                        player_transform.translation,
                        distance_enemy_player / 2.0 + 15.0,
                        enemy_transform.translation,
                        distance_enemy_player / 2.0 + 15.0,
                    );

                    let control_point = if projectile_orientation.0 {
                        control_point_1
                    } else {
                        control_point_2
                    };

                    **projectile_orientation = !projectile_orientation.0;

                    attack_ammo.amount -= 1;
                    commands.entity(spawner_entity).remove::<CanAttack>();

                    commands
                        .spawn((
                            SpriteBundle {
                                texture,
                                transform: Transform {
                                    translation: Vec3::new(
                                        player_transform.translation.x,
                                        player_transform.translation.y,
                                        1.0,
                                    ),
                                    ..default()
                                },
                                ..default()
                            },
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: 0,
                            },
                            AnimationIndices {
                                first: 0,
                                last: 1,
                                is_repeating: true,
                            },
                            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                            Sensor,
                            Collider::ball(18.0 / 2.0),
                            ArcaneMissile,
                        ))
                        .insert((
                            Projectile,
                            ProjectileFromWeapon(WeaponsTypes::ArcaneMissile),
                            ProjectileDeleteOnHit,
                            ProjectileDamage(50.0),
                            ProjectileTarget(entity),
                            ProjectileOrigin(player_transform.translation),
                            ProjectileControlPoint(control_point),
                            ProjectileImpulse(700.0),
                            ProjectileSpeedAsDuration {
                                timer: Timer::from_seconds(0.3, TimerMode::Once),
                            },
                            // ProjectileLifetime {
                            //     timer:Timer::from_seconds(0.31, TimerMode::Once),
                            // },
                            ProjectileBundleCollider::default(),
                            ProjectileFixedScale,
                            Name::new("Arcane missile Attack"),
                        ));
                }
            }
    }
}

fn move_arcane_missile(
    mut commands: Commands,
    mut arcane_missiles: Query<
        (
            Entity,
            &mut Transform,
            &mut Sprite,
            &ProjectileTarget,
            &mut ProjectileSpeedAsDuration,
            &ProjectileOrigin,
            &ProjectileControlPoint,
        ),
        (With<ArcaneMissile>, Without<Enemy>),
    >,
    enemies: Query<&Transform, (With<Enemy>, Without<ArcaneMissile>)>,
    time: Res<Time>,
    // mut gizmos: Gizmos,
) {
    for (
        arcane_missile_entity,
        mut transform,
        mut sprite,
        projectile_target,
        mut projectile_speed_as_duration,
        projectile_origin,
        projectile_control_point,
    ) in &mut arcane_missiles
    {
        if let Ok(enemy_transform) = enemies.get(projectile_target.0) {
            projectile_speed_as_duration.timer.tick(time.delta());

            //debug
            // gizmos.circle_2d(Vec2::new(projectile_control_point.0.x,projectile_control_point.0.y),3.0, Color::WHITE);

            // let distance_enemy_player = enemy_transform.translation.distance(projectile_origin.0);
            // gizmos.circle_2d(Vec2::new(projectile_origin.x,projectile_origin.y),distance_enemy_player/2.0 + 10.0, Color::PURPLE);
            // gizmos.circle_2d(Vec2::new(enemy_transform.translation.x,enemy_transform.translation.y),distance_enemy_player/2.0 + 10.0, Color::RED);

            let direction = (transform.translation.truncate()
                - enemy_transform.translation.truncate())
            .normalize();
            // sprite.flip_x = direction.x < 0.0;

            transform.translation = simple_bezier(
                projectile_origin.0,
                projectile_control_point.0,
                enemy_transform.translation,
                projectile_speed_as_duration.timer.fraction(),
            );
            // rotate the projectile toward the enemy
            transform.rotation = Quat::from_rotation_z(direction.to_angle() - PI)
        } else {
            // delete projectile
            commands.entity(arcane_missile_entity).despawn_recursive();
        }
    }
}
