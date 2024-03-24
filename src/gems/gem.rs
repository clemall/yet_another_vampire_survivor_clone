use std::time::Duration;
use bevy_pixel_camera::{PixelViewport, PixelZoom};
use bevy_rapier2d::prelude::*;
use crate::components::*;
use crate::enemies::enemy::{damage_enemy, enemy_death_check};
use bevy::{
    math::{cubic_splines::CubicCurve, vec3},
    prelude::*,
};
use bevy_inspector_egui::egui::debug_text::print;

pub struct GemsPlugin;

impl Plugin for GemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_gem_on_enemy_death,
                gem_retrieve_by_user
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
    mut player: Query<(&Transform), With<Player>>,
    rapier_context: Res<RapierContext>,
    mut collect_experience: EventWriter<CollectExperience>,
) {
    for (gem_entity, collider, transform, mut gem) in &mut gems {
        rapier_context.intersections_with_shape(
            transform.translation().truncate(),
            0.0,
            collider,
            QueryFilter::new(),
            |entity| {
                if let Ok((player_transform)) = player.get_mut(entity) {
                    println!("Add experience ({}) to player", gem.experience);
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
