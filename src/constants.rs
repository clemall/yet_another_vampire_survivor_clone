use bevy::prelude::Color;
use bevy_rapier2d::prelude::*;
pub const SCREEN_WIDTH: i32 = 640;
pub const SCREEN_HEIGHT: i32 = 360;

// start at level 2 because player is already level 1
pub const MAP_LEVEL_EXPERIENCE: [u32; 25] = [
    5, 7, 10, 15, 20, 30, 40, 60, 80, 110, 140, 180, 220, 270, 320, 380, 460, 550, 660, 770, 880,
    990, 1100, 1250, 1500,
];

pub const MAX_LEVEL: u32 = 5;

// collision group
pub const PLAYER_GROUP: Group = Group::GROUP_1;
pub const ENEMY_GROUP: Group = Group::GROUP_2;
pub const PROJECTILE_GROUP: Group = Group::GROUP_3;
pub const GEM_GROUP: Group = Group::GROUP_30;

// pub const FONT: &str = "fonts/dogica.ttf";
// pub const DAMAGE_FONT_SIZE: f32 = 11.0;

// pub const FONT: &str = "fonts/dogicapixel.ttf";
// pub const DAMAGE_FONT_SIZE: f32 = 11.0;

pub const DAMAGE_FONT: &str = "fonts/VCR_OSD_MONO_1.001.ttf";
pub const DAMAGE_FONT_SIZE: f32 = 18.0;
// pub const FONT: &str = "fonts/VT323-Regular.ttf";
// pub const DAMAGE_FONT_SIZE: f32 = 22.0;

// pub const DAMAGE_FONT_COLOR: Color = Color::rgb(0.164, 0.686, 0.905); // blue
// pub const DAMAGE_FONT_COLOR: Color = Color::rgb(0.937, 0.956, 0.207); // yellow
pub const DAMAGE_FONT_COLOR: Color = Color::WHITE; // white

pub const FONT: &str = "fonts/FiraMono-Medium.ttf";
pub const FONT_BOLD: &str = "fonts/FiraSans-Bold.ttf";

// Z index.....
pub const ENEMY_Z_INDEX: f32 = 99.0;
pub const PLAYER_Z_INDEX: f32 = 98.0;
pub const PROJECTILE_Z_INDEX: f32 = 199.0;
pub const GEM_Z_INDEX: f32 = 50.0; // under enemies makes it better
pub const GEM_BOSS_Z_INDEX: f32 = 999.0; // Above anything
