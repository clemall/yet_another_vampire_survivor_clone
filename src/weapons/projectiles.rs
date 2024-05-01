use crate::components::*;
use crate::enemies::enemy::enemy_death_check;
use crate::math_utils::simple_bezier;
use bevy::app::{App, Plugin, Update};
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::{PI, TAU};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_projectile_colliding_with_enemy.after(enemy_death_check),
                start_reload_attack_spawner,
                reloading_attack_spawner,
                projectile_lifetime_tick,
                projectile_despawn_after_lifetime,
                projectile_position_on_player,
                projectile_follow_player,
                projectile_move_toward_direction,
                projectile_move_around_player,
                projectile_move_spiral,
                // projectile_move_boomerang,
                projectile_move_toward_target_in_arc,
                projectile_rotate_on_self,
                projectile_update_area,
                weapons_update_stats,
                delay_between_attack_timer_tick,
            )
                .run_if(in_state(GameState::Gameplay)),
        );
        app.add_systems(PostUpdate, projectile_delete);
    }
}

fn projectile_delete(
    mut commands: Commands,
    projectiles: Query<Entity, (With<Projectile>, With<ProjectileDeleteMe>)>,
) {
    for projectile_entity in &projectiles {
        commands.entity(projectile_entity).despawn_recursive();
    }
}

fn delay_between_attack_timer_tick(
    mut commands: Commands,
    mut attack_timers: Query<(Entity, &mut DelayBetweenAttacks)>,
    time: Res<Time>,
) {
    for (entity, mut attack_timer) in &mut attack_timers {
        attack_timer.timer.tick(time.delta());
        if attack_timer.timer.just_finished() {
            commands.entity(entity).insert(CanAttack);
        }
    }
}

fn handle_projectile_colliding_with_enemy(
    mut commands: Commands,
    mut attacks: Query<
        (
            Entity,
            &Transform,
            &CollidingEntities,
            &ProjectileDamage,
            &ProjectileType,
            Option<&mut ProjectileTimeBetweenDamage>,
            Option<&mut AlreadyHitEnemies>,
            Option<&ProjectilePierce>,
            Option<&ProjectileImpulse>,
            // Option<&TriggersOnHit>,
        ),
        (With<Projectile>, Without<ColliderDisabled>),
    >,
    mut eneny_hit_event: EventWriter<OnEnemyHit>,
    player_stats: Res<PlayerInGameStats>,
    time: Res<Time>,
) {
    for (
        projectile_entity,
        projectile_transform,
        colliding_entities,
        projectile_damage,
        projectile_type,
        projectile_delay_between_damage,
        mut hit_enemies,
        should_projectile_pierce,
        projectile_impulse,
        // triggers_on_hit,
    ) in &mut attacks
    {
        if let Some(mut attack_timer) = projectile_delay_between_damage {
            attack_timer.timer.tick(time.delta());
            if !attack_timer.timer.just_finished() {
                // early return, attack not ready
                continue;
            }
        }

        for enemy_entity in colliding_entities.iter() {
            // Maybe check if the entity is an enemy
            // if let Ok(transform) = enemies.get_mut(enemy_entity) {}

            if let Some(hit_enemies) = hit_enemies.as_deref_mut() {
                if hit_enemies.seen.contains(&enemy_entity) {
                    continue;
                }
                hit_enemies.seen.push(enemy_entity);
            }
            eneny_hit_event.send(OnEnemyHit {
                enemy_entity,
                damage: projectile_damage.0 * player_stats.power,
                projectile_position: projectile_transform.translation,
                projectile_type: projectile_type.0,
                impulse: projectile_impulse.map(|projectile_impulse| projectile_impulse.0),
            });

            // if let Some(trigger) = triggers_on_hit{
            //     for aura_system in trigger.auras_systems.iter() {
            //         commands.run_system_with_input(*aura_system, PayloadOnHit{
            //             target: enemy_entity,
            //             target_position: None,
            //         });
            //     }
            // }

            match should_projectile_pierce {
                None => {
                    commands
                        .entity(projectile_entity)
                        .insert(ProjectileDeleteMe);
                }
                Some(_) => {}
            }
        }
    }
}

pub fn start_reload_attack_spawner(
    mut commands: Commands,
    mut attack_spawners: Query<(Entity, &AttackAmmo), Without<AttackSpawnerIsReloading>>,
) {
    for (entity, attack_ammo) in &mut attack_spawners {
        if attack_ammo.amount == 0 {
            commands.entity(entity).insert(AttackSpawnerIsReloading {
                timer: Timer::from_seconds(attack_ammo.reload_time, TimerMode::Once),
            });
        }
    }
}

fn reloading_attack_spawner(
    mut commands: Commands,
    mut attack_spawners: Query<
        (Entity, &mut AttackSpawnerIsReloading, &mut AttackAmmo),
        With<AttackSpawnerIsReloading>,
    >,
    time: Res<Time>,
) {
    for (entity, mut attack_reload, mut attack_ammo) in &mut attack_spawners {
        attack_reload.timer.tick(time.delta());

        if attack_reload.timer.just_finished() {
            attack_ammo.amount = attack_ammo.size;

            commands.entity(entity).remove::<AttackSpawnerIsReloading>();
        }
    }
}

fn projectile_lifetime_tick(
    mut projectiles: Query<&mut ProjectileLifetime, With<Projectile>>,
    time: Res<Time>,
) {
    for mut attack_duration in &mut projectiles {
        attack_duration.timer.tick(time.delta());
    }
}

fn projectile_despawn_after_lifetime(
    mut commands: Commands,
    mut projectiles: Query<(Entity, &mut ProjectileLifetime), With<Projectile>>,
) {
    for (entity, attack_duration) in &mut projectiles {
        if attack_duration.timer.just_finished() {
            commands.entity(entity).insert(ProjectileDeleteMe);
            // commands.entity(entity).despawn_recursive();
        }
    }
}

fn projectile_position_on_player(
    player: Query<&mut Transform, With<Player>>,
    mut projectiles: Query<&mut Transform, (With<ProjectilePositionOnPlayer>, Without<Player>)>,
) {
    for mut transform in &mut projectiles {
        if let Ok(player) = player.get_single() {
            transform.translation.x = player.translation.x;
            transform.translation.y = player.translation.y;
        }
    }
}

fn projectile_follow_player(
    player: Query<&mut Transform, With<Player>>,
    mut projectiles: Query<
        (&mut Transform, &ProjectileSpeed),
        (With<ProjectileFollowPlayer>, Without<Player>),
    >,
    time: Res<Time>,
) {
    let player_transform = player.single();
    for (mut transform, speed) in &mut projectiles {
        let direction = (transform.translation.truncate()
            - player_transform.translation.truncate())
        .normalize();

        let mut velocity = Vec2::ZERO;

        velocity.x = direction.x * time.delta_seconds() * speed.0;
        velocity.y = direction.y * time.delta_seconds() * speed.0;

        transform.translation -= velocity.extend(0.0);
    }
}

fn projectile_move_toward_direction(
    mut projectiles: Query<
        (&mut Transform, &ProjectileSpeed, &ProjectileDirection),
        With<Projectile>,
    >,
    time: Res<Time>,
) {
    for (mut transform, speed, direction) in &mut projectiles {
        transform.translation.x -= direction.x * time.delta_seconds() * speed.0;
        transform.translation.y -= direction.y * time.delta_seconds() * speed.0;
    }
}

fn projectile_move_toward_target_in_arc(
    mut commands: Commands,
    mut arcane_missiles: Query<
        (
            Entity,
            &mut Transform,
            &ProjectileTarget,
            &mut ProjectileSpeedAsDuration,
            &ProjectileOrigin,
            &ProjectileControlPoint,
        ),
        (With<Projectile>, Without<Enemy>),
    >,
    enemies: Query<&Transform, (With<Enemy>, Without<Projectile>)>,
    time: Res<Time>,
) {
    for (
        arcane_missile_entity,
        mut transform,
        projectile_target,
        mut projectile_speed_as_duration,
        projectile_origin,
        projectile_control_point,
    ) in &mut arcane_missiles
    {
        if let Ok(enemy_transform) = enemies.get(projectile_target.0) {
            projectile_speed_as_duration.timer.tick(time.delta());

            let direction = (transform.translation.truncate()
                - enemy_transform.translation.truncate())
            .normalize();

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

fn projectile_move_around_player(
    mut projectiles: Query<
        (
            &mut Transform,
            &ProjectileSpeed,
            &ProjectileRotateAroundPlayer,
        ),
        With<Projectile>,
    >,
    player: Query<&mut Transform, (With<Player>, Without<Projectile>)>,
    time: Res<Time>,
) {
    for (mut transform, speed, projectile_rotate_around_player) in &mut projectiles {
        if let Ok(player_transform) = player.get_single() {
            transform.translation.x = (projectile_rotate_around_player.angle
                + time.elapsed().as_secs_f32() * speed.0)
                .sin()
                * projectile_rotate_around_player.distance;
            transform.translation.y = (projectile_rotate_around_player.angle
                + time.elapsed().as_secs_f32() * speed.0)
                .cos()
                * projectile_rotate_around_player.distance;

            transform.translation.x += player_transform.translation.x;
            transform.translation.y += player_transform.translation.y;
        }
    }
}

fn projectile_move_spiral(
    mut projectiles: Query<
        (
            &mut Transform,
            &ProjectileSpeed,
            &mut ProjectileSpiralAroundPlayer,
            &ProjectileOrigin,
        ),
        With<Projectile>,
    >,
    time: Res<Time>,
) {
    for (mut transform, speed, mut projectile_spiral_around_player, projectile_origin) in
        &mut projectiles
    {
        transform.translation.x =
            (projectile_spiral_around_player.angle + time.elapsed().as_secs_f32() * speed.0).sin()
                * projectile_spiral_around_player.distance;
        transform.translation.y =
            (projectile_spiral_around_player.angle + time.elapsed().as_secs_f32() * speed.0).cos()
                * projectile_spiral_around_player.distance;

        transform.translation.x += projectile_origin.x;
        transform.translation.y += projectile_origin.y;

        projectile_spiral_around_player.distance +=
            projectile_spiral_around_player.spiral_speed * time.delta_seconds();
    }
}

//
// fn projectile_move_boomerang(
//     mut projectiles: Query<(
//         &mut Transform,
//         &ProjectileSpeed,
//         &mut ProjectileRotateAroundPlayer,
//         &ProjectileOrigin,
//     ), (With<Projectile>, With<ProjectileSpeed>)>,
//     time: Res<Time>,
// ) {
//     for (
//         mut transform,
//         speed,
//         mut projectile_rotate_around_player,
//         origin,
//     )  in &mut projectiles {
//         let angle:f32 = 0.02;
//         // let angle:f32 = 0.1;
//         direction.x = angle.cos() * direction.x - angle.sin() * direction.y;
//         direction.y = angle.sin() * direction.x + angle.cos() * direction.y;
//         // **direction = direction.normalize();
//
//
//         transform.translation.x += direction.x * time.delta_seconds() * speed.0;
//         transform.translation.y += direction.y * time.delta_seconds() * speed.0;
//         // transform.translation.x -= transform.translation.x.cos();
//         // transform.translation.y -= transform.translation.y.cos();
//     }
// }

fn projectile_rotate_on_self(
    mut projectiles: Query<(&mut Transform, &ProjectileRotateOnSelf)>,
    time: Res<Time>,
) {
    for (mut transform, speed) in &mut projectiles {
        transform.rotate_z(speed.0 * TAU * time.delta_seconds());
    }
}

// update area of weapons that doesn't spawn projectile.
fn projectile_update_area(
    mut projectiles: Query<&mut Transform, With<ProjectileliveForever>>,
    player_stats: Res<PlayerInGameStats>,
) {
    if !player_stats.is_changed() {
        return;
    }

    for mut transform in &mut projectiles {
        transform.scale = Vec3::splat(player_stats.area);
    }
}

fn weapons_update_stats(
    mut attack_ammos: Query<&mut AttackAmmo>,
    player_stats: Res<PlayerInGameStats>,
) {
    if !player_stats.is_changed() {
        return;
    }
    for mut attack_ammo in &mut attack_ammos {
        attack_ammo.reload_time = attack_ammo.default_reload_time * player_stats.attack_reload;
        attack_ammo.size = attack_ammo.default_size + player_stats.attack_amount;
    }
}
