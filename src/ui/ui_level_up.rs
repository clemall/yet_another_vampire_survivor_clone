use crate::components::*;
use crate::constants::SCREEN_WIDTH;
use bevy::prelude::*;
use rand::seq::SliceRandom;

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

    // read this: https://stackoverflow.com/questions/34215280/how-can-i-randomly-select-one-element-from-a-vector-or-array
    let rarity: [Rarity; 7] = Rarity::array();

    for _ in 0..5 {
        let rarity = *rarity.choose(&mut rand::thread_rng()).unwrap();
        let item = *rarity.get_items().choose(&mut rand::thread_rng()).unwrap();

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
                    item_type: item,
                    rarity: rarity,
                },
            ))
            .id();

        let item_name = commands
            .spawn(
                TextBundle::from_section(
                    item.name(),
                    TextStyle {
                        font_size: 16.0,
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
                    item.description(),
                    TextStyle {
                        font_size: 12.0,
                        color: Color::BLACK,
                        ..default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(70.0),
                    left: Val::Percent(5.0),
                    ..default()
                }),
            )
            .id();

        let item_rarity = commands
            .spawn(
                TextBundle::from_section(
                    rarity.name(),
                    TextStyle {
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
                println!("{:?}", upgrade.item_type);
                item_pickup.send(ItemPickup {
                    item_type: upgrade.item_type,
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
