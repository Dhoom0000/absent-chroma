use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::ui::{UiPlugin, prelude::*};

#[derive(Component, Clone)]
pub enum UiLabelType {
    Play,
    Quit,
}

pub fn show_main_menu(mut commands: Commands) {
    //spawn a UI Camera

    let camera_config = Camera {
        hdr: true,
        is_active: true,
        order: 0,
        ..Default::default()
    };

    commands.spawn((
        camera_config,
        Camera2d::default(),
        UiPickingCamera,
        RenderLayers::from_layers(&[0, 1]),
    ));

    let main_node = Node {
        display: Display::Flex,
        box_sizing: BoxSizing::BorderBox,
        position_type: PositionType::Relative,
        overflow: Overflow {
            x: OverflowAxis::Clip,
            y: OverflowAxis::Scroll,
        },
        overflow_clip_margin: OverflowClipMargin {
            visual_box: OverflowClipBox::ContentBox,
            margin: 25.,
        },
        left: Val::Px(0.),
        right: Val::Px(0.),
        top: Val::Px(0.),
        bottom: Val::Px(0.),
        width: Val::Vw(100.),
        height: Val::Vh(100.),
        min_width: Val::Vw(90.),
        min_height: Val::Vh(90.),
        max_width: Val::Vw(100.),
        max_height: Val::Vh(100.),
        aspect_ratio: Some(2560. / 1440.),
        align_items: AlignItems::FlexStart,
        justify_items: JustifyItems::Stretch,
        align_content: AlignContent::Center,
        justify_content: JustifyContent::SpaceAround,
        align_self: AlignSelf::Center,
        justify_self: JustifySelf::Center,
        margin: UiRect::all(Val::Px(10.)),
        padding: UiRect::all(Val::Px(5.)),
        border: UiRect::all(Val::Px(1.)),
        flex_direction: FlexDirection::Column,
        flex_wrap: FlexWrap::Wrap,
        ..Default::default()
    };

    let title_text_bundle = (
        Text::new("Absent Chroma"),
        TextFont {
            font_size: 80.,
            ..default()
        },
    );

    let play_text_bundle = (
        Text::new("Play"),
        TextFont {
            font_size: 40.,
            ..default()
        },
        TextColor(Color::Srgba(Srgba::hex("00ff00").unwrap())),
        UiLabelType::Play,
        Button,
    );

    let mut quit_text_bundle = play_text_bundle.clone();
    quit_text_bundle.0 = Text::new("Quit");
    quit_text_bundle.3 = UiLabelType::Quit;

    let main_node_id = commands
        .spawn((
            main_node,
            BackgroundColor(Color::Srgba(Srgba::hex("171717").unwrap())),
        ))
        .id();

    commands.entity(main_node_id).with_children(|parent| {
        parent.spawn(title_text_bundle);
        parent.spawn(play_text_bundle);
        parent.spawn(quit_text_bundle);
    });
}

pub fn listen_ui_input(
    mut query: Query<(&Interaction, &UiLabelType), Changed<Interaction>>,
    mut event_writer: EventWriter<AppExit>,
) {
    for (interaction, label_type) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => match label_type {
                UiLabelType::Quit => {
                    event_writer.write(AppExit::Success);
                }

                _ => {}
            },

            _ => {}
        }
    }
}
