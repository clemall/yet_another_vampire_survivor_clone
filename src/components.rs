use crate::constants::*;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Gameplay,
    MainMenu,
    GameOver,
    PlayerLevelUp,
}

// PLAYER

#[derive(Component)]
pub struct Player {
    pub facing: Facing,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerStats {
    pub mul_max_health: f32,
    pub mul_move_speed: f32,
    // pub mul_recovery: u32,
    // pub mul_resistance: f32,
    // pub mul_power: f32,
    // pub mul_area: f32,
    // pub mul_attack_speed: f32,
    // pub mul_attack_duration: f32,
    // pub mul_attack_amount: u32,
    // pub mul_attack_reload_duration: f32,
    // pub mul_luck: f32,
    // pub mul_experience: f32,
    // pub mul_greed: f32,
    // pub mul_curse: f32,
    pub mul_magnet: f32,
    // pub mul_extra_life: f32,
}

#[derive(Resource, Debug, Deserialize, Serialize)]
pub struct PlayerMetaStats {
    pub data: PlayerStats,
}

// Will be set by a ron file for each character
#[derive(Resource, Debug, Deserialize, Serialize)]
pub struct CharacterStats {
    pub data: PlayerStats,
}
#[derive(Resource, Debug, Deserialize, Serialize)]
pub struct PlayerInGameStats {
    pub max_health: f32,
    pub move_speed: f32,
    // pub recovery: u32,
    // pub resistance: f32,
    // pub power: f32,
    // pub area: f32,
    // pub attack_speed: f32,
    // pub attack_duration: f32,
    // pub attack_amount: u32,
    // pub attack_reload_duration: f32,
    // pub luck: f32,
    // pub experience: f32,
    // pub greed: f32,
    // pub curse: f32,
    pub magnet: f32,
    // pub extra_life: f32,
}
pub const BASE_MAX_HEALTH: f32 = 100.0;
pub const BASE_MOVE_SPEED: f32 = 60.0;
pub const BASE_MAGNET: f32 = 20.0;
// Default value for all character before multiplication
impl Default for PlayerInGameStats {
    fn default() -> Self {
        Self {
            max_health: BASE_MAX_HEALTH,
            move_speed: BASE_MOVE_SPEED,
            magnet: BASE_MAGNET,
        }
    }
}

#[derive(Resource, Debug)]
pub struct PlayerExperience {
    pub level: u32,
    pub amount_experience: u32,
}

#[derive(Component)]
pub struct PlayerPickupRadius;

// HEALTH

#[derive(Component, Deref, DerefMut)]
pub struct Health(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct MaxHealth(pub f32);

// Enemy

#[derive(Resource, Debug, Deserialize, Serialize)]
pub struct EnemiesResource {
    pub bat: EnemyData,
    pub bee: EnemyData,
    pub golem: EnemyData,
    pub rabbit: EnemyData,
    pub skull: EnemyData,
    pub boss_wolf: EnemyData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnemyData {
    pub texture_patch: String,
    pub texture_layout_size: Vec2,
    pub texture_layout_columns: usize,
    pub texture_layout_rows: usize,
    pub animation_last_indice: usize,
    pub health: f32,
    pub speed: f32,
    pub damage: f32,
    pub collider_height: f32,
    pub collider_radius: f32,
    pub mass: f32,
    pub experience_drop: Option<u32>,
    pub is_boss: bool,
    pub is_semi_boss: bool,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EnemyTypes {
    Bat,
    Bee,
    Golem,
    Rabbit,
    Skull,
    BossWolf,
}

#[derive(Component, Deref, DerefMut)]
pub struct EnemyVelocity(pub Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct EnemySpeed(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct EnemyDamageOverTime(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct EnemyExperienceDrop(pub u32);

#[derive(Component)]
pub struct EnemyBossDrop;

// Animation

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
    pub is_repeating: bool,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub enum Facing {
    Left,
    Right,
}

// WEAPONS

#[derive(Component)]
pub struct ClawSpawner;
#[derive(Component)]
pub struct ArcaneMissileSpawner;
#[derive(Component)]
pub struct ShurikenSpawner;
#[derive(Component)]
pub struct ChainLightningSpawner;
#[derive(Component)]
pub struct FireAreaSpawner;
#[derive(Component)]
pub struct SlowDomeSpawner;
#[derive(Component)]
pub struct BouncingBallSpawner;
#[derive(Component)]
pub struct Claw;
#[derive(Component)]
pub struct ArcaneMissile;
#[derive(Component)]
pub struct FireArea;
#[derive(Component)]
pub struct Shuriken;
#[derive(Component)]
pub struct ChainLightning;
#[derive(Component)]
pub struct SlowDome;
#[derive(Component)]
pub struct BouncingBall;

// pub struct PayloadOnHit{
//     pub target: Entity,
//     pub target_position: Option<Vec3>,
// }

// #[derive(Resource, Debug)]
// pub struct SlowDomeOnHitSystems {
//     // entity ID
//     pub slow_enemy:SystemId<PayloadOnHit>,
// }
#[derive(Component)]
pub struct VelocityAura {
    pub value: f32,
    pub lifetime: Timer,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WeaponsTypes {
    Claw,
    FireArea,
    ArcaneMissile,
    Shuriken,
    ChainLightning,
    SlowDome,
    BouncingBall,
    BouncingBallSplit,
}
#[derive(Resource, Debug)]
pub struct PlayerWeapons {
    // entity ID
    pub weapons: Vec<WeaponsTypes>,
}

// GEM
#[derive(Component)]
pub struct Gem {
    pub experience: u32,
}

#[derive(Component)]
pub struct GemIsAttracted;

// EVENTS

#[derive(Event)]
pub struct EnemyDied {
    pub position: Vec3,
    pub experience: u32,
}

#[derive(Event)]
pub struct EnemyBossDied {
    pub position: Vec3,
}

#[derive(Event)]
pub struct EnemyReceivedDamage {
    pub damage: f32,
    pub enemy_entity: Entity,
    pub projectile_position: Vec3,
    pub impulse: Option<f32>,
    // pub position: Vec3,
    pub weapon_projectile_type: WeaponsTypes,
}

#[derive(Event)]
pub struct PlayerReceivedDamage {
    pub damage: f32,
}

#[derive(Event)]
pub struct CollectExperience {
    pub experience: u32,
}

#[derive(Event)]
pub struct SpawnEnemy {
    pub enemy_types: EnemyTypes,
}

// UI
#[derive(Component)]
pub struct PlayerHealthUIParent;

#[derive(Component)]
pub struct PlayerHealthUI;

#[derive(Component)]
pub struct PlayerExperienceBarUIParent;

#[derive(Component)]
pub struct PlayerExperienceUI;
#[derive(Component)]
pub struct LevelUpUI;

#[derive(Component)]
pub struct ButtonLevelUpUI;

#[derive(Component)]
pub struct WorldTextUI {
    pub lifetime: Timer,
    pub velocity: Vec2,
    pub position: Vec2,
}

// Attack and projectile

// bundle
/// Minimum component for a projectile to be colliding with enemies
/// Set the group, activate events and more
#[derive(Bundle)]
pub struct ProjectileBundleCollider {
    collision_group: CollisionGroups,
    active_events: ActiveEvents,
    colliding_entities: CollidingEntities,
}

impl Default for ProjectileBundleCollider {
    fn default() -> Self {
        Self {
            collision_group: CollisionGroups::new(PROJECTILE_GROUP, ENEMY_GROUP),
            active_events: ActiveEvents::COLLISION_EVENTS,
            colliding_entities: CollidingEntities::default(),
        }
    }
}

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct ProjectileType(pub WeaponsTypes);

#[derive(Component)]
pub struct ProjectileDamage(pub f32);

#[derive(Component)]
pub struct ProjectileDeleteOnHit;

#[derive(Component)]
pub struct ProjectileFollowPlayer;

#[derive(Component)]
pub struct ProjectileTimeBetweenDamage {
    pub timer: Timer,
}

#[derive(Component)]
pub struct ProjectileRotateOnSelf;

// #[derive(Component, Deref, DerefMut)]
// pub struct ProjectileVelocity(pub Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct ProjectileOrigin(pub Vec3);

#[derive(Component, Deref, DerefMut)]
pub struct ProjectileControlPoint(pub Vec3);

#[derive(Component, Deref, DerefMut)]
pub struct ProjectileSpeed(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct ProjectileDirection(pub Vec2);

#[derive(Component)]
pub struct ProjectileRotateAroundPlayer {
    pub angle: f32,
    pub distance: f32,
}

#[derive(Component)]
pub struct ProjectileSpiralAroundPlayer {
    pub angle: f32,
    pub distance: f32,
    pub spiral_speed: f32,
}

#[derive(Component, Deref, DerefMut)]
pub struct ProjectileBendLeftOrRight(pub bool);

#[derive(Component, Deref, DerefMut)]
pub struct ProjectileTarget(pub Entity);

#[derive(Component, Deref, DerefMut)]
pub struct ProjectileImpulse(pub f32);

#[derive(Component)]
pub struct ProjectileLifetime {
    pub timer: Timer,
}

#[derive(Component)]
pub struct ProjectileDeleteMe;

// #[derive(Component)]
// pub struct TriggersOnHit{
//     pub auras_systems: Vec<SystemId<PayloadOnHit>>
// }

// Use for projectile that target enemies and takes X seconds to meet the target
// arcane missile use it
#[derive(Component)]
pub struct ProjectileSpeedAsDuration {
    pub timer: Timer,
}

#[derive(Component, Deref, DerefMut)]
pub struct AlreadyHitEnemies {
    // entity ID
    pub seen: Vec<u32>,
}

// Delay between 2 attacks
// could be use as reload when the weapon has no real reload time
// like claw
// rename cast delay
#[derive(Component)]
pub struct DelayBetweenAttacks {
    pub timer: Timer,
}

#[derive(Component)]
pub struct AttackAmmo {
    pub size: u32,
    pub amount: u32,
    pub reload_time: f32, //seconds
}

// works with AttackAmmo.reload_time that is used to set
// the timer on AttackReloadDuration
#[derive(Component)]
pub struct AttackReloadDuration {
    pub timer: Timer,
}

// Waves
#[derive(Component)]
pub struct WaveManager {
    pub start_delay: Timer,
    pub end_delay: Timer,
    pub waves_prefab: Vec<Wave>,
    pub waves: Vec<Entity>,
}

#[derive(Component, Clone)]
pub struct Wave {
    pub enemy_type: EnemyTypes,
    pub delay_between_spawn: Timer,
    pub amount_per_timer_trigger: u32,
}

#[derive(Resource)]
pub struct WaveManagerGlobalTime {
    pub global_time: Stopwatch,
}
