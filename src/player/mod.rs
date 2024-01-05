
use bevy::{
    prelude::*, utils::HashMap, render::camera
};
//use bevy_tnua_rapier3d::*;
use bevy_tnua::{prelude::*, TnuaProximitySensor, TnuaAnimatingState};
use bevy_rapier3d::prelude::*;
use bevy_tnua_rapier3d::{TnuaRapier3dIOBundle, TnuaRapier3dPlugin, TnuaRapier3dSensorShape};
use std::f32::consts::{FRAC_2_PI, PI};

use crate::MainCamera;

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
            ))
            .add_systems(Update, camera_always_follow_player);
            
    }
}

fn fix_character_rotation(
    mut query: Query<(Entity, &mut Transform, &Name), Added<Name>>
) {

    for (_, mut t, name) in query.iter_mut() {
        if name.as_str() == "character-digger" {
            println!("only called once...");
            t.rotation = Quat::from_rotation_y(PI);
        }
    }
}

fn camera_always_follow_player(
    player_query: Query<&Transform, With<MainPlayer>>,
    mut camera_query: Query<(&mut Transform, &MainCamera), Without<MainPlayer>>
) {

    let Ok(player) = player_query.get_single() else { return };
    let Ok((mut transform,_)) = camera_query.get_single_mut() else {return};

    transform.look_at(player.translation, Vec3::Y);
}


fn setup_player(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {

    let mut cmd = commands.spawn_empty();
    cmd.insert(SceneBundle {
        scene: asset_server.load("character-digger.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 3.0, 0.0),
        ..Default::default()
    });
    cmd.insert(GltfSceneHandler {
        names_from: asset_server.load("character-digger.glb"),
    });
    cmd.insert(Collider::capsule(Vec3::new(0.0,0.1,0.0), Vec3::new(0.0, 0.9, 0.0), 0.17));
    cmd.insert(TnuaRapier3dSensorShape(Collider::capsule(Vec3::new(0.0,0.2,0.0), Vec3::new(0.0, 0.9, 0.0), 0.17)));
    cmd.insert(RigidBody::Dynamic);
    cmd.insert(TnuaRapier3dIOBundle::default());
    cmd.insert(TnuaControllerBundle::default());
    cmd.insert(TnuaAnimatingState::<animations::AnimationState>::default());
    cmd.insert(MainPlayer);
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
) {

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::Up) {
        direction -= Vec3::Z;
    }
    if keyboard.pressed(KeyCode::Down) {
        direction += Vec3::Z;
    }
    if keyboard.pressed(KeyCode::Left) {
        direction -= Vec3::X;
    }
    if keyboard.pressed(KeyCode::Right) {
        direction += Vec3::X;
    }

    direction = direction.clamp_length_max(1.0);

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
        /*air_actions_counter.update(controller.as_mut());

        let crouch = falling_through_control_scheme.perform_and_check_if_still_crouching(
            crouch,
            crouch_just_pressed,
            fall_through_helper.as_mut(),
            sensor.as_mut(),
            ghost_sensor,
            1.0,
        );*/

        let speed_factor = 1.0;

        controller.basis(TnuaBuiltinWalk {
            desired_velocity: if turn_in_place {
                Vec3::ZERO
            } else {
                direction * speed_factor * 1.0
            },
            desired_forward: direction.normalize_or_zero(),
            float_height: 0.2,
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
    }
}

