use bevy_rapier2d::prelude::*;
use crate::components::*;
use bevy::{
    prelude::*,
};

pub struct GemsPlugin;

impl Plugin for GemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_gem_on_enemy_death,
                gem_retrieve_by_user,
                move_gem_attracted_by_player,
            ).run_if(in_state(GameState::Gameplay))
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
                transform: Transform::from_xyz(event.position.x, event.position.y, 0.0),
                ..default()
            },
            RigidBody::Fixed,
            Sensor,
            LockedAxes::ROTATION_LOCKED_Z,
            Collider::ball(7.0),
            Gem{
                experience:event.experience,
            },
            Name::new("Gem experience"),
        ));
    }
}



fn gem_retrieve_by_user(
    mut commands: Commands,
    mut gems: Query<(
        Entity,
        &Collider,
        &GlobalTransform,
        &mut Gem,
    ), Without<ColliderDisabled>>,
    mut player: Query<Entity, With<Player>>,
    rapier_context: Res<RapierContext>,
    mut collect_experience: EventWriter<CollectExperience>,
) {
    for (gem_entity, collider, transform, gem) in &mut gems {
        rapier_context.intersections_with_shape(
            transform.translation().truncate(),
            0.0,
            collider,
            QueryFilter::new(),
            |entity| {
                if let Ok(_entity) = player.get_mut(entity) {
                    collect_experience.send(
                        CollectExperience{
                            experience: gem.experience
                        }
                    );

                    // delete gem
                    commands.entity(gem_entity).despawn_recursive();
                }
                true
            },
        );

    }
}



fn move_gem_attracted_by_player(
    player: Query<&Transform, (With<Player>, Without<Gem>)>,
    mut gems: Query<&mut Transform, (Without<ColliderDisabled>,With<Gem>, With<GemIsAttracted>)>,
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


