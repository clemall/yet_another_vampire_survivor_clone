use std::fmt;
use bevy::math::Vec2;
use bevy::prelude::{Component, Deref, DerefMut, Entity, Resource, Timer};

#[derive(Component)]
pub struct Player{
    pub facing: Facing,
}

#[derive(Component)]
pub struct PlayerUI;

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


#[derive(Component)]
pub struct HealthUI;

pub enum Facing {
    Left,
    Right,
}

#[derive(Component)]
pub struct Claw {
    pub damage: f32,
}

#[derive(Component)]
pub struct FireArea {
    pub damage: f32,
}

#[derive(Component)]
pub struct ClawSpawner;

#[derive(Component)]
pub struct AttackDuration {
    pub timer: Timer,
}

#[derive(Component)]
pub struct AlreadyHitEnemies {
    // entity ID
    pub seen:Vec<u32>,
}


#[derive(Component)]
pub struct AttackTimer {
    pub timer: Timer,
}


#[derive(Component)]
pub struct WorldTextUI {
    pub lifetime: Timer,
    pub velocity: Vec2,
    pub position: Vec2,
}

#[derive(Debug, PartialEq)]
pub enum WeaponsTypes {
    CLAW,
    FIRE_AREA
}
#[derive(Resource, Debug)]
pub struct PlayerWeapons {
    // entity ID
    pub weapons:Vec<WeaponsTypes>,
}
