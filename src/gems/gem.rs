use crate::components::*;
use crate::constants::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct GemsPlugin;

impl Plugin for GemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_gem_on_enemy_death,
                gem_retrieve_by_user,
                move_gem_attracted_by_player,
                gem_hit_player_pickup_radius,
            )
                .run_if(in_state(GameState::Gameplay)),
        );
    }
}

fn spawn_gem_on_enemy_death(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut enemy_died: EventReader<EnemyDied>,
) {
    for event in enemy_died.read() {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("gem.png"),
                transform: Transform::from_xyz(event.position.x, event.position.y, GEM_Z_INDEX),
                ..default()
            },
            RigidBody::Fixed,
            Sensor,
            LockedAxes::ROTATION_LOCKED_Z,
            Collider::ball(7.0),
            CollisionGroups::new(GEM_GROUP, PLAYER_GROUP),
            ActiveEvents::COLLISION_EVENTS,
            ActiveCollisionTypes::STATIC_STATIC,
            CollidingEntities::default(),
            Gem {
                experience: event.experience,
            },
            Name::new("Gem experience"),
        ));
    }
}

fn gem_hit_player_pickup_radius(
    mut commands: Commands,
    mut gems: Query<
        (Entity, &CollidingEntities),
        (
            Changed<CollidingEntities>,
            Without<ColliderDisabled>,
            Without<GemIsAttracted>,
        ),
    >,
    player_pickup: Query<Entity, With<PlayerPickupRadius>>,
) {
    let player_pickup = player_pickup.single();
    for (gem_entity, colliding_entities) in &mut gems {
        if colliding_entities.contains(player_pickup) {
            commands.entity(gem_entity).try_insert(GemIsAttracted);
        }
    }
}

fn gem_retrieve_by_user(
    mut commands: Commands,
    mut gems: Query<
        (Entity, &Gem, &CollidingEntities),
        (Without<ColliderDisabled>, With<GemIsAttracted>),
    >,
    player: Query<Entity, With<Player>>,
    mut collect_experience: EventWriter<CollectExperience>,
) {
    let player = player.single();
    for (gem_entity, gem, colliding_entities) in &mut gems {
        if colliding_entities.contains(player) {
            collect_experience.send(CollectExperience {
                experience: gem.experience,
            });

            // delete gem
            commands.entity(gem_entity).despawn_recursive();
        }
    }
}

fn move_gem_attracted_by_player(
    player: Query<&Transform, (With<Player>, Without<Gem>)>,
    mut gems: Query<&mut Transform, (Without<ColliderDisabled>, With<Gem>, With<GemIsAttracted>)>,
    time: Res<Time>,
) {
    let player_transform = player.single();
    for mut gem_transform in &mut gems {
        let direction = (gem_transform.translation.truncate()
            - player_transform.translation.truncate())
        .normalize();

        gem_transform.translation.x -= direction.x * time.delta_seconds() * 200.0;
        gem_transform.translation.y -= direction.y * time.delta_seconds() * 200.0;
    }
}
