use bevy::{
    prelude::*, 
    core_pipeline::{tonemapping::Tonemapping, Skybox}, 
    render::render_resource::{TextureViewDimension, TextureViewDescriptor}, 
    asset::LoadState, 
    gltf::Gltf, 
    utils::HashMap, 
    input::mouse::MouseMotion, 
    core::{Pod, Zeroable}
};

use bevy_rapier3d::prelude::*;
use bevy_tnua::controller::TnuaController;
use iyes_progress::{prelude::AssetsLoading, Progress, ProgressSystem};

use bevy_ggrs::*;
// the network thing
use bevy_matchbox::prelude::*;

use crate::{camera::{ThirdPersonCameraPlugin, ThirdPersonCamera}, player::{PlayerPlugin, Player, Head, self, MainPlayer}, AppState, Cubemap, map, despawn_screen, game, ui::splash::{splash_setup, OnSplashScreen, update_splash}};


#[repr(C)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable, Debug, Default, Reflect)]
pub struct PlayerState {
    pub head_rotation: Vec3,
    pub w: f32,
    pub input: u8,
    _padding: [u8; 3],
}


pub type Config = bevy_ggrs::GgrsConfig<PlayerState, PeerId>;

pub struct GamePlugin;

#[derive(Resource)]
pub struct GameResources {
    pub map: Handle<Scene>,
    pub player: Handle<Gltf>,
    pub player_model: Handle<Scene>,
    pub skybox: Handle<Image>,
    pub local_player_id: Option<PeerId>,
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
            .add_plugins(
                GgrsPlugin::<Config>::default(),
            )
            .rollback_component_with_clone::<Transform>()
            //.add_systems(OnEnter(AppState::GameLoading), setup)
            .add_systems(
                OnEnter(AppState::GameLoading), 
                (
                    splash_setup,
                    setup,
                    map::setup, 
                    despawn_screen::<Camera2d>,
                    start_matchbox_socket,
                )
            )
            .add_systems(OnExit(AppState::GameLoading), despawn_screen::<OnSplashScreen>)
            .add_systems(Update, (update_splash).run_if(in_state(AppState::GameLoading)))
            .add_systems(Update, skybox_asset_loaded.run_if(in_state(AppState::GameLoading)))
            .add_systems(Update, (wait_for_players.track_progress()).run_if(in_state(AppState::GameLoading)))
            .add_systems(ReadInputs, (read_local_inputs).run_if(in_state(AppState::InGame)));
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
        map, player, player_model, skybox,
        local_player_id: None,
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


fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://10.42.200.56:3536/hitomowaji?next=2";
    info!("connecting to {room_url}");
    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
}

fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut game_resources: ResMut<GameResources>,
) -> Progress {
    if socket.get_channel(0).is_err() {
        return false.into();
    }

    socket.update_peers();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return false.into();
    }

    info!("all peers have joined, going in-game");

    let mut session_builder = ggrs::SessionBuilder::<Config>::new()
        .with_num_players(num_players)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    let channel = socket.take_channel(0).unwrap();

    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::P2P(ggrs_session));

    game_resources.local_player_id = Some(socket.id().unwrap());
    return true.into();
}


pub const INPUT_UP: u8 = 1 << 0;
pub const INPUT_DOWN: u8 = 1 << 1;
pub const INPUT_LEFT: u8 = 1 << 2;
pub const INPUT_RIGHT: u8 = 1 << 3;
pub const INPUT_JUMP: u8 = 1 << 4;
pub const INPUT_RUN: u8 = 1 << 5;

fn read_local_inputs(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    local_players: Res<LocalPlayers>,

    player_query: Query<&Player, With<MainPlayer>>,
    head_query: Query<&Transform, With<Head>>,
) {
    
    
    
    
    let mut local_inputs = HashMap::new();
    
    for handle in &local_players.0 {


        let mut input = 0u8;

        if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
            input |= INPUT_UP;
        }
        if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
            input |= INPUT_DOWN;
        }
        if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
            input |= INPUT_LEFT
        }
        if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
            input |= INPUT_RIGHT;
        }
        if keys.any_pressed([KeyCode::Space, KeyCode::Return]) {
            input |= INPUT_JUMP;
        }
        if keys.any_pressed([KeyCode::ShiftLeft]) {
            input |= INPUT_RUN;
        }

        
        //let rotation = if let Ok(local_player) = player_query.get_single() {
        //    if local_player.handle == 200 {
        //        let Ok(head_transform) = head_query.get(local_player.head.unwrap()) else { return };
        //        Some(head_transform.rotation)
        //    } else { None }

        //} else {
        //    None
        //};

        //if let Some(rotation) = rotation {
        //    local_inputs.insert(*handle, 
        //        PlayerState {
        //            input,
        //            //head_rotation: rotation.xyz(),
        //            //w: rotation.w,
        //            ..default()
        //        }    
        //    );

        //} else {
            local_inputs.insert(*handle, 
                PlayerState {
                    input,
                    ..default()
                }    
            );
        //}

    }

    commands.insert_resource(LocalInputs::<Config>(local_inputs));
}