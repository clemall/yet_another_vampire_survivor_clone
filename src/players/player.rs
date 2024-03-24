use bevy::prelude::*;
use bevy_rapier2d::dynamics::{LockedAxes, RigidBody};
use bevy_rapier2d::geometry::Sensor;
use bevy_rapier2d::prelude::Collider;
use crate::components::*;
use crate::constants::MAP_LEVEL_EXPERIENCE;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_player_plugin));
        app.add_systems(Update, (
            player_movement,
            player_game_over,
            // compute_experience_from_collect,
            ).run_if(in_state(GameState::Gameplay))
        );
        app.add_systems(Update,compute_experience.run_if(in_state(GameState::Gameplay)));

    }
}

const AVATAR_SPEED: f32 = 60.0;


fn setup_player_plugin(mut commands: Commands,
         asset_server: Res<AssetServer>,
         mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {

    let texture = asset_server.load("player.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 4, 1, Option::from(Vec2::new(1.0, 1.0)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 3, is_repeating: true };


    commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture,
            ..default()
        })
        .insert(TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        })
        .insert(animation_indices)
        .insert(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .insert(Collider::ball(4.0),)
        .insert(Health(100.0))
        .insert(MaxHealth(100.0))
        .insert(Player{
            facing: Facing::Right,
        })
        .insert(Name::new("Health UI"));



}





// public because of the camera, see camera.rs
pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Sprite, &mut Player), With<Player>>,
    time: Res<Time>,
) {
    let (mut avatar_transform, mut avatar_sprite, mut player) = query.single_mut();
    let mut direction = Vec2::new(0., 0.);

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
     if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y += -1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x += -1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction.x != 0.0 {
        avatar_sprite.flip_x = direction.x < 0.;
        match avatar_sprite.flip_x {
            true => {
                player.facing = Facing::Left
            }
            false => {
                player.facing = Facing::Right
            }
        }
    }


    direction = direction.normalize_or_zero();

    avatar_transform.translation.x += direction.x * AVATAR_SPEED * time.delta_seconds();
    avatar_transform.translation.y += direction.y * AVATAR_SPEED * time.delta_seconds();
}


fn player_game_over(
    health: Query<&Health, With<Player>>,
    // mut game_state: ResMut<NextState<GameState>>,
    // audio: Res<Audio>,
    assets: Res<AssetServer>,
) {
    let health = health.single();

    if health.0 <= 0.0 {
        // audio.play_with_settings(
        //     assets.load("death.wav"),
        //     PlaybackSettings {
        //         repeat: false,
        //         volume: 0.9,
        //         speed: 1.0,
        //     },
        // );
        // game_state.set(GameState::GameOver);
    }
}



fn compute_experience(
    mut commands: Commands,
    mut collect_experience: EventReader<CollectExperience>,
    mut player_experience: ResMut<PlayerExperience>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in collect_experience.read() {
        player_experience.amount_experience += event.experience;
    }

    let amount_of_experience_before_leveling = MAP_LEVEL_EXPERIENCE[player_experience.level as usize];

    if player_experience.amount_experience >= amount_of_experience_before_leveling{
        // 0 or the remaining experience
        player_experience.amount_experience -= amount_of_experience_before_leveling;

        player_experience.level += 1;

        // GG player leveled up
        next_state.set(GameState::PlayerLevelUp);
        println!("Should be in player level up STATE");
    }

}

