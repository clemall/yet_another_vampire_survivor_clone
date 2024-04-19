use crate::components::*;
use crate::constants::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // TODO: META value, load from file, maybe ron file or bin?
        app.insert_resource(PlayerMetaStats {
            data: PlayerStats {
                mul_max_health: 0.1,
                mul_move_speed: 0.1,
                add_recovery: 0.0,
                mul_power: 0.0,
                mul_area: 0.0,
                mul_magnet: 1.0,
            },
        });

        // TODO: add more characters loaded from ron file
        app.insert_resource(CharacterStats {
            data: PlayerStats {
                mul_max_health: 0.1,
                mul_move_speed: 0.1,
                add_recovery: 0.0,
                mul_power: 0.0,
                mul_area: 0.0,
                mul_magnet: 0.0,
            },
        });

        // Default value for all character before multiplication
        app.insert_resource(PlayerInGameStats { ..default() });

        app.insert_resource(PlayerExperience {
            level: 1,
            amount_experience: 0,
        });
        app.insert_resource(PlayerWeapons {
            weapons: Vec::new(),
        });

        app.add_systems(
            Startup,
            (setup_player_in_game_stats, setup_player_plugin).chain(),
        );

        app.add_systems(
            Update,
            update_player_stats.run_if(resource_exists_and_changed::<PlayerInGameStats>),
        );

        app.add_systems(
            Update,
            (
                player_movement,
                player_received_damage,
                player_game_over,
                compute_experience,
                player_health_recovery,
            )
                .run_if(in_state(GameState::Gameplay)),
        );
    }
}

fn setup_player_plugin(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    player_stats: Res<PlayerInGameStats>,
) {
    let texture = asset_server.load("player.png");
    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(16.0, 16.0),
        4,
        1,
        Option::from(Vec2::new(1.0, 1.0)),
        None,
    );
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
        AnimationIndices {
            first: 0,
            last: 3,
            is_repeating: true,
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        // RigidBody::Dynamic,
        // LockedAxes::ROTATION_LOCKED_Z,
        // Damping {
        //     linear_damping: 100.0,
        //     angular_damping: 1.0,
        // },
        Collider::ball(4.0),
        CollisionGroups::new(PLAYER_GROUP, ENEMY_GROUP | GEM_GROUP),
        ActiveEvents::COLLISION_EVENTS,
        ActiveCollisionTypes::default() | ActiveCollisionTypes::STATIC_STATIC,
        Health(player_stats.max_health),
        MaxHealth(player_stats.max_health),
        HealthRecovery(player_stats.recovery),
        Player {
            facing: Facing::Right,
        },
        Name::new("Player"),
    );

    let player_pickup_collider = (
        TransformBundle { ..default() },
        Sensor,
        Collider::ball(player_stats.magnet),
        CollisionGroups::new(PLAYER_GROUP, GEM_GROUP),
        ActiveEvents::COLLISION_EVENTS,
        ActiveCollisionTypes::STATIC_STATIC,
        PlayerPickupRadius,
        Name::new("Player pickup collider"),
    );

    commands.spawn(player).with_children(|commands| {
        commands.spawn(player_pickup_collider);
    });
}

fn setup_player_in_game_stats(
    meta_stats: Res<PlayerMetaStats>,
    character_stats: Res<CharacterStats>,
    mut player_stats: ResMut<PlayerInGameStats>,
) {
    player_stats.max_health += (BASE_MAX_HEALTH * meta_stats.data.mul_max_health)
        + (BASE_MAX_HEALTH * character_stats.data.mul_max_health);

    player_stats.move_speed += (BASE_MOVE_SPEED * meta_stats.data.mul_move_speed)
        + (BASE_MOVE_SPEED * character_stats.data.mul_move_speed);

    player_stats.magnet += (BASE_MAGNET * meta_stats.data.mul_magnet)
        + (BASE_MAGNET * character_stats.data.mul_magnet);

    player_stats.area +=
        (BASE_AREA * meta_stats.data.mul_area) + (BASE_AREA * character_stats.data.mul_area);

    // Additive
    player_stats.recovery += meta_stats.data.add_recovery + character_stats.data.add_recovery;

}

fn update_player_stats(
    mut commands: Commands,
    player_stats: Res<PlayerInGameStats>,
    mut player: Query<(&mut MaxHealth, &mut HealthRecovery), With<Player>>,
    pickup_radius_entity: Query<Entity, With<PlayerPickupRadius>>,
) {
    let (mut player_max_health, mut player_recovery) = player.single_mut();
    player_max_health.0 = player_stats.max_health;
    player_recovery.0 = player_stats.recovery;

    let pickup_radius_entity = pickup_radius_entity.single();
    commands
        .entity(pickup_radius_entity)
        .insert(Collider::ball(player_stats.magnet));

    // Debug stuff
    println!("Debug player stats:");
    println!("max_health: {}", player_stats.max_health);
    println!("recovery: {}", player_stats.recovery);
    println!("move_speed: {}", player_stats.move_speed);
    println!("magnet: {}", player_stats.magnet);
    println!("power: {}", player_stats.power);
    println!("area: {}", player_stats.area);
}

// public because of the camera, see camera.rs
pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Sprite, &mut Player), With<Player>>,
    player_stats: Res<PlayerInGameStats>,
    time: Res<Time>,
) {
    println!("je passe");
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
            true => player.facing = Facing::Left,
            false => player.facing = Facing::Right,
        }
    }

    direction = direction.normalize_or_zero();

    avatar_transform.translation.x += direction.x * player_stats.move_speed * time.delta_seconds();
    avatar_transform.translation.y += direction.y * player_stats.move_speed * time.delta_seconds();
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

    let amount_of_experience_before_leveling =
        MAP_LEVEL_EXPERIENCE[player_experience.level as usize];

    if player_experience.amount_experience >= amount_of_experience_before_leveling {
        // 0 or the remaining experience
        player_experience.amount_experience -= amount_of_experience_before_leveling;

        player_experience.level += 1;

        // GG player leveled up
        next_state.set(GameState::PlayerLevelUp);
    }
}

fn player_received_damage(
    mut received_damage: EventReader<PlayerReceivedDamage>,
    mut player: Query<&mut Health, With<Player>>,
) {
    let mut player_health = player.single_mut();
    for event in received_damage.read() {
        player_health.0 -= event.damage;
    }
}

fn player_health_recovery(
    mut player: Query<(&mut Health, &HealthRecovery), With<Player>>,
    time: Res<Time>,
) {
    let (mut player_health, player_recovery) = player.single_mut();
    player_health.0 += player_recovery.0 * time.delta_seconds();
}
