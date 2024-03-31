use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::*;
use crate::constants::MAP_LEVEL_EXPERIENCE;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player_plugin);
        app.add_systems(Update, (
            player_movement,
            player_game_over,
            compute_experience,
            gem_hit_player_pickup_radius,
            ).run_if(in_state(GameState::Gameplay))
        );

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

    let player = (
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture,
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
        AnimationIndices { first: 0, last: 3, is_repeating: true },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Collider::ball(4.0),
        Health(100.0),
        MaxHealth(100.0),
        Player{
            facing: Facing::Right,
        },
        Name::new("Player")
    );

    let player_pickup_collider = (
        Collider::ball(50.0),
        TransformBundle {..default()},
        Sensor,
        PlayerPickupRadius,
        Name::new("Player pickup collider")
    );

    commands.spawn(player).with_children(|commands|{
        commands.spawn(player_pickup_collider);
    });



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
    mut _game_state: ResMut<NextState<GameState>>,
    // audio: Res<Audio>,
    // assets: Res<AssetServer>,
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
    }

}



fn gem_hit_player_pickup_radius(
    mut commands: Commands,
    mut gems: Query<Entity, (Without<ColliderDisabled>, Without<GemIsAttracted>)>,
    player: Query<&Children, With<Player>>,
    player_pickup: Query<( &GlobalTransform, &Collider), With<PlayerPickupRadius>>,
    rapier_context: Res<RapierContext>,
) {
    if let Ok(player_children) = player.get_single() {
         for &child in player_children.iter() {
             if let Ok((pickup_transform,pickup_collider)) = player_pickup.get(child) {
                 rapier_context.intersections_with_shape(
                    pickup_transform.translation().truncate(),
                    0.0,
                    pickup_collider,
                    QueryFilter::new(),
                    |entity| {
                        if let Ok(gem_entity) = gems.get_mut(entity) {
                            commands.entity(gem_entity).try_insert(GemIsAttracted);
                        }
                        true
                    },
                );
             }
         }
    }

}
