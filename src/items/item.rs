use crate::components::*;
use bevy::prelude::*;
use std::collections::HashMap;
use std::fs;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_loot_table);
        app.add_systems(
            Update,
            (trigger_item, apply_luck_to_loot_table).run_if(in_state(GameState::Gameplay)),
        );
    }
}

fn setup_loot_table(mut commands: Commands) {
    let data = fs::read_to_string("assets/items.ron").unwrap();
    let item_resource: ItemsResource = ron::from_str(&data).unwrap();
    commands.insert_resource(item_resource.clone());

    let mut loot_table = LootTable {
        weighted_rarity: item_resource.weighted_rarity.clone(),
        item_by_rarity: HashMap::from([
            (Rarity::Common, Vec::new()),
            (Rarity::Uncommon, Vec::new()),
            (Rarity::Rare, Vec::new()),
            (Rarity::Epic, Vec::new()),
            (Rarity::Legendary, Vec::new()),
            (Rarity::Cursed, Vec::new()),
            (Rarity::Unique, Vec::new()),
        ]),
    };
    // populate loot_table.item_by_rarity
    // Will be a map of rarity with for each a list of item
    // the "item" is simply a key to the ItemsResource map of items
    for (item_key, item_data) in item_resource.items.clone() {
        for (rarity, _variation) in item_data.rarity_to_effects {
            match loot_table.item_by_rarity.get_mut(&rarity) {
                Some(item_list) => {
                    item_list.push(item_key.clone());
                }
                None => {
                    println!("Check why the key is not found, should not be possible");
                }
            };
        }
    }

    commands.insert_resource(loot_table);
}

fn trigger_item(
    mut item_event: EventReader<ItemPickup>,
    mut player_stats: ResMut<PlayerInGameStats>,
    item_resource: Res<ItemsResource>,
    mut loot_table: ResMut<LootTable>,
) {
    for event in item_event.read() {
        let item_data = item_resource.items.get(&event.item_key).unwrap();
        let item_effects = item_data.rarity_to_effects.get(&event.rarity).unwrap();

        for effect in item_effects.effects.iter() {
            match effect.base_stat {
                PlayerBaseStatsType::MaxHealth => {
                    player_stats.max_health += BASE_MAX_HEALTH * effect.value;
                }
                PlayerBaseStatsType::Recovery => {
                    player_stats.recovery += BASE_RECOVERY * effect.value;
                }
                PlayerBaseStatsType::MoveSpeed => {
                    player_stats.move_speed += BASE_MOVE_SPEED * effect.value;
                }
                PlayerBaseStatsType::Magnet => {
                    player_stats.magnet += BASE_MAGNET * effect.value;
                }
                PlayerBaseStatsType::Power => {
                    player_stats.power += BASE_POWER * effect.value;
                }
                PlayerBaseStatsType::Area => {
                    player_stats.area += BASE_AREA * effect.value;
                }
                PlayerBaseStatsType::Luck => {
                    player_stats.luck += BASE_LUCK * effect.value;
                }
                PlayerBaseStatsType::Resistance => {
                    player_stats.resistance += BASE_LUCK * effect.value;
                }
                PlayerBaseStatsType::AttackSpeed => {
                    player_stats.attack_speed += BASE_ATTACK_SPEED * effect.value;
                }
                PlayerBaseStatsType::AttackReloadDuration => {
                    // effect.value is negative
                    player_stats.attack_reload += BASE_ATTACK_RELOAD * effect.value;
                    // weapon reload time should NOT be lower than 0s
                    player_stats.attack_reload = player_stats.attack_reload.max(0.0);
                }
                PlayerBaseStatsType::AttackDuration => {
                    player_stats.attack_duration += BASE_ATTACK_DURATION * effect.value;
                }
                PlayerBaseStatsType::AttackAmount => {
                    player_stats.attack_amount += effect.value as u32;
                }
            }
        }

        // handle unique, remove them from the poll after getting picked
        if event.rarity == Rarity::Unique {
            loot_table
                .item_by_rarity
                .get_mut(&event.rarity)
                .unwrap()
                .retain(|key| key.as_str() != event.item_key)
        }
    }
}

fn apply_luck_to_loot_table(
    mut loot_table: ResMut<LootTable>,
    player_stats: Res<PlayerInGameStats>,
    item_resource: Res<ItemsResource>,
) {
    if !player_stats.is_changed() {
        return;
    }

    for (index, (rarity, value)) in item_resource.weighted_rarity.iter().enumerate() {
        let value_as_f32: f32 = *value as f32;
        match rarity {
            Rarity::Common | Rarity::Uncommon => {
                loot_table.weighted_rarity[index].1 = (value_as_f32
                    + (value_as_f32 - value_as_f32 * player_stats.luck))
                    .max(1.0) as u32;
            }
            Rarity::Rare => {
                // Luck as no effect for rare
            }
            Rarity::Epic | Rarity::Legendary => {
                loot_table.weighted_rarity[index].1 = (value_as_f32 * player_stats.luck) as u32;
            }
            _ => {}
        }
    }
}
