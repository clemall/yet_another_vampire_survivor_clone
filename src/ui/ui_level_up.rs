use crate::components::*;
use crate::constants::{FONT, FONT_BOLD, SCREEN_WIDTH};
use bevy::prelude::*;
use rand::distributions::{Distribution, WeightedIndex};
use rand::seq::SliceRandom;
use rand::Rng;

pub struct UiLevelUpPlugin;

impl Plugin for UiLevelUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::PlayerLevelUp), spawn_level_up_ui);
        app.add_systems(OnExit(GameState::PlayerLevelUp), despawn_level_up_ui);
        app.add_systems(
            Update,
            button_interaction.run_if(in_state(GameState::PlayerLevelUp)),
        );
    }
}

const HOVERED_BUTTON: Color = Color::rgb(0.80, 0.80, 0.80);
const NORMAL_BUTTON: Color = Color::rgb(1., 1., 1.);
const POPUP_BG_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.95);

fn despawn_level_up_ui(mut commands: Commands, ui: Query<Entity, With<LevelUpUI>>) {
    for ui in &ui {
        commands.entity(ui).despawn_recursive();
    }
}

fn spawn_level_up_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera: Query<&Camera>,
    loot_table: Res<LootTable>,
    items_resource: Res<ItemsResource>,
) {
    let level_up_parent = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            LevelUpUI,
            Name::new("UI Group Level UP"),
        ))
        .id();

    let level_up_popup = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(80.0),
                position_type: PositionType::Relative,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            background_color: POPUP_BG_COLOR.into(),
            ..default()
        })
        .id();

    commands
        .entity(level_up_parent)
        .push_children(&[level_up_popup]);

    let camera = camera.single();
    let view_dimensions = camera.logical_viewport_size().unwrap();
    let ratio = view_dimensions.x / SCREEN_WIDTH as f32;

    let dist = WeightedIndex::new(loot_table.weighted_rarity.iter().map(|item| item.1)).unwrap();

    for index in 0..5 {
        let mut rarity = loot_table.weighted_rarity[dist.sample(&mut rand::thread_rng())].0;

        // has a small chance to be a cursed item
        if rand::thread_rng().gen_range(0.0..100.0) < 1.0 {
            rarity = loot_table.weighted_rarity[5].0;
        }
        let item_key = loot_table
            .item_by_rarity
            .get(&rarity)
            .unwrap()
            .choose(&mut rand::thread_rng())
            .unwrap();

        let item_name = items_resource
            .items
            .get(&item_key.clone())
            .unwrap()
            .name
            .clone();

        let item_description = items_resource
            .items
            .get(&item_key.clone())
            .unwrap()
            .rarity_to_effects
            .get(&rarity)
            .unwrap()
            .description
            .clone();

        let texture = match rarity {
            Rarity::Common => asset_server.load("item_ui_background_common.png"),
            Rarity::Uncommon => asset_server.load("item_ui_background_uncommon.png"),
            Rarity::Rare => asset_server.load("item_ui_background_rare.png"),
            Rarity::Epic => asset_server.load("item_ui_background_epic.png"),
            Rarity::Legendary => asset_server.load("item_ui_background_legendary.png"),
            Rarity::Cursed => asset_server.load("item_ui_background_curse.png"),
            Rarity::Unique => asset_server.load("item_ui_background_unique.png"),
        };

        let card_item = commands
            .spawn((
                ButtonBundle {
                    style: Style {
                        position_type: PositionType::Relative,
                        width: Val::Px(80. * ratio),
                        height: Val::Px(112. * ratio),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    image: UiImage::new(texture),
                    ..default()
                },
                ButtonUpgrade {
                    item_key: item_key.clone().into(),
                    rarity: rarity,
                },
            ))
            .id();

        let item_name = commands
            .spawn(
                TextBundle::from_section(
                    item_name,
                    TextStyle {
                        font: asset_server.load(FONT_BOLD),
                        font_size: 22.0,
                        color: Color::BLACK,
                        ..default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(61.0),

                    ..default()
                }),
            )
            .id();

        let item_description = commands
            .spawn(
                TextBundle::from_section(
                    item_description,
                    TextStyle {
                        font: asset_server.load(FONT),
                        font_size: 16.0,
                        color: Color::BLACK,
                        ..default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(70.0),
                    left: Val::Percent(5.0),
                    width: Val::Percent(90.0),
                    ..default()
                }),
            )
            .id();

        let item_rarity = commands
            .spawn(
                TextBundle::from_section(
                    rarity.name(),
                    TextStyle {
                        font: asset_server.load(FONT_BOLD),
                        font_size: 16.0,
                        color: Color::BLACK,
                        ..default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(90.0),
                    left: Val::Percent(5.0),
                    ..default()
                }),
            )
            .id();

        commands.entity(card_item).push_children(&[item_name]);
        commands
            .entity(card_item)
            .push_children(&[item_description]);
        commands.entity(card_item).push_children(&[item_rarity]);

        commands.entity(level_up_popup).push_children(&[card_item]);
    }
}

fn button_interaction(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonUpgrade), // UiImage
        (Changed<Interaction>, With<Button>),
    >,
    mut item_pickup: EventWriter<ItemPickup>,
) {
    for (interaction, mut image, upgrade) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                item_pickup.send(ItemPickup {
                    item_key: upgrade.item_key.clone(),
                    rarity: upgrade.rarity,
                });

                next_state.set(GameState::Gameplay);
            }
            Interaction::Hovered => {
                *image = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *image = NORMAL_BUTTON.into();
            }
        }
    }
}
