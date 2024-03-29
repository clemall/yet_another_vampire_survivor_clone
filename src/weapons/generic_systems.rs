use bevy::app::{App, Plugin, Update};
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_rapier2d::dynamics::ExternalImpulse;
use bevy_rapier2d::geometry::{Collider, ColliderDisabled};
use bevy_rapier2d::pipeline::QueryFilter;
use bevy_rapier2d::plugin::RapierContext;
use crate::components::*;
use crate::enemies::enemy::{damage_enemy, enemy_death_check};


pub struct GenericWeaponPlugin;

impl Plugin for GenericWeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update, (
                weapon_damage.after(enemy_death_check),
                start_reload_attack_spawner,
                reloading_attack_spawner,
                projectile_lifetime_tick,
                projectile_despawn,
                projectile_follow_player,
            ).run_if(in_state(GameState::Gameplay)),
        );

    }
}



fn weapon_damage(
    mut commands: Commands,
    mut attacks: Query<(
        Entity,
        &Collider,
        &GlobalTransform,
        &ProjectileDamage,
        Option<&mut ProjectileTimeBetweenDamage>,
        Option<&mut AlreadyHitEnemies>,
        Option<&ProjectileDeleteOnHit>,
        Option<&ProjectileImpulse>,
    ), (With<Projectile>, Without<ColliderDisabled>)>,
    mut enemy: Query<(&mut Health, &Transform), With<Enemy>>,
    mut player: Query<&Transform, With<Player>>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
) {
    for (
        projectile_entity,
        collider,
        transform,
        damage,
        attack_timer,
        mut hit_enemies,
        should_delete_projectile,
        projectile_impulse,
    ) in &mut attacks {

        if let Some(mut attack_timer) = attack_timer{
            attack_timer.timer.tick(time.delta());
            if !attack_timer.timer.just_finished() {
                // early return, attack not ready
                return ()
            }
        }
        rapier_context.intersections_with_shape(
            transform.translation().truncate(),
            0.0,
            collider,
            QueryFilter::new(),
            |enemy_entity| {
                if let Ok((health, transform)) = enemy.get_mut(enemy_entity) {
                    if let Some(hit_enemies) = hit_enemies.as_deref_mut(){
                        if hit_enemies.seen.contains(&enemy_entity.index()){
                            return true
                        }
                        hit_enemies.seen.push(enemy_entity.index());
                    }

                    damage_enemy(&mut commands, enemy_entity, health, transform, damage.0);

                    // maybe use events?
                    if let Some(projectile_impulse) = projectile_impulse{
                        let player_transform = player.single_mut();
                        let direction:Vec2 = transform.translation.truncate() -player_transform.translation.truncate();
                        commands.entity(enemy_entity).try_insert(ExternalImpulse   {
                            impulse: direction.normalize() * projectile_impulse.0,
                            torque_impulse: 0.0,
                        },);
                    }

                    if let Some(_should_delete) = should_delete_projectile{
                        commands.entity(projectile_entity).despawn_recursive();
                    }
                }
                true
            },
        );

    }
}


pub fn start_reload_attack_spawner(
    mut commands: Commands,
    mut attack_spawners: Query<(Entity, &AttackAmmo),  Without<AttackReloadDuration>>,
){
    for (entity, attack_ammo) in &mut attack_spawners {
        if attack_ammo.amount == 0 {
            commands.entity(entity).insert(AttackReloadDuration {
                timer:Timer::from_seconds(attack_ammo.reload_time, TimerMode::Once)
            },);
        }
    }

}

fn reloading_attack_spawner(
    mut commands: Commands,
    mut attack_spawners: Query<(
        Entity,
        &mut AttackReloadDuration,
        &mut AttackAmmo
    ), With<AttackReloadDuration>>,
    time: Res<Time>,
){
    for (
        entity,
        mut attack_reload,
        mut attack_ammo
    ) in &mut attack_spawners {
        attack_reload.timer.tick(time.delta());

        if attack_reload.timer.just_finished() {
            commands.entity(entity).remove::<AttackReloadDuration>();
        }

        attack_ammo.amount = attack_ammo.size;
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

fn projectile_despawn(
    mut commands: Commands,
    mut projectiles: Query<(Entity, &mut ProjectileLifetime), With<Projectile>>,
) {
    for (entity, attack_duration)  in &mut projectiles {
        if attack_duration.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}


fn projectile_follow_player(
    player: Query<&Transform, (With<Player>, Without<FireArea>)>,
    mut projectile: Query<&mut Transform, (With<ProjectileFollowPlayer>, Without<Player>)>,
) {
    if let Ok(mut fire_area) = projectile.get_single_mut() {
        if let Ok(player) = player.get_single() {
            fire_area.translation.x = player.translation.x;
            fire_area.translation.y = player.translation.y;
        }
    }
}
