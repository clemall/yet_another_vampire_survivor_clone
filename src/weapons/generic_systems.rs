use crate::components::*;
use crate::enemies::enemy::enemy_death_check;
use bevy::app::{App, Plugin, Update};
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::TAU;

pub struct GenericWeaponPlugin;

impl Plugin for GenericWeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                projectile_apply_damage.after(enemy_death_check),
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
                projectile_rotate_on_self,
                weapon_update_area,
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

fn projectile_apply_damage(
    mut commands: Commands,
    mut attacks: Query<
        (
            Entity,
            &Transform,
            &CollidingEntities,
            &ProjectileDamage,
            &ProjectileFromWeapon,
            Option<&mut ProjectileTimeBetweenDamage>,
            Option<&mut AlreadyHitEnemies>,
            Option<&ProjectileDeleteOnHit>,
            Option<&ProjectileImpulse>,
            // Option<&TriggersOnHit>,
        ),
        (With<Projectile>, Without<ColliderDisabled>),
    >,
    mut enemy_received_damage: EventWriter<EnemyReceivedDamage>,
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
        should_delete_projectile,
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
                if hit_enemies.seen.contains(&enemy_entity.index()) {
                    continue;
                }
                hit_enemies.seen.push(enemy_entity.index());
            }
            enemy_received_damage.send(EnemyReceivedDamage {
                enemy_entity,
                damage: projectile_damage.0 * player_stats.power,
                projectile_position: projectile_transform.translation,
                weapon_projectile_type: projectile_type.0,
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

            if let Some(_should_delete) = should_delete_projectile {
                commands
                    .entity(projectile_entity)
                    .insert(ProjectileDeleteMe);
                // commands.entity(projectile_entity).despawn_recursive();
            }
        }
    }
}

pub fn start_reload_attack_spawner(
    mut commands: Commands,
    mut attack_spawners: Query<(Entity, &AttackAmmo), Without<AttackReloadDuration>>,
) {
    for (entity, attack_ammo) in &mut attack_spawners {
        if attack_ammo.amount == 0 {
            commands.entity(entity).insert(AttackReloadDuration {
                timer: Timer::from_seconds(attack_ammo.reload_time, TimerMode::Once),
            });
        }
    }
}

fn reloading_attack_spawner(
    mut commands: Commands,
    mut attack_spawners: Query<
        (Entity, &mut AttackReloadDuration, &mut AttackAmmo),
        With<AttackReloadDuration>,
    >,
    time: Res<Time>,
) {
    for (entity, mut attack_reload, mut attack_ammo) in &mut attack_spawners {
        attack_reload.timer.tick(time.delta());

        if attack_reload.timer.just_finished() {
            attack_ammo.amount = attack_ammo.size;

            commands.entity(entity).remove::<AttackReloadDuration>();
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

// update area of weapons that doesn't spawn projectile.
fn weapon_update_area(
    mut projectiles: Query<&mut Transform, (With<Projectile>, Without<ProjectileFixedScale>)>,
    player_stats: Res<PlayerInGameStats>,
) {
    if !player_stats.is_changed() {
        return;
    }

    for mut transform in &mut projectiles {
        transform.scale = Vec3::splat(player_stats.area);
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
    mut projectiles: Query<&mut Transform, (With<Projectile>, With<ProjectileRotateOnSelf>)>,
    time: Res<Time>,
) {
    for mut transform in &mut projectiles {
        transform.rotate_z(1.5 * TAU * time.delta_seconds());
    }
}
