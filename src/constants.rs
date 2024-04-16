use bevy::prelude::Color;
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
