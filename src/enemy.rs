use bevy::prelude::*;
use bevy_rapier2d::dynamics::{Damping, LockedAxes, RigidBody};
use bevy_rapier2d::pipeline::QueryFilter;
use bevy_rapier2d::plugin::RapierContext;
use bevy_rapier2d::prelude::{Collider, ColliderMassProperties, ExternalForce, ExternalImpulse};
use rand::Rng;
use crate::components::*;
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, (compute_enemy_velocity, check_enemy_neighbour, apply_enemy_velocity).chain())
        app.add_systems(Update, (
            enemy_death_check,
            compute_enemy_velocity,
            apply_enemy_velocity,
            enemy_damage_player
            ).chain()
        );
        app.add_systems(Update, update_world_text);
        app.add_systems(Update, debug_spawn_enemies);
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


fn debug_spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
){
    if keyboard_input.just_pressed(KeyCode::KeyH) || keyboard_input.pressed(KeyCode::KeyJ) {
        let mut rng = rand::thread_rng();
        let x: i32 = rng.gen_range(-SCREEN_WIDTH/2..SCREEN_WIDTH/2);
        let y: i32 = rng.gen_range(-SCREEN_HEIGHT/2..SCREEN_HEIGHT/2);

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("enemies.png"),
                transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                ..default()
            },
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED_Z,
            Damping {
                linear_damping: 100.0,
                angular_damping: 1.0,
            },
            Collider::ball(8.0),
            ColliderMassProperties::Density(1.0),
            Enemy{
                health: 30.0
            },
            EnemyVelocity(Vec2::new(0.0, 0.0)),
            EnemySpeed(30.0),
            EnemyDamageOverTime(10.0),
        ));
    }

}


fn enemy_damage_player(
    enemies: Query<(&Collider, &GlobalTransform, &EnemyDamageOverTime),(With<Enemy>,)>,
    mut player: Query<&mut Player>,
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
                if let Ok(mut player) = player.get_mut(entity) {
                    player.health -= damage.0 * time.delta_seconds();
                }
                true
            },
        );
    }
}


pub fn damage_enemy(
    commands: &mut Commands,
    enemy: &mut Enemy,
    position: &Transform,
    damage: f32,
) {
    spawn_world_text(
        commands,
        position.translation.truncate(),
        &format!("{:?}", damage as i32),
    );
    println!("{}", damage);

    enemy.health -= damage;
}

fn enemy_death_check(
    mut commands: Commands,
    mut enemies: Query<(Entity, &Transform, &Enemy)>,
) {
    for (entity, transform, enemy) in &mut enemies {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_world_text(commands: &mut Commands,  position: Vec2, text: &str) {
    let position = position + Vec2::new(-0.2, 1.4);

    let parent = (
        NodeBundle {
            style: Style {
                width: Val::Px(50.),
                height: Val::Px(50.),
                position_type: PositionType::Absolute,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            z_index: ZIndex::Global(-100),
            ..default()
        },
        WorldTextUI {
            lifetime: Timer::from_seconds(0.5, TimerMode::Once),
            velocity: Vec2::new(0.15, 1.5),
            position,
        },
    );

    let text = TextBundle::from_section(
        text,
        TextStyle {
            font: Default::default(),
            font_size: 32.0,
            color: Color::rgb(0.95, 0.2, 0.2),
        },
    );

    commands.spawn(parent).with_children(|commands| {
        commands.spawn(text);
    });
}

fn update_world_text(
    mut commands: Commands,
    mut text: Query<(Entity, &mut Style, &mut WorldTextUI)>,
    main_camera: Query<(&Camera, &GlobalTransform)>,
    // render_camera: Query<&Camera>,
    time: Res<Time>,
) {
    let (camera, transform) = main_camera.single();
    // let final_camera = render_camera.single();

    for (entity, mut style, mut world_ui) in &mut text {
        world_ui.lifetime.tick(time.delta());
        if world_ui.lifetime.just_finished() {
            commands.entity(entity).despawn_recursive();
        }

        world_ui.position = world_ui.position + world_ui.velocity * time.delta_seconds();

        if let Some(mut coords) = camera.world_to_viewport(transform, world_ui.position.extend(0.0)) {
            // let mut coords = coords / Vec2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)
            //     * camera.logical_viewport_size().unwrap();
            coords.y = camera.logical_viewport_size().unwrap().y - coords.y;
            style.left = Val::Px(coords.x);
            style.top = Val::Px(coords.y);

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
