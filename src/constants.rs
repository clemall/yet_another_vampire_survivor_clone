use bevy_rapier2d::prelude::*;
pub const SCREEN_WIDTH: i32 = 640;
pub const SCREEN_HEIGHT: i32 = 360;

// start at level 2 because player is already level 1
pub const MAP_LEVEL_EXPERIENCE: [u32; 5] = [0, 150, 200, 300, 500000];

pub const MAX_LEVEL: u32 = 5;

// collision group
pub const PLAYER_GROUP: Group = Group::GROUP_1;
pub const ENEMY_GROUP: Group = Group::GROUP_2;
pub const PROJECTILE_GROUP: Group = Group::GROUP_3;
pub const GEM_GROUP: Group = Group::GROUP_30;
