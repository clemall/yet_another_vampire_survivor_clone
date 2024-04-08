use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::*;
use crate::enemies::enemies_bundle::EnemyBundle;
use crate::math_utils::{get_random_position_outside_screen};

pub struct BeePlugin;

impl Plugin for BeePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_bee);
    }
}



fn spawn_bee(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
){
    if keyboard_input.just_pressed(KeyCode::KeyH) || keyboard_input.pressed(KeyCode::KeyJ) {
        let texture = asset_server.load("Bee_Walk.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(64.0, 64.0), 4, 4, Option::from(Vec2::new(0.0, 0.0)), None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

         commands.spawn(EnemyBundle {
             sprite_bundle: SpriteBundle{ 
                 texture: texture.clone(),
                 transform: Transform{
                     translation: get_random_position_outside_screen().extend(0.0),
                     rotation: Default::default(),
                     scale: Vec3::new(1.0,1.0, 0.0),
                 },
                  ..default()
             },
             texture_atlas: TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 0,
             },
             animation_indices: AnimationIndices { first: 8, last: 11, is_repeating: true },
             ..default()
         }).with_children(|children| {
            children.spawn((
                Collider::capsule_x(2.0,8.0/2.0),
                TransformBundle::from(Transform::from_xyz(0.0, 5.0, 0.0)),
            ));
         });
    }
}
