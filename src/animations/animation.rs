use bevy::prelude::*;
use crate::components::{AnimationIndices, AnimationTimer};


pub struct AnimationSimplePlugin;

impl Plugin for AnimationSimplePlugin {
    fn build(&self, app: &mut App) {
        // basic animation
        app.add_systems(Update, animate_sprite);
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if indices.is_repeating {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
            else {
                if  atlas.index < indices.last{
                    atlas.index = atlas.index + 1;
                }
            }

        }
    }
}
