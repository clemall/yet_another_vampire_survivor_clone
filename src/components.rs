use bevy::math::Vec2;
use bevy::prelude::*;


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Gameplay,
    MainMenu,
    GameOver,
    PlayerLevelUp,
}

#[derive(Component)]
pub struct Player{
    pub facing: Facing,
}

#[derive(Resource, Debug)]
pub struct PlayerExperience {
    pub level:u32,
    pub amount_experience: u32,
}



#[derive(Component, Deref, DerefMut)]
pub struct Health(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct MaxHealth(pub f32);

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Deref, DerefMut)]
pub struct EnemyVelocity(pub Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct EnemySpeed(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct EnemyDamageOverTime(pub f32);

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


#[derive(Component)]
pub struct ClawSpawner;
#[derive(Component)]
pub struct ArcaneMissileSpawner;

#[derive(Component)]
pub struct FireAreaSpawner;
#[derive(Component)]
pub struct Claw;
#[derive(Component)]
pub struct ArcaneMissile;
#[derive(Component)]
pub struct FireArea;


#[derive(Debug, PartialEq)]
pub enum WeaponsTypes {
    Claw,
    FireArea,
    ArcaneMissile,
}
#[derive(Resource, Debug)]
pub struct PlayerWeapons {
    // entity ID
    pub weapons:Vec<WeaponsTypes>,
}




#[derive(Component)]
pub struct Gem{
    pub experience:u32,
}


// EVENTS

#[derive(Event)]
pub struct EnemyDied{
    pub position:Vec3,
    pub experience:u32,
}

#[derive(Event)]
pub struct CollectExperience{
    pub experience:u32,
}

// UI
#[derive(Component)]
pub struct PlayerUI;

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

#[derive(Component)]
pub struct HealthUI;



// Attack and projectile
#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct ProjectileDamage(pub f32);

#[derive(Component)]
pub struct DeleteProjectileOnHit;

#[derive(Component)]
pub struct ProjectileTimeBetweenDamage {
    pub timer: Timer,
}

// Use for projectile that target enemies and takes X seconds to meet the target
// arcane missile use it
#[derive(Component)]
pub struct ProjectileSpeedAsTime {
    pub timer: Timer,
}

#[derive(Component, Deref, DerefMut)]
pub struct ProjectileVelocity(pub Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct ProjectileOrigin(pub Vec3);

#[derive(Component, Deref, DerefMut)]
pub struct ProjectileControlPoint(pub Vec3);

#[derive(Component, Deref, DerefMut)]
pub struct ProjectileSpeed(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct ProjectileBendLeftOrRight(pub bool);

#[derive(Component, Deref, DerefMut)]
pub struct ProjectileTarget(pub Entity);


// lifetime of the projectile
// Can be use for attack like Claw
#[derive(Component)]
pub struct ProjectileLifetime {
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
pub struct AttackDelayBetweenAttacks {
    pub timer: Timer,
}

// Delay before weapon can attack again
// arcane missile fire every X for 3 attacks with a delay of Y between each attacks
// X is AttackReload, Y would be AttackTimer
// rename recharge time
#[derive(Component)]
pub struct AttackReloadDuration {
    pub timer: Timer,
}

#[derive(Component)]
pub struct AttackAmmo{
    pub size: u32,
    pub amount:u32,
    pub reload_time: f32, //seconds
}
