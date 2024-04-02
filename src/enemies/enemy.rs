use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::*;
use crate::enemies::bats::BatPlugin;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // bats enemy
        app.add_plugins(BatPlugin);
        // basic enemy logic
        app.add_systems(Update, (
            enemy_death_check,
            enemy_applied_impulse,
            enemy_applied_received_damage,
            compute_enemy_velocity,
            apply_aura_on_enemy_velocity,
            apply_enemy_velocity,
            ).chain().run_if(in_state(GameState::Gameplay))
        );

        app.add_systems(Update, (
            enemy_damage_player,
           ).run_if(in_state(GameState::Gameplay))
        );

    }
}


fn compute_enemy_velocity(
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemies: Query<(&Transform, &mut Sprite, &mut EnemyVelocity, &EnemySpeed),(With<Enemy>,)>,
    time: Res<Time>,
) {
    let player_transform = player.single();
    for (transform, mut sprite, mut velocity, speed) in &mut enemies {
        let direction = (transform.translation.truncate()
            - player_transform.translation.truncate())
            .normalize();
        sprite.flip_x = direction.x < 0.0;

        velocity.x = direction.x * time.delta_seconds() * speed.0;
        velocity.y = direction.y * time.delta_seconds() * speed.0;
    }
}


fn apply_aura_on_enemy_velocity(
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut EnemyVelocity, &mut VelocityAura, &mut Sprite)>,
    time: Res<Time>,
) {
    for (entity, mut velocity, mut aura, mut sprite) in &mut enemies {
        velocity.x *= aura.value;
        velocity.y *= aura.value;
        
        aura.lifetime.tick(time.delta());
        
        sprite.color = Color::Rgba {
            red: 0.0,
            green: 0.0,
            blue: 1.0,
            alpha: 1.0,
        };

        if aura.lifetime.just_finished() {
            sprite.color = Color::WHITE;
            commands.entity(entity).remove::<VelocityAura>();
        }
    }
}


fn apply_enemy_velocity(
    mut enemies: Query<(&mut Transform, &EnemyVelocity)>,
) {
    for (mut transform, velocity) in &mut enemies {
        transform.translation -= velocity.extend(0.0);
    }
}



fn enemy_damage_player(
    enemies: Query<(&CollidingEntities, &EnemyDamageOverTime),With<Enemy>>,
    player: Query<Entity, With<Player>>,
    time: Res<Time>,
    mut player_received_damage_event: EventWriter<PlayerReceivedDamage>,
) {
    let player= player.single();
    for (colliding_entities, damage) in enemies.iter() {
        if colliding_entities.contains(player) {
            player_received_damage_event.send(
            PlayerReceivedDamage{
                damage: damage.0 * time.delta_seconds()
            }
        );
        }
    }
    
}

pub fn enemy_applied_impulse(
    mut commands: Commands,
    mut enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut player: Query<&Transform, With<Player>>,
    mut eneny_hit_event: EventReader<EnemyHitByProjectile>,
) {
    let player_transform = player.single_mut();
    for event in eneny_hit_event.read() {
        if let Some(impulse) = event.impulse{
            if let Ok((enemy_entity, enemy_transform)) = enemies.get_mut(event.enemy_entity){
                 let direction:Vec2 = enemy_transform.translation.truncate() -player_transform.translation.truncate();
                 commands.entity(enemy_entity).try_insert(ExternalImpulse   {
                    impulse: direction.normalize() * impulse,
                    torque_impulse: 0.0,
                 },);
            }
        }
    }
}

pub fn enemy_applied_received_damage(
    mut enemies: Query<&mut Health, With<Enemy>>,
    mut eneny_received_damaged_event: EventReader<EnemyReceivedDamage>,
) {
    for event in eneny_received_damaged_event.read() {
        if let Ok(mut health) = enemies.get_mut(event.enemy_entity){
            **health -= event.damage;
        }
    }
}

pub fn enemy_death_check(
    mut commands: Commands,
    mut enemies: Query<(Entity, &Transform, &Health, &EnemyExperienceDrop), With<Enemy>>,
    mut enemy_died: EventWriter<EnemyDied>,
) {
    for (entity, transform, health, experience) in &mut enemies {
        if health.0 <= 0.0 {
            enemy_died.send(
                EnemyDied{
                    position: transform.translation.clone(),
                    experience: experience.0 }
            );
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
