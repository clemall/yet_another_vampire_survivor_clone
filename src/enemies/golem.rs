use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::*;
use crate::enemies::enemies_bundle::EnemyBundle;
use crate::math_utils::{get_random_position_outside_screen};

pub struct GolemPlugin;

impl Plugin for GolemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_golem);
    }
}



fn spawn_golem(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
){
    if keyboard_input.just_pressed(KeyCode::KeyN) || keyboard_input.pressed(KeyCode::KeyM) {
        let texture = asset_server.load("Golem_Run.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 4, 1, Option::from(Vec2::new(0.0, 0.0)), None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

         commands.spawn(EnemyBundle {
             sprite_bundle: SpriteBundle{ 
                 texture: texture.clone(),
                 transform: Transform{
                     translation: get_random_position_outside_screen().extend(0.0),
                     rotation: Default::default(),
                     scale: Vec3::new(1.4,1.4, 0.0),
                 },
                  ..default()
             },
             texture_atlas: TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 0,
             },
             animation_indices: AnimationIndices { first: 0, last: 3, is_repeating: true },
             enemy_speed: EnemySpeed(18.0),
             collider: Collider::capsule_y(3.5,18.0/2.0),
             ..default()
         });
         // }).with_children(|children| {
         //    children.spawn((
         //        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
         //    ));
         // });
    }
}
