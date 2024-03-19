use std::time::Duration;
use bevy::prelude::*;
use bevy_pixel_camera::{PixelViewport, PixelZoom};
use bevy_rapier2d::prelude::*;
use crate::components::{AlreadyHitEnemies, AnimationIndices, AnimationTimer, AttackDuration, AttackTimer, Claw, ClawSpawner, Enemy, Facing, Player};

use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::enemy::damage_enemy;
use crate::player::player_movement;

const CLAWS_POSITION_X:f32 = 28.0;
pub struct WeaponClawPlugin;

impl Plugin for WeaponClawPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, spawn_claw);
        // app.add_systems(Update,(claw_attack_facing, claw_attack, claw_damage));
        app.add_systems(Update,(
            spawn_claw_attack,
            claw_attack_duration_tick,
            claw_attack_animation_and_collider,
            claw_damage,
            claw_attack_despawn
        ));
    }
}

pub fn setup_claw_spawner(commands: &mut Commands)-> Entity{
    let mut timer = Timer::from_seconds(2.0, TimerMode::Repeating);
    timer.set_elapsed(Duration::from_secs(1));

    commands.spawn((
        ClawSpawner,
        AttackTimer{
            timer:timer,
        },
        Name::new("Claw Spawner"),
    )).id()
}

fn spawn_claw_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut claw_spawner: Query<(&mut ClawSpawner, &mut AttackTimer,)>,
    mut player: Query<(&Transform, &mut Player)>,
    time: Res<Time>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
){
    let (mut spawner, mut attack_timer) = claw_spawner.single_mut();
    let (player_transform, player) = player.single_mut();
    attack_timer.timer.tick(time.delta());


    if attack_timer.timer.just_finished() {
        factory_claw(&mut commands, &asset_server, &mut texture_atlas_layouts, &player_transform, Facing::Left);
        factory_claw(&mut commands, &asset_server, &mut texture_atlas_layouts, &player_transform, Facing::Right);
    }
}

fn factory_claw(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    player_transform:&Transform,
    facing: Facing,
) {
    let texture = asset_server.load("claw.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(48.0, 48.0), 2, 1, Option::from(Vec2::new(1.0, 1.0)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let mut pos_x = player_transform.translation.x;
    let mut is_flip = false;
    match facing {
        Facing::Left => {
            pos_x -= CLAWS_POSITION_X;
            is_flip = true;
        }
        Facing::Right => {
            pos_x += CLAWS_POSITION_X;
            is_flip = false;
        }
    }

    let mut timer_attack_duration = Timer::from_seconds(0.3, TimerMode::Once);

    commands.spawn((
        SpriteBundle {
            texture,
            // transform: Transform::from_xyz(pos_x, player_transform.translation.y, 1.0),
            transform: Transform{
                translation: Vec3::new(pos_x, player_transform.translation.y, 1.0),
                // scale: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                flip_x: is_flip,
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
        AnimationIndices { first: 0, last: 1, is_repeating: false },
        AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
        Sensor,
        Collider::cuboid(48.0/2.0, 48.0/2.0),
        AttackDuration{
            timer:timer_attack_duration,
        },
        Claw{
            damage: 10.0,
        },
        AlreadyHitEnemies{seen:Vec::new()},
        Name::new("Claw Attack"),
    ));
}

fn claw_attack_duration_tick(
    mut claws: Query<&mut AttackDuration, With<Claw>>,
    time: Res<Time>,
) {
     for ( mut attack_duration) in &mut claws {
         attack_duration.timer.tick(time.delta());
     }
}

fn claw_attack_animation_and_collider(
    mut claws: Query<(&mut Transform, &mut Sprite, &mut Collider, &mut AttackDuration), With<Claw>>,
    time: Res<Time>,
) {
    for (mut tranform, mut sprite,mut collider, mut attack_duration)  in &mut claws {
        // transform claw attack
        // tranform.scale.x = (attack_duration.timer.fraction() * 1.0) + 0.7;
        // tranform.scale.y = (attack_duration.timer.fraction() * 1.0) + 0.7;
        // collider claw attack
        // let new_scale = Vec2::new(
        //     (attack_duration.timer.fraction() * 1.1),
        //     (attack_duration.timer.fraction() * 1.2)
        // );
        // collider.set_scale(new_scale, 1);
    }
}

fn claw_attack_despawn(
    mut commands: Commands,
    mut claws: Query<(Entity, &mut AttackDuration), With<Claw>>,
    time: Res<Time>,
) {
    for (entity, mut attack_duration)  in &mut claws {
        if attack_duration.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
//
// pub fn spawn_claw(
//     commands: &mut Commands,
//     asset_server: &Res<AssetServer>,
// )  -> Entity{
//     let mut timer = Timer::from_seconds(2.0, TimerMode::Repeating);
//     timer.set_elapsed(Duration::from_secs(1));
//
//     commands.spawn((
//         SpriteBundle {
//             texture: asset_server.load("claw.png"),
//             transform: Transform::from_xyz(CLAWS_POSITION_X, 0.0, 1.0),
//             ..default()
//         },
//         Sensor,
//         Collider::cuboid(16.0/2.0, 48.0/2.0),
//         AttackTimer{
//             timer:timer,
//         },
//         Claw{
//             damage: 100.0,
//         },
//         Name::new("Claw Attack"),
//     )).id()
//
// }

// pub fn claw_attack_facing(
//     mut claws: Query<(&mut Transform, &mut Sprite, &Visibility), With<Claw>>,
//     player: Query<&Player>,
// ) {
//     let player = player.single();
//
//     if let Ok((mut whip, mut sprite, visibility)) = claws.get_single_mut() {
//         // Change the orientation of the claw only when we are not attacking.
//         if visibility == Visibility::Hidden {
//             whip.translation = match player.facing {
//                 Facing::Left => {
//                     sprite.flip_x = true;
//                     Vec3::new(-CLAWS_POSITION_X, 0.0, 0.0)
//                 }
//                 Facing::Right => {
//                     sprite.flip_x = false;
//                     Vec3::new(CLAWS_POSITION_X, 0.0, 0.0)
//                 }
//             };
//         }
//     }
// }

//
// fn claw_attack(
//     mut commands: Commands,
//     mut claws: Query<(
//         Entity,
//         &Collider,
//         &GlobalTransform,
//         &mut Claw,
//         &mut AttackTimer,
//         &mut Visibility,
//     )>,
//     mut enemy: Query<(&mut Enemy, &Transform)>,
//     rapier_context: Res<RapierContext>,
//     time: Res<Time>,
// ) {
//     for (entity,collider, transform, mut claw,mut attack_timer, mut visibility) in &mut claws {
//         attack_timer.timer.tick(time.delta());
//
//         if attack_timer.timer.fraction() < 0.2 || attack_timer.timer.fraction() > 0.9 {
//             *visibility = Visibility::Visible;
//             commands.entity(entity).remove::<ColliderDisabled>();
//         } else {
//             *visibility = Visibility::Hidden;
//             commands.entity(entity).insert(ColliderDisabled);
//         };
//
//
//         // if attack_timer.timer.just_finished() {
//         //     rapier_context.intersections_with_shape(
//         //         transform.translation().truncate(),
//         //         0.0,
//         //         collider,
//         //         QueryFilter::new(),
//         //         |entity| {
//         //             if let Ok((mut enemy, transform)) = enemy.get_mut(entity) {
//         //                 damage_enemy(&mut commands,  &mut enemy, transform, claw.damage);
//         //             }
//         //             true
//         //         },
//         //     );
//         // }
//     }
// }


fn claw_damage(
    mut commands: Commands,
    mut claws: Query<(
        &Collider,
        &GlobalTransform,
        &mut Claw,
        &mut AlreadyHitEnemies,
    ), Without<ColliderDisabled>>,
    mut enemy: Query<(&mut Enemy, &Transform)>,
    mut player: Query<(&Transform, &mut Player)>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
) {
    for (collider, transform, mut claw, mut seen_enemies) in &mut claws {
        // let filter = QueryFilter::new()
        //     .predicate(&|handle: Entity| seen_enemies.seen.contains(&handle.index()));

        rapier_context.intersections_with_shape(
            transform.translation().truncate(),
            0.0,
            collider,
            QueryFilter::new(),
            |entity| {
                if let Ok((mut enemy, transform)) = enemy.get_mut(entity) {
                    if !seen_enemies.seen.contains(&entity.index()){
                        damage_enemy(&mut commands,  &mut enemy, transform, claw.damage);

                        seen_enemies.seen.push(entity.index());

                        let (player_transform, player) = player.single_mut();
                        let mut direction:Vec2 = (transform.translation.truncate() -player_transform.translation.truncate());
                        commands.entity(entity).insert(ExternalImpulse   {
                            impulse: direction.normalize() * 2000.0,
                            torque_impulse: 0.0,
                        },);
                    }
                }
                true
            },
        );
    }
}

