use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::ui::FocusPolicy;

const MY_UI_RENDER_LAYER: [usize; 1] = [1];

#[derive(Component, Clone)]
pub enum UiLabelType {
    Play,
    Quit,
}

#[derive(Component)]
pub struct MainMenu;

pub fn show_main_menu(mut commands: Commands) {
    //spawn a UI Camera

    let camera_config = Camera {
        hdr: true,
        is_active: true,
        order: 3,
        ..Default::default()
    };

    // add UiPickingCamera component to enable mouse events
    commands.spawn((
        camera_config,
        Camera2d,
        UiPickingCamera,
        RenderLayers::from_layers(&MY_UI_RENDER_LAYER),
    ));

    // create and format a base node to use for all the UI entities, and then we only need to change some parts
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

    // define a main menu Node and some Compnents
    let mut main_menu_bundle = (
        base_node.clone(),
        FocusPolicy::Pass,
        BackgroundColor(Color::Srgba(Srgba::hex("171717").unwrap())),
        BorderColor(Color::WHITE),
        BorderRadius::all(Val::Px(5.)),
        Visibility::Visible,
        MainMenu,
        RenderLayers::from_layers(&MY_UI_RENDER_LAYER),
    );

    main_menu_bundle.0.justify_self = JustifySelf::Stretch;
    main_menu_bundle.0.align_content = AlignContent::Center;
    main_menu_bundle.0.justify_content = JustifyContent::Center;

    // Node Bundle for title
    let title_text_bundle = (
        Text::new("Absent Chroma"),
        TextFont {
            font_size: 80.,
            ..default()
        },
        BorderColor(Color::Srgba(Srgba::hex("0000ff").unwrap())),
        base_node.clone(),
        Visibility::Inherited,
    );

    // The Play button
    let play_text_bundle = (
        Text::new("Play"),
        TextFont {
            font_size: 42.,
            ..default()
        },
        TextColor(Color::Srgba(Srgba::hex("00ff00").unwrap())),
        UiLabelType::Play,
        Button,
        BorderColor(Color::Srgba(Srgba::hex("ff00ff").unwrap())),
        base_node.clone(),
        Visibility::Inherited,
    );

    // Quit button
    let mut quit_text_bundle = play_text_bundle.clone();
    quit_text_bundle.0 = Text::new("Quit");
    quit_text_bundle.3 = UiLabelType::Quit;

    // spawn the main node
    let main_node_id = commands.spawn(main_menu_bundle).id();

    // spawn all other entities as children of the main node
    commands.entity(main_node_id).with_children(|parent| {
        parent.spawn(title_text_bundle);
        parent.spawn(play_text_bundle);
        parent.spawn(quit_text_bundle);
    });
}

pub fn listen_ui_input(
    mut query: Query<(&Interaction, &UiLabelType), Changed<Interaction>>,
    mut event_writer: EventWriter<AppExit>,
    mut menu: Query<&mut Visibility, With<MainMenu>>,
) {
    // write logic to handle each combinations of the query
    for (interaction, label_type) in query.iter_mut() {
        if interaction == &Interaction::Pressed {
            match label_type {
                UiLabelType::Quit => {
                    // If quit button pressed, exit the app
                    event_writer.write(AppExit::Success);
                }

                UiLabelType::Play => {
                    // if play button pressed, hide the menu
                    let mut visibility = menu.single_mut().expect("Couldn't query the Main Menu.");
                    visibility.toggle_visible_hidden();
                }
            }
        }
    }
}
