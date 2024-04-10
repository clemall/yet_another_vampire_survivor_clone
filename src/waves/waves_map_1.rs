use crate::components::*;
use bevy::prelude::*;

pub struct WavesMap1Plugin;

impl Plugin for WavesMap1Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_waves);
    }
}

fn setup_waves(mut commands: Commands) {
    commands.spawn((
        WaveManager {
            start_delay: Timer::from_seconds(1.0, TimerMode::Once),
            end_delay: Timer::from_seconds(10.0, TimerMode::Once),
            waves_prefab: vec![
                Wave {
                    enemy_type: EnemyTypes::Bat,
                    delay_between_spawn: Timer::from_seconds(1.0, TimerMode::Repeating),
                },
                Wave {
                    enemy_type: EnemyTypes::Bat,
                    delay_between_spawn: Timer::from_seconds(2.0, TimerMode::Repeating),
                },
                Wave {
                    enemy_type: EnemyTypes::Golem,
                    delay_between_spawn: Timer::from_seconds(5.0, TimerMode::Repeating),
                },
                Wave {
                    enemy_type: EnemyTypes::Rabbit,
                    delay_between_spawn: Timer::from_seconds(1.0, TimerMode::Repeating),
                },
            ],
            waves: Vec::new(),
        },
        Name::new("Wave manager 1"),
    ));

    commands.spawn((
        WaveManager {
            start_delay: Timer::from_seconds(11.0, TimerMode::Once),
            end_delay: Timer::from_seconds(40.0, TimerMode::Once),
            waves_prefab: vec![
                Wave {
                    enemy_type: EnemyTypes::Skull,
                    delay_between_spawn: Timer::from_seconds(0.2, TimerMode::Repeating),
                },
                Wave {
                    enemy_type: EnemyTypes::Golem,
                    delay_between_spawn: Timer::from_seconds(5.0, TimerMode::Repeating),
                },
            ],
            waves: Vec::new(),
        },
        Name::new("Wave manager 2"),
    ));
}
