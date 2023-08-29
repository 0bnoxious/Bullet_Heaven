use bevy::prelude::*;

use self::button::PreloadResource;

pub mod assets;
pub mod background;
pub mod button;

pub fn setup_game_menu(
    asset_server: Res<AssetServer>,
    mut preload_resource: ResMut<PreloadResource>,
) {
    let background_panel = asset_server.load("main_menu/background_panel.png");
    let tug_o_war_logo = asset_server.load("main_menu/tug_o_war.png");
    let player = asset_server.load("main_menu/player.png");
    let button = asset_server.load("main_menu/button.png");
    let button_hover = asset_server.load("main_menu/button_hover.png");

    preload_resource.images.extend(vec![
        background_panel,
        tug_o_war_logo,
        player,
        button,
        button_hover,
    ]);
}
