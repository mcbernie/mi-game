use bevy::prelude::*;
use iyes_progress::prelude::AssetsLoading;

#[derive(Resource)]
pub struct MyUiAssets {
    ui_font: Handle<Font>,
    logo: Handle<Image>,
}

pub fn load_ui_assets(
    mut commands: Commands,
    ass: Res<AssetServer>,
    // we need to add our handles here, to track their loading progress:
    mut loading: ResMut<AssetsLoading>,
) {
    let ui_font = ass.load("quicksand-light.ttf");
    let logo = ass.load("logo_1.png");
    // etc ...

    // don't forget to add them so they can be tracked:
    loading.add(&ui_font);
    loading.add(&logo);

    commands.insert_resource(MyUiAssets { ui_font, logo });
}

pub mod splash;