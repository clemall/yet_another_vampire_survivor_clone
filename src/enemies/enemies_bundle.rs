use crate::components::*;
use crate::constants::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub sprite_bundle: SpriteBundle,
    pub texture_atlas: TextureAtlas,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
    pub rigid_body: RigidBody,
    pub locked_axes: LockedAxes,
    pub damping: Damping,
    pub collider: Collider,
    pub collision_group: CollisionGroups,
    pub active_events: ActiveEvents,
    pub active_collision_types: ActiveCollisionTypes,
    pub collider_mass_properties: ColliderMassProperties,
    pub colliding_entities: CollidingEntities,
    pub enemy: Enemy,
    pub health: Health,
    pub enemy_speed: EnemySpeed,
    pub enemy_velocity: EnemyVelocity,
    pub enemy_damage_overtime: EnemyDamageOverTime,
    pub enemy_experience_drop: EnemyExperienceDrop,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: Default::default(),
            texture_atlas: Default::default(),
            animation_indices: AnimationIndices {
                first: 0,
                last: 0,
                is_repeating: true,
            },
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED_Z,
            damping: Damping {
                linear_damping: 100.0,
                angular_damping: 1.0,
            },
            collider: Default::default(),
            collision_group: CollisionGroups::new(
                ENEMY_GROUP,
                PLAYER_GROUP | ENEMY_GROUP | PROJECTILE_GROUP,
            ),
            active_events: ActiveEvents::COLLISION_EVENTS,
            active_collision_types: Default::default(),
            collider_mass_properties: ColliderMassProperties::Density(2.0),
            colliding_entities: CollidingEntities::default(),
            enemy: Enemy,
            health: Health(10.0),
            enemy_speed: EnemySpeed(25.0),
            enemy_velocity: EnemyVelocity(Vec2::new(0.0, 0.0)),
            enemy_damage_overtime: EnemyDamageOverTime(10.0),
            enemy_experience_drop: EnemyExperienceDrop(1),
        }
    }
}
