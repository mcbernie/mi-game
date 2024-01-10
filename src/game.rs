use bevy::{prelude::*, core_pipeline::{tonemapping::Tonemapping, Skybox}, render::render_resource::{TextureViewDimension, TextureViewDescriptor}, asset::LoadState, gltf::Gltf, audio::Source};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use iyes_progress::prelude::AssetsLoading;

use crate::{camera::{ThirdPersonCameraPlugin, ThirdPersonCamera}, player::PlayerPlugin, AppState, Cubemap, map, despawn_screen, game, ui::splash::{splash_setup, OnSplashScreen}};

pub struct GamePlugin;

#[derive(Resource)]
pub struct GameResources {
    pub map: Handle<Scene>,
    pub player: Handle<Gltf>,
    pub player_model: Handle<Scene>,
    pub skybox: Handle<Image>,
    //pub sound_foot_1: Handle<Source>,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_plugins(WorldInspectorPlugin::new())
            .add_plugins(ThirdPersonCameraPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::default(),
                //RapierDebugRenderPlugin::default(),
            ))
            //.add_systems(OnEnter(AppState::GameLoading), setup)
            .add_systems(
                OnEnter(AppState::GameLoading), 
                (
                    splash_setup,
                    setup,
                    map::setup, 
                    despawn_screen::<Camera2d>
                )
            )
            .add_systems(OnExit(AppState::GameLoading), despawn_screen::<OnSplashScreen>)
            .add_systems(Update, skybox_asset_loaded.run_if(in_state(AppState::GameLoading)));
    }
}

pub fn load_game_assets(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {

    let map: Handle<Scene> = ass.load("de_dust2.glb#Scene0");
    let player: Handle<Gltf> = ass.load("my_character.glb");
    let player_model: Handle<Scene> = ass.load("my_character.glb#Scene0");
    let skybox: Handle<Image> = ass.load("textures/Ryfjallet_cubemap.png");

    loading.add(&map);
    loading.add(&player);
    loading.add(&player_model);
    loading.add(&skybox);
    
    commands.insert_resource(GameResources {
        map, player, player_model, skybox
    });

}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameResources>,
) {

    println!("enter game state: loading");
    commands.spawn((DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_3)),
        ..default()
    }, Name::new("GlobalLight")));

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-10.0, 2.5, 0.0),
            tonemapping: Tonemapping::AcesFitted,
            /*projection: Projection::Perspective(PerspectiveProjection { 
             near:-1.0001,
             far: 1000.0,
             ..Default::default()
            }),*/
            ..default()
        },
        //camera::CameraController::default(),
        //Skybox(skybox_handle.clone()),
        ThirdPersonCamera::default(),
    ));

    commands.insert_resource(Cubemap {
        is_loaded: false,
        index: 0,
        image_handle: game_assets.skybox.clone(),
    });

}

fn skybox_asset_loaded(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle) == LoadState::Loaded {
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.0 = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}