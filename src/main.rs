
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use iyes_progress::prelude::*;
use ui::splash;


mod game;
mod map;
mod camera;
mod player;
mod ui;


#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    index: usize,
    image_handle: Handle<Image>,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, States)]
enum AppState {
    #[default]
    Splash,
    //MainMenu,
    GameLoading,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }
        ))
        .add_plugins(WorldInspectorPlugin::new())
        .add_state::<AppState>()
        .add_plugins(
            ProgressPlugin::new(AppState::Splash)
                .continue_to(AppState::GameLoading)
                .track_assets(),
        )
        .add_plugins(
            ProgressPlugin::new(AppState::GameLoading)
                .continue_to(AppState::InGame)
                .track_assets()
        )
        .add_plugins(splash::SplashPlugin)
        .add_plugins(game::GamePlugin)
        .add_systems(Startup, setup)
        //.insert_resource(GizmoConfig {
        //    aabb: AabbGizmoConfig {
        //        draw_all: true,
        //        default_color: Some(Color::DARK_GREEN),
        //    },
        //    ..Default::default()
        //})
        .run();
}

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

#[derive(Component)]
struct Player;


fn setup(
    mut commands: Commands,
) {
    commands.spawn(
        Camera2dBundle::default()
    );
}


// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
