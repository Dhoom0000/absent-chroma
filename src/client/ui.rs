use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::ui::{UiPlugin, prelude::*};
use bincode::de;

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
            y: OverflowAxis::Clip,
        },
        overflow_clip_margin: OverflowClipMargin {
            visual_box: OverflowClipBox::ContentBox,
            margin: 10.,
        },
        width: Val::Vw(90.),
        height: Val::Vh(90.),
        aspect_ratio: Some(2560. / 1440.),
        align_items: AlignItems::Center,
        justify_items: JustifyItems::Center,
        align_self: AlignSelf::Center,
        justify_self: JustifySelf::Center,
        align_content: AlignContent::Center,
        justify_content: JustifyContent::Center,
        margin: UiRect {
            left: Val::Px(10.),
            right: Val::Px(10.),
            top: Val::Px(10.),
            bottom: Val::Px(10.),
        },
        padding: UiRect {
            left: Val::Px(5.),
            right: Val::Px(5.),
            top: Val::Px(5.),
            bottom: Val::Px(5.),
        },
        flex_direction: FlexDirection::Column,
        flex_wrap: FlexWrap::Wrap,
        ..Default::default()
    };

    let title_label = Text::new("Absent Chroma");

    let main_node_id = commands.spawn((main_node,)).id();

    commands.entity(main_node_id).with_children(|parent| {
        parent.spawn(title_label);
    });
}
