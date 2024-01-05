
use bevy::{
    prelude::*, utils::HashMap, render::camera, core_pipeline::tonemapping::Tonemapping, input::mouse::MouseMotion
};
//use bevy_tnua_rapier3d::*;
use bevy_tnua::{prelude::*, TnuaProximitySensor, TnuaAnimatingState};
use bevy_rapier3d::prelude::*;
use bevy_tnua_rapier3d::{TnuaRapier3dIOBundle, TnuaRapier3dPlugin, TnuaRapier3dSensorShape};
use std::f32::consts::{FRAC_2_PI, PI};

use crate::{MainCamera, camera::ThirdPersonCameraTarget};

use self::ani_patcher::GltfSceneHandler;

mod ani_patcher;
mod animations;

#[derive(Resource)]
pub struct PlayerStats {
    pub health: f32,
    pub max_health: f32,
    pub stamina: f32,
    pub max_stamina: f32,
    pub mana: f32,
    pub max_mana: f32,
}

#[derive(Component)]
pub struct MainPlayer;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                TnuaRapier3dPlugin,
                TnuaControllerPlugin,
            ))
            .add_systems(Startup, (
                setup_player,
            ))
            .add_systems(Update, apply_controls.in_set(TnuaUserControlsSystemSet))
            .add_systems(Update, (
                fix_character_rotation,
                ani_patcher::animation_patcher_system,
                animations::animate,
            ));
            //.add_systems(Update, camera_always_follow_player);
            
    }
}

fn fix_character_rotation(
    mut query: Query<(Entity, &mut Transform, &Name), Added<Name>>
) {

    for (_, mut t, name) in query.iter_mut() {
        if name.as_str() == "Armature" {
            println!("only called once...");
            t.rotate_y(PI);
        }
    }
}


fn setup_player(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {

    let mut cmd = commands.spawn(Name::new("Player1"));
    cmd.insert(SceneBundle {
        scene: asset_server.load("my_character.glb#Scene0"),
        transform: Transform::from_xyz(6.0, 2012.6, 12.0),
        ..Default::default()
    });
    cmd.insert(GltfSceneHandler {
        names_from: asset_server.load("my_character.glb"),
    });
    cmd.insert(Collider::capsule(Vec3::new(0.0,0.4,0.0), Vec3::new(0.0, 1.6, 0.0), 0.3));
    //cmd.insert(TnuaRapier3dSensorShape(Collider::capsule(Vec3::new(0.0,0.0,0.0), Vec3::new(0.0, 1.6, 0.0), 0.3)));
    cmd.insert(RigidBody::Dynamic);
    cmd.insert(TnuaRapier3dIOBundle::default());
    cmd.insert(TnuaControllerBundle::default());
    cmd.insert(TnuaAnimatingState::<animations::AnimationState>::default());
    cmd.insert(MainPlayer);
    cmd.insert(ThirdPersonCameraTarget);
}

#[allow(clippy::type_complexity)]
fn apply_controls(
    //mut egui_context: EguiContexts,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(
        //&CharacterMotionConfigForPlatformerExample,
        &mut TnuaController,
        //&mut TnuaCrouchEnforcer,
        &mut TnuaProximitySensor,
        //&TnuaGhostSensor,
        //&mut TnuaSimpleFallThroughPlatformsHelper,
        //&FallingThroughControlScheme,
        //&mut TnuaSimpleAirActionsCounter,
    )>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<MainPlayer>)>,
) {


    //let jump = keyboard.pressed(KeyCode::Space);
    //let dash = keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

    let turn_in_place = keyboard.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]);

    //let crouch_buttons = [KeyCode::ControlLeft, KeyCode::ControlRight];
    //let crouch = keyboard.any_pressed(crouch_buttons);
    //let crouch_just_pressed = keyboard.any_just_pressed(crouch_buttons);

    for (
        //config,
        mut controller,
        //mut crouch_enforcer,
        mut sensor,
        /*ghost_sensor,
        mut fall_through_helper,
        falling_through_control_scheme,
        mut air_actions_counter,*/
    ) in query.iter_mut()
    {

        let cam = match cam_q.get_single() {
            Ok(cam) => cam,
            Err(e) => Err(format!("Error retriving camera: {}", e)).unwrap(),
        };
        /*air_actions_counter.update(controller.as_mut());

        let crouch = falling_through_control_scheme.perform_and_check_if_still_crouching(
            crouch,
            crouch_just_pressed,
            fall_through_helper.as_mut(),
            sensor.as_mut(),
            ghost_sensor,
            1.0,
        );*/
        let mut direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::W) {
            direction += cam.forward();
        }
        if keyboard.pressed(KeyCode::S) {
            direction += cam.back();
        }
        if keyboard.pressed(KeyCode::A) {
            direction += cam.left();
        }
        if keyboard.pressed(KeyCode::D) {
            direction += cam.right();
        }

        direction.y = 0.0;
        direction = direction.clamp_length_max(1.0);

        let speed_factor = 3.0;

        controller.basis(TnuaBuiltinWalk {
            desired_velocity: direction * speed_factor,
            desired_forward: direction,
            //desired_forward: Vec3::Y,
            //spring_strengh: 1000.0,
            float_height: 0.027,
            ..Default::default()
        });

        /*if crouch {
            controller.action(crouch_enforcer.enforcing(config.crouch.clone()));
        }

        if jump {
            controller.action(TnuaBuiltinJump {
                allow_in_air: air_actions_counter.air_count_for(TnuaBuiltinJump::NAME)
                    <= config.actions_in_air,
                ..config.jump.clone()
            });
        }

        if dash {
            controller.action(TnuaBuiltinDash {
                displacement: direction.normalize() * config.dash_distance,
                desired_forward: direction.normalize(),
                allow_in_air: air_actions_counter.air_count_for(TnuaBuiltinDash::NAME)
                    <= config.actions_in_air,
                ..config.dash.clone()
            });
        }*/

        // turn player by mouse position

    }
}
