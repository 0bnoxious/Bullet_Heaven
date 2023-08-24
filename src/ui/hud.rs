use bevy::prelude::*;
use kayak_ui::{
    prelude::{widgets::*, *},
    CameraUIKayak,
};

use crate::global::GameState;

#[derive(Component, Clone)]
pub struct HudWidgetProps {
    game_state: GameState,
    game_timer: i32,
    player_hp: i32,
}

impl Default for HudWidgetProps {
    fn default() -> Self {
        Self {
            game_state: GameState::Playing,
            game_timer: 0,
            player_hp: 0,
        }
    }
}

// Our own version of widget_update that handles resource change events.
pub fn widget_update_with_resource<
    Props: PartialEq + Component + Clone,
    State: PartialEq + Component + Clone,
>(
    In((entity, previous_entity)): In<(Entity, Entity)>,
    widget_context: Res<KayakWidgetContext>,
    time_resource: Res<Time>,
    widget_param: WidgetParam<Props, State>,
) -> bool {
    widget_param.has_changed(&widget_context, entity, previous_entity) || time_resource.is_changed()
}

pub fn setup_hud(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
    time_res: Res<Time>,
) {
    let camera_entity = commands
        .spawn(Camera2dBundle::default())
        .insert(CameraUIKayak)
        .id();

    font_mapping.set_default(asset_server.load("roboto.kayak_font"));
    // font_mapping.force_subpixel(&asset_server.load("roboto.kayak_font"));

    let mut widget_context = KayakRootContext::new(camera_entity);
    widget_context.add_plugin(KayakWidgetsContextPlugin);
    let parent_id = None;
    rsx! {
        <KayakAppBundle>
            <TextWidgetBundle
                text={TextProps {
                    content: time_res.delta_seconds().to_string(),
                    size: 20.0,
                    alignment: Alignment::Middle,
                    ..Default::default()
                }}
            />
        </KayakAppBundle>
    };

    commands.spawn((widget_context, EventDispatcher::default()));
}
