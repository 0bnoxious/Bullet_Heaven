use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "main_menu/background_panel.png")]
    pub background_panel: Handle<Image>,

    #[asset(path = "main_menu/tug_o_war.png")]
    pub tug_o_war_logo: Handle<Image>,

    #[asset(path = "main_menu/player.png")]
    pub player: Handle<Image>,

    #[asset(path = "main_menu/button.png")]
    pub button: Handle<Image>,

    #[asset(path = "main_menu/button_hover.png")]
    pub button_hover: Handle<Image>,
}
