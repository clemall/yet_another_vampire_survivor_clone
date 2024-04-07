use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::math_utils::{get_random_position_in_screen, get_random_position_outside_screen};

pub struct BatPlugin;

impl Plugin for BatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_bats);
    }
}



fn spawn_bats(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
){
    if keyboard_input.just_pressed(KeyCode::KeyO) || keyboard_input.pressed(KeyCode::KeyP) {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("enemies.png"),
                // transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                transform: Transform{
                    translation: get_random_position_outside_screen().extend(0.0),
                    rotation: Default::default(),
                    scale: Vec3::new(0.5,0.5, 0.0),
                },

                ..default()
            },
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED_Z,
            Damping {
                linear_damping: 100.0,
                angular_damping: 1.0,
            },
            Collider::ball(8.0),
            CollisionGroups::new(ENEMY_GROUP,PLAYER_GROUP | ENEMY_GROUP | PROJECTILE_GROUP),
            ActiveEvents::COLLISION_EVENTS,
            CollidingEntities::default(),
            ActiveCollisionTypes::default(),
        )).insert((
            ColliderMassProperties::Density(2.0),
            Enemy,
            Health(50.0),
            EnemySpeed(30.0),
            EnemyVelocity(Vec2::new(0.0, 0.0)),
            EnemyDamageOverTime(10.0),
            EnemyExperienceDrop(1),
            // VelocityAura{
            //     value: 0.5,
            //     lifetime: Timer::from_seconds(2.0, TimerMode::Once),
            // },
        ));
    }

}
