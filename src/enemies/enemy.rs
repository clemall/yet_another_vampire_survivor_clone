use bevy::prelude::*;
use bevy_rapier2d::dynamics::{Damping, LockedAxes, RigidBody};
use bevy_rapier2d::pipeline::QueryFilter;
use bevy_rapier2d::plugin::RapierContext;
use bevy_rapier2d::prelude::{Collider, ColliderMassProperties, ExternalForce, ExternalImpulse};
use rand::Rng;
use crate::components::*;
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::enemies::bats::BatPlugin;
use crate::ui::ui_enemy::spawn_world_text;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // bats enemy
        app.add_plugins(BatPlugin);
        // basic enemy logic
        app.add_systems(Update, (
            compute_enemy_velocity,
            apply_enemy_velocity,
            enemy_damage_player
            ).chain()
        );
        app.add_systems(Update, enemy_death_check);

    }
}


fn compute_enemy_velocity(
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemies: Query<(&mut Transform, &mut Sprite, &mut EnemyVelocity, &mut EnemySpeed),(With<Enemy>,)>,
    time: Res<Time>,
) {
    let player_transform = player.single();
    for (mut transform, mut sprite, mut velocity,  mut speed) in &mut enemies {
        let mut direction = (transform.translation.truncate()
            - player_transform.translation.truncate())
            .normalize();
        sprite.flip_x = direction.x < 0.0;

        velocity.x = direction.x * time.delta_seconds() * speed.0;
        velocity.y = direction.y * time.delta_seconds() * speed.0;
    }
}




fn apply_enemy_velocity(
    mut enemies: Query<(&mut Transform, &mut EnemyVelocity)>,
) {
    for (mut transform, mut velocity) in &mut enemies {
        transform.translation -= velocity.extend(0.0);
    }
}



fn enemy_damage_player(
    enemies: Query<(&Collider, &GlobalTransform, &EnemyDamageOverTime),(With<Enemy>,)>,
    mut health: Query<(&mut Health), With<Player>>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
) {
    for (collider, transform, damage) in &enemies {
        rapier_context.intersections_with_shape(
            transform.translation().truncate(),
            0.0,
            collider,
            QueryFilter::new(),
            |entity| {
                if let Ok(mut health) = health.get_mut(entity) {
                    **health -= damage.0 * time.delta_seconds();
                }
                true
            },
        );
    }
}


pub fn damage_enemy(
    commands: &mut Commands,
    entity: Entity,
    mut health: Mut<Health>,
    position: &Transform,
    damage: f32,
) {
    // TODO: Use event ?
    spawn_world_text(
        commands,
        position.translation.truncate(),
        &format!("{:?}", damage as i32),
    );

    **health -= damage;

    // if health.0 <= 0.0 {
    //     commands.entity(entity).despawn_recursive();
    // }
}

pub fn enemy_death_check(
    mut commands: Commands,
    mut enemies: Query<(Entity, &Transform, &Health), With<Enemy>>,
) {
    for (entity, transform, health) in &mut enemies {
        if health.0 <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

//
// fn enemy_movement(
//     player: Query<&Transform, (With<Player>, Without<Enemy>)>,
//     mut enemies: Query<(&mut Transform, &mut Sprite, &mut Enemy)>,
//     mut enemies_neighbour: Query<(&mut Transform, &mut Sprite, &mut Enemy)>,
//     time: Res<Time>,
// ) {
//     let player_transform = player.single();
//
//
//     for (mut transform, mut sprite, mut enemy) in &mut enemies {
//         let mut direction = (transform.translation.truncate()
//             - player_transform.translation.truncate())
//         .normalize();
//         sprite.flip_x = direction.x < 0.0;
//
//         enemy.velocity.x = (direction.x * time.delta_seconds() * enemy.speed);
//         enemy.velocity.y = (direction.y * time.delta_seconds() * enemy.speed);
//
//         for (mut neighbour_transform, mut sprite, mut neighbour_enemy) in &mut enemies_neighbour {
//             if transform.translation.distance(neighbour_transform.translation) < 20.0 {
//                 let avoid_x = transform.translation.x - neighbour_transform.translation.x;
//                 let avoid_y = transform.translation.y - neighbour_transform.translation.y;
//
//                 enemy.velocity.x += avoid_x;
//                 enemy.velocity.y += avoid_y;
//
//                 let mut _dist = enemy.velocity.x * enemy.velocity.x + enemy.velocity.y * enemy.velocity.y;
//                 if _dist != 0.0
//                 {
//                     _dist = _dist.sqrt();
//                     enemy.velocity.x /= _dist * enemy.speed;
//                     enemy.velocity.y /= _dist * enemy.speed;
//                 }
//             }
//             // transform.translation -= enemy.velocity.extend(0.0);
//         }
//         transform.translation -= enemy.velocity.extend(0.0);
//     }
// }


//
// fn check_enemy_neighbour(
//     mut enemies: Query<(&mut Transform, &mut Enemy, &mut Velocity)>,
//     player: Query<&Transform, (With<Player>, Without<Enemy>)>,
//     time: Res<Time>,
// ){
//     let player_transform = player.single();
//     let mut combos = enemies.iter_combinations_mut();
//     while let Some([(mut trans1, mut enemy1, mut velocity1), (mut trans2, mut enemy2, mut velocity12)]) = combos.fetch_next() {
//         if trans1.translation.distance(trans2.translation) < 18.0 {
//             let avoid_x = player_transform.translation.x - trans2.translation.x;
//             let avoid_y = player_transform.translation.y - trans2.translation.y;
//
//             velocity1.x += avoid_x;
//             velocity1.y += avoid_y;
//
//             let mut _dist =velocity1.x * velocity1.x + velocity1.y *velocity1.y;
//             if _dist != 0.0
//             {
//                 _dist = _dist.sqrt();
//                 velocity1.x /= _dist * enemy1.speed ;
//                 velocity1.y /= _dist * enemy1.speed ;
//             }
//         }
//     }
// }
