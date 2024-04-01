use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::constants::*;


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
pub struct Player{
    pub facing: Facing,
}

#[derive(Resource, Debug)]
pub struct PlayerExperience {
    pub level:u32,
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
#[derive(Component)]
pub struct Enemy;

#[derive(Component, Deref, DerefMut)]
pub struct EnemyVelocity(pub Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct EnemySpeed(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct EnemyDamageOverTime(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct EnemyExperienceDrop(pub u32);

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
pub struct VelocityAura{
    pub value: f32,
    pub lifetime: Timer,
}

#[derive(Debug, PartialEq)]
pub enum AurasTypes {
    Slow,
    OnFire,
    Poisoned,
}

#[derive(Debug, PartialEq)]
pub enum WeaponsTypes {
    Claw,
    FireArea,
    ArcaneMissile,
    Shuriken,
    ChainLightning,
    SlowDome,
}
#[derive(Resource, Debug)]
pub struct PlayerWeapons {
    // entity ID
    pub weapons:Vec<WeaponsTypes>,
}






// GEM
#[derive(Component)]
pub struct Gem{
    pub experience:u32,
}

#[derive(Component)]
pub struct GemIsAttracted;










// EVENTS

#[derive(Event)]
pub struct EnemyDied{
    pub position:Vec3,
    pub experience:u32,
}

#[derive(Event)]
pub struct EnemyReceivedDamage{
    pub damage:f32,
    pub enemy_entity:Entity,
}

#[derive(Event)]
pub struct PlayerReceivedDamage{
    pub damage:f32,
}

#[derive(Event)]
pub struct EnemyHitByProjectile{
    pub enemy_entity:Entity,
    pub impulse: Option<f32>,
    pub effects: Option<Vec<AurasTypes>>,
    
}

#[derive(Event)]
pub struct CollectExperience{
    pub experience:u32,
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
#[derive(Bundle)]
pub struct ProjectileBundleCollider{
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
pub struct ProjectileRotateAroundPlayer{
    pub angle: f32,
    pub distance: f32,
}

#[derive(Component)]
pub struct ProjectileSpiralAroundPlayer{
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
pub struct ProjectileAuraOnHit {
    pub effects: Vec<AurasTypes>,
}



// Use for projectile that target enemies and takes X seconds to meet the target
// arcane missile use it
#[derive(Component)]
pub struct ProjectileSpeedAsDuration {
    pub timer: Timer,
}

#[derive(Component, Deref, DerefMut)]
pub struct AlreadyHitEnemies {
    // entity ID
    pub seen:Vec<u32>,
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
pub struct AttackAmmo{
    pub size: u32,
    pub amount:u32,
    pub reload_time: f32, //seconds
}

// works with AttackAmmo.reload_time that is used to set
// the timer on AttackReloadDuration
#[derive(Component)]
pub struct AttackReloadDuration {
    pub timer: Timer,
}
