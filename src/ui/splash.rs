
use bevy::prelude::*;

use crate::{AppState, despawn_screen, game::load_game_assets};

use super::load_ui_assets;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Splash), (load_ui_assets,splash_setup, load_game_assets))
            .add_systems(Update, (update_splash).run_if(in_state(AppState::Splash)))
            .add_systems(OnExit(AppState::Splash), despawn_screen::<OnSplashScreen>);
    } 
}

#[derive(Component)]
pub struct OnSplashScreen;

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let logo = asset_server.load("logo_1.png");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..Default::default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Px(300.0),
                        ..default()
                    },
                    image: UiImage::new(logo),
                    ..default()
                },
            ));
        });
}


fn update_splash(dt: Res<Time>, mut q: Query<&mut Transform, With<OnSplashScreen>>) {
    for mut t in q.iter_mut() {
        t.rotate(Quat::from_rotation_z(dt.delta_seconds() * 0.5));
    }
}