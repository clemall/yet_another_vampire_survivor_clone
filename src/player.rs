use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;
use crate::components::*;
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::weapons::claw::{setup_claw_spawner};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_player_plugin,setup_player_ui));
        app.add_systems(Update, (
            player_movement,
            player_game_over,
            player_health_ui_sync
            )
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
    let animation_indices = AnimationIndices { first: 0, last: 3, is_repeating: true };


    // let claw = spawn_claw(&mut commands, &asset_server);
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
        .insert(Player{
            health: 100.0,
            max_health: 100.0,
            facing: Facing::Right,
        });
        // .add_child(claw);

    // default weapon
    setup_claw_spawner(&mut commands);
}

fn setup_player_ui(mut commands: Commands,
         asset_server: Res<AssetServer>,
) {

    let parent_node = (
        NodeBundle {
            style: Style {
                width: Val::Px(80.),
                height: Val::Px(5.),
                // WTF, should be SCREEN_WIDTH / 2... but the screen UI seems to be 1200px,
                left: Val::Px(SCREEN_WIDTH as f32 - 40.0),
                right: Val::Auto,
                top: Val::Px(SCREEN_HEIGHT as f32 + 20.0),
                bottom: Val::Auto,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        },
        PlayerUI,
        Name::new("Player UI"),
    );

    let health_node = (
        NodeBundle {
            style: Style {
                width: Val::Px(80.),
                height: Val::Px(5.),
                ..default()
            },
            background_color: BackgroundColor(Color::RED),
            ..default()
        },
        HealthUI,
        Name::new("Health UI"),
    );

    commands.spawn(parent_node).with_children(|commands| {
        commands.spawn(health_node);
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
    player: Query<&Player>,
    // mut game_state: ResMut<NextState<GameState>>,
    // audio: Res<Audio>,
    assets: Res<AssetServer>,
) {
    let player = player.single();

    if player.health <= 0.0 {
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


//
fn player_health_ui_sync(mut ui: Query<&mut Style, With<HealthUI>>, player: Query<&Player>) {
    let mut style = ui.single_mut();
    let player = player.single();

    let percent = player.health / player.max_health;
    style.width = Val::Percent(percent * 100.0);
}

