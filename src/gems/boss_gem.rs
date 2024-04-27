use crate::components::*;
use crate::constants::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct GemsBossPlugin;

impl Plugin for GemsBossPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_boss_gem_on_enemy_death, gem_boss_retrieve_by_user)
                .run_if(in_state(GameState::Gameplay)),
        );
    }
}

fn spawn_boss_gem_on_enemy_death(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut boss_died: EventReader<EnemyBossDied>,
) {
    for event in boss_died.read() {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("gem_boss.png"),
                transform: Transform::from_xyz(
                    event.position.x,
                    event.position.y,
                    GEM_BOSS_Z_INDEX,
                ),
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
            GemBoss,
            Name::new("Gem Boss"),
        ));
    }
}

fn gem_boss_retrieve_by_user(
    mut commands: Commands,
    mut gems: Query<(Entity, &CollidingEntities), (With<GemBoss>, Without<ColliderDisabled>)>,
    player: Query<Entity, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let player = player.single();
    for (gem_entity, colliding_entities) in &mut gems {
        if colliding_entities.contains(player) {
            next_state.set(GameState::PlayerUpdateWeapon);

            // delete gem
            commands.entity(gem_entity).despawn_recursive();
        }
    }
}
