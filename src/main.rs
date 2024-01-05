use std::time::Duration;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy::{
    animation::RepeatAnimation,
    pbr::CascadeShadowConfigBuilder,
    core_pipeline::tonemapping::Tonemapping,
    asset::LoadState,
    core_pipeline::Skybox,
    prelude::*,
    render::{
        render_resource::{TextureViewDescriptor, TextureViewDimension},
        renderer::RenderDevice,
        texture::CompressedImageFormats,
    },
};

use bevy_rapier3d::prelude::*;
use camera::ThirdPersonCameraPlugin;
use player::PlayerPlugin;

use crate::camera::ThirdPersonCamera;

mod map;
mod camera;
mod player;

const CUBEMAPS: &[(&str, CompressedImageFormats)] = &[
    (
        "textures/Ryfjallet_cubemap.png",
        CompressedImageFormats::NONE,
    ),
    (
        "textures/Ryfjallet_cubemap_astc4x4.ktx2",
        CompressedImageFormats::ASTC_LDR,
    ),
    (
        "textures/Ryfjallet_cubemap_bc7.ktx2",
        CompressedImageFormats::BC,
    ),
    (
        "textures/Ryfjallet_cubemap_etc2.ktx2",
        CompressedImageFormats::ETC2,
    ),
];

#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    index: usize,
    image_handle: Handle<Image>,
}

#[derive(Component)]
pub struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ThirdPersonCameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            //RapierDebugRenderPlugin::default(),
        ))
        .add_systems(Startup, (setup, map::setup))
        .add_systems(Update, (
            asset_loaded,
            //setup_scene_once_loaded, 
            //keyboard_animation_control, 
            //set_head_size, 
            //get_current_position_of_root,
            /*camera::camera_controller,*/
        ))
        .run();
}

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct HasRoot {
    root: Entity,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    /*commands.insert_resource(Animations(vec![
        asset_server.load("character-digger.glb#Animation0"),
        asset_server.load("character-digger.glb#Animation1"),
        asset_server.load("character-digger.glb#Animation2"),
        asset_server.load("character-digger.glb#Animation3"),
        asset_server.load("character-digger.glb#Animation4"),
        asset_server.load("character-digger.glb#Animation5"),
    ]));*/

    // try loading a model
    /*commands.spawn((SceneBundle {
        scene: asset_server.load("character-digger.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    }, Player));*/
    // cube
    //commands.spawn(PbrBundle {
    //    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //    material: materials.add(Color::rgb_u8(124, 144, 255).into()),
    //    transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //    ..default()
    //});
    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 12500.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_3)),
        ..default()
    });

    let skybox_handle = asset_server.load(CUBEMAPS[0].0);

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            tonemapping: Tonemapping::AcesFitted,
            /*projection: Projection::Perspective(PerspectiveProjection { 
             near:0.0,
             far: 1000.0,
             ..Default::default()
            }),*/
            ..default()
        },
        //camera::CameraController::default(),
        Skybox(skybox_handle.clone()),
        ThirdPersonCamera::default(),
    ));

    commands.insert_resource(Cubemap {
        is_loaded: false,
        index: 0,
        image_handle: skybox_handle,
    });

}

// Once the scene is loaded, start the animation
fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.0[0].clone_weak()).repeat();
    }

    // move back until we dedect the root / parent Mesh (player)

}

fn asset_loaded(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle) == LoadState::Loaded {
        info!("Swapping to {}...", CUBEMAPS[cubemap.index].0);
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

fn set_head_size(
    mut commands: Commands,
    enemies: Query<(Entity, &Player, Without<HasRoot>)>,
    mut bones: Query<(&Name, )>,
    children: Query<&Children>,
) {
    fn recurse(
        main_id: Entity,
        commands: &mut Commands,
        id: Entity,
        enemy: &Player,
        bones: &mut Query<(&Name,)>,
        childrens: &Query<&Children>,
    ) -> bool {
        if let Ok((name, )) = bones.get_mut(id) {
            println!("found bone: {:?}", name);
            // safe id to faster access head
            if *name == Name::new("root") {
                //tf.scale = Vec3::ONE * 2.2;

                println!("ID: {:?} is root", id);
                commands.entity(main_id).insert(HasRoot { root: id });
                return false;
            }
        }
        if let Ok(children) = childrens.get(id) {
            for child in children {
                //println!("child: {:?}", child)
                if !recurse(main_id, commands, *child, enemy, bones, childrens) {
                    return false
                }
            }
        }
        true
    }

    for (id, enemy,()) in enemies.iter() {
        println!("player and some other entity: {:?}", id);
        if !recurse(id, &mut commands, id, enemy, &mut bones, &children) {
            break;
        }
    }
}

fn get_current_position_of_root(
    mut player: Query<(Entity, &Player, &HasRoot, &mut Transform, &Children)>,
    sub_childs: Query<(Entity, &Children)>,
    animation_players: Query<&AnimationPlayer>,
    root_bone: Query<(Entity, &Transform, Without<HasRoot>)>,
)  {
    for (id, _, has_root, mut c_transform, childs) in player.iter_mut() {
        
        let f = childs[0];
        let sub_s = sub_childs.get(f).unwrap();
        let a = sub_s.1[0];
        let animations = animation_players.get(a).unwrap();

        //println!("animation: {:?}", animations.is_paused());
        let (bone, transform, ()) = root_bone.get(has_root.root).unwrap();

        c_transform.translation = transform.translation;
        //println!("player current transform from Root: {:?}", transform);
    }
}

fn keyboard_animation_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    anisave: Res<Assets<AnimationClip>>,
    mut current_animation: Local<usize>,
) {
    for mut player in &mut animation_players {
        if keyboard_input.just_pressed(KeyCode::Space) {
            if player.is_paused() {
                player.resume();
            } else {
                player.pause();
            }
        }

        if keyboard_input.just_pressed(KeyCode::Up) {
            let speed = player.speed();
            player.set_speed(speed * 1.2);
        }

        if keyboard_input.just_pressed(KeyCode::Down) {
            let speed = player.speed();
            player.set_speed(speed * 0.8);
        }

        if keyboard_input.just_pressed(KeyCode::Left) {
            let elapsed = player.seek_time();
            player.seek_to(elapsed - 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::Right) {
            let elapsed = player.seek_time();
            player.seek_to(elapsed + 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::Return) {
            *current_animation = (*current_animation + 1) % animations.0.len();

            let ani = animations.0[*current_animation].clone_weak();
            if let Some(animation) = anisave.get(&ani) {
                dbg!(animation);
            }

            player
                .play_with_transition(
                    animations.0[*current_animation].clone_weak(),
                    Duration::from_millis(250),
                )
                .repeat();
        }

        if keyboard_input.just_pressed(KeyCode::Key1) {
            player.set_repeat(RepeatAnimation::Count(1));
            player.replay();
        }

        if keyboard_input.just_pressed(KeyCode::Key3) {
            player.set_repeat(RepeatAnimation::Count(3));
            player.replay();
        }

        if keyboard_input.just_pressed(KeyCode::Key5) {
            player.set_repeat(RepeatAnimation::Count(5));
            player.replay();
        }

        if keyboard_input.just_pressed(KeyCode::L) {
            player.set_repeat(RepeatAnimation::Forever);
        }
    }
}