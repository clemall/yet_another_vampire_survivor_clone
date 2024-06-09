use crate::components::*;
use bevy::prelude::*;

pub struct WavesMap1Plugin;

impl Plugin for WavesMap1Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_waves_map_1);
    }
}

fn setup_waves_map_1(mut commands: Commands) {
    commands.spawn((
        WaveManager {
            start_timer: Timer::from_seconds(0.0, TimerMode::Once),
            end_timer: Timer::from_seconds(30.0, TimerMode::Once),
            waves_prefab: vec![
                Wave {
                    enemy_type: EnemyTypes::Bat,
                    delay_between_spawn: Timer::from_seconds(2.0, TimerMode::Repeating),
                    amount_per_timer_trigger: 2,
                },
                Wave {
                    enemy_type: EnemyTypes::BossWolf,
                    delay_between_spawn: Timer::from_seconds(10.0, TimerMode::Once),
                    amount_per_timer_trigger: 1,
                },
            ],
            waves: Vec::new(),
        },
        Name::new("Wave manager 0"),
    ));
    commands.spawn((
        WaveManager {
            start_timer: Timer::from_seconds(30.0, TimerMode::Once),
            end_timer: Timer::from_seconds(60.0, TimerMode::Once),
            waves_prefab: vec![
                Wave {
                    enemy_type: EnemyTypes::Bat,
                    delay_between_spawn: Timer::from_seconds(0.8, TimerMode::Repeating),
                    amount_per_timer_trigger: 1,
                },
                Wave {
                    enemy_type: EnemyTypes::Rabbit,
                    delay_between_spawn: Timer::from_seconds(1.0, TimerMode::Repeating),
                    amount_per_timer_trigger: 1,
                },
            ],
            waves: Vec::new(),
        },
        Name::new("Wave manager 1"),
    ));
    commands.spawn((
        WaveManager {
            start_timer: Timer::from_seconds(45.0, TimerMode::Once),
            end_timer: Timer::from_seconds(60.0, TimerMode::Once),
            waves_prefab: vec![Wave {
                enemy_type: EnemyTypes::Golem,
                delay_between_spawn: Timer::from_seconds(5.0, TimerMode::Repeating),
                amount_per_timer_trigger: 1,
            }],
            waves: Vec::new(),
        },
        Name::new("Wave manager 1"),
    ));

    commands.spawn((
        WaveManager {
            start_timer: Timer::from_seconds(50.0, TimerMode::Once),
            end_timer: Timer::from_seconds(90.0, TimerMode::Once),
            waves_prefab: vec![
                Wave {
                    enemy_type: EnemyTypes::Skull,
                    delay_between_spawn: Timer::from_seconds(0.2, TimerMode::Repeating),
                    amount_per_timer_trigger: 10,
                },
                Wave {
                    enemy_type: EnemyTypes::Golem,
                    delay_between_spawn: Timer::from_seconds(5.0, TimerMode::Repeating),
                    amount_per_timer_trigger: 1,
                },
            ],
            waves: Vec::new(),
        },
        Name::new("Wave manager 2"),
    ));

    commands.spawn((
        WaveManager {
            start_timer: Timer::from_seconds(90.0, TimerMode::Once),
            end_timer: Timer::from_seconds(160.0, TimerMode::Once),
            waves_prefab: vec![
                Wave {
                    enemy_type: EnemyTypes::Bee,
                    delay_between_spawn: Timer::from_seconds(0.8, TimerMode::Repeating),
                    amount_per_timer_trigger: 2,
                },
                Wave {
                    enemy_type: EnemyTypes::Rabbit,
                    delay_between_spawn: Timer::from_seconds(3.0, TimerMode::Repeating),
                    amount_per_timer_trigger: 3,
                },
            ],
            waves: Vec::new(),
        },
        Name::new("Wave manager 3"),
    ));
    commands.spawn((
        WaveManager {
            start_timer: Timer::from_seconds(160.0, TimerMode::Once),
            end_timer: Timer::from_seconds(165.0, TimerMode::Once),
            waves_prefab: vec![Wave {
                enemy_type: EnemyTypes::BossWolf,
                delay_between_spawn: Timer::from_seconds(0.1, TimerMode::Once),
                amount_per_timer_trigger: 1,
            }],
            waves: Vec::new(),
        },
        Name::new("Wave manager 4"),
    ));
}
