use crate::components::*;
use bevy::prelude::*;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                trigger_item_max_health,
                trigger_item_magnet,
                trigger_item_move_speed,
                trigger_item_power,
                trigger_item_curse_damage,
            )
                .run_if(in_state(GameState::Gameplay)),
        );
    }
}

fn trigger_item_max_health(
    mut item_pickup: EventReader<ItemPickup>,
    mut player_stats: ResMut<PlayerInGameStats>,
) {
    for event in item_pickup.read() {
        if event.item_type != ItemsTypes::MaxHealth {
            continue;
        }

        match event.rarity {
            Rarity::Common => {
                // increase max health by 10% of base value
                player_stats.max_health += BASE_MAX_HEALTH * 0.1;
            }
            Rarity::Uncommon => {
                player_stats.max_health += BASE_MAX_HEALTH * 0.15;
            }
            Rarity::Rare => {
                player_stats.max_health += BASE_MAX_HEALTH * 0.2;
            }
            Rarity::Epic => {
                player_stats.max_health += BASE_MAX_HEALTH * 0.3;
            }
            Rarity::Legendary => {
                player_stats.max_health += BASE_MAX_HEALTH * 0.5;
            }
            _ => {}
        }
    }
}

fn trigger_item_magnet(
    mut item_pickup: EventReader<ItemPickup>,
    mut player_stats: ResMut<PlayerInGameStats>,
) {
    for event in item_pickup.read() {
        if event.item_type != ItemsTypes::Magnet {
            continue;
        }

        match event.rarity {
            Rarity::Common => {
                player_stats.magnet += BASE_MAGNET * 0.1;
            }
            Rarity::Uncommon => {
                player_stats.magnet += BASE_MAGNET * 0.15;
            }
            Rarity::Rare => {
                player_stats.magnet += BASE_MAGNET * 0.2;
            }
            Rarity::Epic => {
                player_stats.magnet += BASE_MAGNET * 0.3;
            }
            Rarity::Legendary => {
                player_stats.magnet += BASE_MAGNET * 0.5;
            }
            _ => {}
        }
    }
}

fn trigger_item_move_speed(
    mut item_pickup: EventReader<ItemPickup>,
    mut player_stats: ResMut<PlayerInGameStats>,
) {
    for event in item_pickup.read() {
        if event.item_type != ItemsTypes::MoveSpeed {
            continue;
        }

        match event.rarity {
            Rarity::Common => {
                player_stats.move_speed += BASE_MOVE_SPEED * 0.05;
            }
            Rarity::Uncommon => {
                player_stats.move_speed += BASE_MOVE_SPEED * 0.07;
            }
            Rarity::Rare => {
                player_stats.move_speed += BASE_MOVE_SPEED * 0.1;
            }
            Rarity::Epic => {
                player_stats.move_speed += BASE_MOVE_SPEED * 0.15;
            }
            Rarity::Legendary => {
                player_stats.move_speed += BASE_MOVE_SPEED * 0.2;
            }
            _ => {}
        }
    }
}

fn trigger_item_power(
    mut item_pickup: EventReader<ItemPickup>,
    mut player_stats: ResMut<PlayerInGameStats>,
) {
    for event in item_pickup.read() {
        if event.item_type != ItemsTypes::Power {
            continue;
        }

        match event.rarity {
            Rarity::Common => {
                player_stats.power += BASE_POWER * 0.05;
            }
            Rarity::Uncommon => {
                player_stats.power += BASE_POWER * 0.1;
            }
            Rarity::Rare => {
                player_stats.power += BASE_POWER * 0.2;
            }
            Rarity::Epic => {
                player_stats.power += BASE_POWER * 0.3;
            }
            Rarity::Legendary => {
                player_stats.power += BASE_POWER * 0.5;
            }
            _ => {}
        }
    }
}

fn trigger_item_curse_damage(
    mut item_pickup: EventReader<ItemPickup>,
    mut player_stats: ResMut<PlayerInGameStats>,
) {
    for event in item_pickup.read() {
        if event.item_type != ItemsTypes::WipCurseDamage {
            continue;
        }

        match event.rarity {
            Rarity::Cursed => {
                // double damage
                player_stats.power += BASE_POWER * 1.0;
                // but make you slow
                player_stats.move_speed -= BASE_MOVE_SPEED * 0.5;
            }
            _ => {}
        }
    }
}
