use bevy::prelude::*;

use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use bevy_rapier2d::prelude::*;
use rand::Rng;
use crate::components::*;

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
        let mut rng = rand::thread_rng();
        let x: i32 = rng.gen_range(-SCREEN_WIDTH/2..SCREEN_WIDTH/2);
        let y: i32 = rng.gen_range(-SCREEN_HEIGHT/2..SCREEN_HEIGHT/2);

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("enemies.png"),
                // transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                transform: Transform{
                    translation: Vec3::new(x as f32, y as f32, 0.0),
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
            ColliderMassProperties::Density(2.0),
            Enemy,
            Health(50.0),
            MaxHealth(500.0),
            EnemyVelocity(Vec2::new(0.0, 0.0)),
            EnemySpeed(30.0),
            EnemyDamageOverTime(10.0),
        ));
    }

}
