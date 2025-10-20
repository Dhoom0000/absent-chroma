use bevy::{camera::visibility::RenderLayers, prelude::*, ui::FocusPolicy};

use crate::client::{AppState, LAYER_UI};

mod actions;

pub struct UiPlugin;

#[derive(Debug, Clone, Component)]
struct MainMenu;

#[derive(Resource)]
struct MainMenuReady;

#[derive(Component, Clone)]
enum UiLabelType {
    Play,
    Connect,
    Exit,
}

impl UiLabelType {
    fn iter() -> impl Iterator<Item = UiLabelType> {
        [UiLabelType::Play, UiLabelType::Connect, UiLabelType::Exit].into_iter()
    }

    fn as_str(&self) -> &'static str {
        match self {
            UiLabelType::Play => "Play",
            UiLabelType::Connect => "Connect",
            UiLabelType::Exit => "Exit",
        }
    }
}

impl UiPlugin {
    fn render_main_menu(mut commands: Commands) {
        let camera_config = Camera {
            order: 10,
            is_active: true,
            ..default()
        };

        commands.spawn((
            camera_config,
            Camera2d::default(),
            UiPickingCamera,
            RenderLayers::layer(LAYER_UI),
        ));

        let base_node = Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            box_sizing: BoxSizing::BorderBox,
            position_type: PositionType::Relative,
            overflow: Overflow::clip(),
            overflow_clip_margin: OverflowClipMargin::padding_box(),
            aspect_ratio: Some(2560. / 1440.),
            align_items: AlignItems::Start,
            justify_items: JustifyItems::Stretch,
            // align_self: AlignSelf::Start,
            // justify_self: JustifySelf::Start,
            align_content: AlignContent::FlexStart,
            justify_content: JustifyContent::Start, // check this
            margin: UiRect::all(Val::Px(1.)),
            padding: UiRect::all(Val::Px(10.)),
            border: UiRect::all(Val::Px(5.)),
            flex_wrap: FlexWrap::Wrap,
            // flex_grow: 1.,
            // flex_shrink: 1.,
            flex_basis: Val::Auto,
            row_gap: Val::Px(10.),
            column_gap: Val::Px(10.),
            grid_auto_flow: GridAutoFlow::Column,
            ..Default::default()
        };

        // Define a main menu Node and some components.
        let mut main_menu_bundle = (
            base_node.clone(),
            FocusPolicy::Pass,
            BackgroundColor(Color::Srgba(Srgba::hex("171717").unwrap())),
            BorderColor::all(Color::WHITE),
            BorderRadius::all(Val::Px(5.)),
            Visibility::Visible,
            MainMenu,
            RenderLayers::layer(LAYER_UI),
        );

        main_menu_bundle.0.justify_self = JustifySelf::Stretch;
        main_menu_bundle.0.align_content = AlignContent::Center;
        main_menu_bundle.0.justify_content = JustifyContent::Center;

        // Node Bundle for title.
        let title_text_bundle = (
            Text::new("Absent Chroma"),
            TextFont {
                font_size: 80.,
                ..default()
            },
            BorderColor::all(Color::Srgba(Srgba::hex("0000ff").unwrap())),
            base_node.clone(),
            Visibility::Inherited,
        );

        // Spawn the main node.
        let main_node_id = commands.spawn(main_menu_bundle).id();

        // Spawn all other entities as children of the main node.
        commands.entity(main_node_id).with_children(|parent| {
            parent.spawn(title_text_bundle);

            // Spawn all label variants.
            for variant in UiLabelType::iter() {
                let text_bundle = (
                    Text::new(variant.as_str()),
                    TextFont {
                        font_size: 42.,
                        ..default()
                    },
                    TextColor(Color::Srgba(Srgba::hex("00ff00").unwrap())),
                    variant,
                    Button,
                    BorderColor::all(Color::Srgba(Srgba::hex("ff00ff").unwrap())),
                    base_node.clone(),
                    Visibility::Inherited,
                );

                parent.spawn(text_bundle);
            }
        });
    }

    fn show_menu(query: Option<Query<&mut Camera, With<UiPickingCamera>>>) {
        match query {
            Some(mut camera) => {
                let mut rendering = camera
                    .single_mut()
                    .expect("Error querying Main Menu Camera");

                rendering.is_active = true;
            }

            None => {}
        }
    }

    fn hide_menu(query: Option<Query<&mut Camera, With<UiPickingCamera>>>) {
        match query {
            Some(mut camera) => {
                let mut rendering = camera
                    .single_mut()
                    .expect("Error querying Main Menu Camera");

                rendering.is_active = false;
            }

            None => {}
        }
    }

    fn set_resource(mut commands: Commands) {
        commands.insert_resource(MainMenuReady);
    }
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (Self::render_main_menu, Self::set_resource).chain(),
        );
        app.add_systems(Update, actions::listen_ui_input);

        app.add_systems(
            OnEnter(AppState::MainMenu),
            Self::show_menu.run_if(resource_exists::<MainMenuReady>),
        );
        app.add_systems(
            OnExit(AppState::MainMenu),
            Self::hide_menu.run_if(resource_exists::<MainMenuReady>),
        );
    }
}
