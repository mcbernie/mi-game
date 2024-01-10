
use bevy::{
    prelude::*, utils::HashMap, render::{camera, view::NoFrustumCulling, primitives::Aabb}, core_pipeline::tonemapping::Tonemapping, input::mouse::MouseMotion, transform::TransformSystem, animation::animation_player, math::Vec3A
};
//use bevy_tnua_rapier3d::*;
use bevy_tnua::{prelude::*, TnuaProximitySensor, TnuaAnimatingState};
use bevy_rapier3d::prelude::*;
use bevy_tnua_rapier3d::{TnuaRapier3dIOBundle, TnuaRapier3dPlugin, TnuaRapier3dSensorShape};
use std::f32::consts::{FRAC_2_PI, PI};

use crate::{MainCamera, camera::ThirdPersonCameraTarget, AppState, game::GameResources};

use self::ani_patcher::GltfSceneHandler;

mod ani_patcher;
mod animations;
mod oponent;

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

#[derive(Component, Default)]
pub struct Player {
    pub head: Option<Entity>,
}


#[derive(Component)]
pub struct Foot {
    pub left: bool,
    pub triggered: bool,
    pub body: Entity,
}

#[derive(Component)]
pub struct Head;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                TnuaRapier3dPlugin,
                TnuaControllerPlugin,
            ))
            .add_systems(OnEnter(AppState::GameLoading), (
                setup_player,
            ))
            .add_systems(Update, (apply_controls.in_set(TnuaUserControlsSystemSet)).run_if(in_state(AppState::InGame)))
            .add_systems(Update, (
                fix_character_rotation,
                ani_patcher::animation_patcher_system,
                animations::animate,
            ).run_if(in_state(AppState::InGame)))
            .add_systems(PostUpdate, (
                    (rotate_head_to_camera_translation, foot_steps).after(animation_player),

            ).run_if(in_state(AppState::InGame)));
            //.add_systems(Update, camera_always_follow_player);
            
    }
}

fn fix_character_rotation(
    mut player_query: Query<(Entity, &Children, &Name, &mut Player), Added<Player>>,
    player_child_query_first: Query<(Entity, &Children)>,
    mut amature_query: Query<(Entity, &mut Transform, &Name, &Children)>,
    child_query: Query<(Entity, &Name, &Children)>,
    custom_child_query: Query<(&Name, Option<&Children>)>,
    mut fix_aabb_query: Query<&mut Aabb>,
    mut command: Commands,
) {

    fn recurse(
        what_to_find: String,
        main_id: Entity,
        id: Entity,
        bones: &Query<(&Name, Option<&Children>)>,
    ) -> Option<Entity> {
        if let Ok((name, childs, )) = bones.get(id) {
            if *name == Name::new(what_to_find.clone()) {
                return Some(id);
            }
            if let Some(childs) = childs {
                for child in childs {
                    if let Some(e) = recurse(what_to_find.clone(), main_id, *child, bones) {
                        return Some(e)
                    }
                }
            }
        }
        None
    }

    // query all players and try to get children
    for (player_body_entity, player_childs, player_name, mut player) in player_query.iter_mut() {

        // got one Child without Name..
        for player_first_child in player_childs {
            let Ok((_, childs_in_sub)) = player_child_query_first.get(*player_first_child) else { continue };
            for player_child in childs_in_sub {

                if let Ok((_, mut t, name, childs)) = amature_query.get_mut(*player_child) {

                    if name.as_str() == "Armature" {
                        t.rotate_y(PI);
                        t.translation.y = -1.0;
                        for child in childs.iter() {
                            let (e, name, _) = child_query.get(*child).unwrap();
                            if name.as_str() == "mixamorig:Hips" {
                                if let Some(e) = recurse("mixamorig:Head".to_string(),e, e, &custom_child_query) {
                                    command.spawn(SpotLightBundle {
                                        transform: Transform::from_xyz(0.0, 0.0, 2.0)
                                            .looking_at(Vec3::new(0.0,0.5,10.0), Vec3::Y),
                                        spot_light: SpotLight {
                                            intensity: 2400.0, // lumens
                                            color: Color::WHITE,
                                            shadows_enabled: true,
                                            inner_angle: 0.1,
                                            outer_angle: 0.3,
                                            range: 400.0,
                                            ..default()
                                        },
                                        ..default()
                                    }).set_parent(e);
                                    command.entity(e).insert(Head);
                                    player.head = Some(e);
                                }
                                if let Some(e) = recurse("mixamorig:LeftFoot".to_string(), e, e, &custom_child_query) {
                                    command.entity(e).insert(Foot{left: true, triggered: false, body: player_body_entity });
                                }
                                if let Some(e) = recurse("mixamorig:RightFoot".to_string(), e, e, &custom_child_query) {
                                    command.entity(e).insert(Foot{left: false, triggered: false, body: player_body_entity});
                                }
                            }
                            if name.as_str() == "Soldier_body" {
                                if let Some(e) = recurse("Soldier_body.001".to_string(), e, e, &custom_child_query) {
                                    if let Ok(mut aabb) = fix_aabb_query.get_mut(e) {
                                        aabb.center = Vec3A::new(0.0, 4.0, -80.0);
                                        aabb.half_extents = Vec3A::new(38.0, 46.0, 83.0);
                                    }
                                }
                            }
                            if name.as_str() == "Soldier_head" {
                                if let Some(e) = recurse("Soldier_head.001.0".to_string(), e, e, &custom_child_query) {
                                    if let Ok(mut aabb) = fix_aabb_query.get_mut(e) {
                                        aabb.center = Vec3A::new(0.0, -3.0, -166.0);
                                        aabb.half_extents = Vec3A::new(5.0, 5.0, 5.0);
                                        //aabb.half_extents = Vec3A::new(12.0, 9.0, 10.0);
                                    }
                                }
                                if let Some(e) = recurse("Soldier_head.001.1".to_string(), e, e, &custom_child_query) {
                                    if let Ok(mut aabb) = fix_aabb_query.get_mut(e) {
                                        //aabb.center = Vec3A::new(0.0, 3.0, -166.0);
                                        //aabb.half_extents = Vec3A::new(20.0, 21.0, 20.0);
                                        aabb.center = Vec3A::new(0.0, -3.0, -166.0);
                                        aabb.half_extents = Vec3A::new(5.0, 5.0,5.0);
                                        //aabb.half_extents = Vec3A::new(12.0, 9.0, 10.0);
                                    }
                                }

                            }
                        }
                    }
                }
            }
        }

    }
}


fn setup_player(
    mut commands: Commands, 
    game_assets: Res<GameResources>,
) {

    let mut cmd = commands.spawn(Name::new("Player1"));
    cmd.insert(SceneBundle {
        scene: game_assets.player_model.clone(),
        transform: Transform::from_xyz(6.0, 2020.0, 12.0),
        ..Default::default()
    });
    cmd.insert(GltfSceneHandler {
        names_from: game_assets.player.clone(),
    });
    cmd.insert(Collider::capsule_y(0.3, 0.4));
    cmd.insert(TnuaRapier3dSensorShape(Collider::cylinder(
        0.0, 0.50,
    )));
    cmd.insert(SpatialListener::new(0.5));
    cmd.insert(RigidBody::Dynamic);
    cmd.insert(TnuaRapier3dIOBundle::default());
    cmd.insert(TnuaControllerBundle::default());
    cmd.insert(TnuaAnimatingState::<animations::AnimationState>::default());
    cmd.insert(MainPlayer);
    cmd.insert(Player::default());
    cmd.insert(ThirdPersonCameraTarget);


    commands.spawn((
        Name::new("Player2"),
        Player::default(),
        TransformBundle {
            local: Transform::from_xyz(4.0, 2014.0, 11.0), 
            ..Default::default()
        },
        VisibilityBundle {
            visibility: Visibility::Visible,
            ..Default::default()
        },
        NoFrustumCulling,
        game_assets.player_model.clone(),
        GltfSceneHandler {
            names_from: game_assets.player.clone(),
        },
        TnuaRapier3dSensorShape(Collider::cylinder(
            0.0, 0.50,
        )),
        Collider::capsule_y(0.49, 0.3),
        RigidBody::Dynamic,
        //GravityScale(2.5),
        //ColliderMassProperties::Density(2.0),
        //Sensor,
        //Velocity { linvel: Vec3::new(0.0, -0.5, 0.0), angvel: Vec3::new(0.0, 0.0, 0.0) },
        TnuaRapier3dIOBundle::default(),
        TnuaControllerBundle::default(),
        TnuaAnimatingState::<animations::AnimationState>::default(),
    ));
    //cmd.insert(Emitter::default());
}

#[allow(clippy::type_complexity)]
fn apply_controls(
    //mut egui_context: EguiContexts,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(
        &MainPlayer,
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
    mut head_query: Query<&mut Transform, Without<Camera3d>>, // get all heads
) {


    //let jump = keyboard.pressed(KeyCode::Space);
    //let dash = keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

    let turn_in_place = keyboard.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]);

    //let crouch_buttons = [KeyCode::ControlLeft, KeyCode::ControlRight];
    //let crouch = keyboard.any_pressed(crouch_buttons);
    //let crouch_just_pressed = keyboard.any_just_pressed(crouch_buttons);

    for (
        config,
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

        let speed_factor = if keyboard.pressed(KeyCode::ShiftLeft) {
            6.0
        } else {
            3.0
        };

        controller.basis(TnuaBuiltinWalk {
            desired_velocity: direction * speed_factor,
            desired_forward: direction,
            //desired_forward: Vec3::Y,
            spring_strengh: 2000.0,
            float_height: 1.0,
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

fn foot_steps(
    mut command: Commands,
    mut steps_query: Query<(Entity, &GlobalTransform, &mut Foot)>,
    query_root_body: Query<&GlobalTransform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    for (foot_entity, t, mut f) in steps_query.iter_mut() {
        let player = query_root_body.get(f.body).unwrap();
        let player_transform = player.compute_transform();

        let foot_transform = t.compute_transform();
        let foot_distance = player_transform.translation.distance(foot_transform.translation);
        if foot_distance > 0.88 {
            if f.triggered == false {
                command.entity(foot_entity).insert(
                AudioBundle {
                    source: asset_server.load("02-footstep.ogg"),
                    settings: PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Remove,
                        spatial: true,
                        ..default()
                    },
                    ..Default::default()
                });
                f.triggered = true;
            }
        } else {
            f.triggered = false;
            //println!("foot up");
        }
    }
}

fn rotate_head_to_camera_translation(
    query_a_player: Query<(&Player, &Transform), With<MainPlayer>>,
    cam_q: Query<&Transform, With<Camera3d>>,
    mut head_q: Query<&mut Transform, (With<Head>, Without<Camera3d>, Without<MainPlayer>)>,
) {
    let camera = match cam_q.get_single() {
        Ok(cam) => cam,
        Err(e) => Err(format!("Error retriving camera: {}", e)).unwrap(),
    };

    let (config, p_transform) = query_a_player.get_single().unwrap();
    if let Some(head) = config.head {
        let mut get_head = head_q.get_mut(head).unwrap(); 
        //get_head.rotation = camera.rotation;
        //get_head.look_at(camera.translation, Vec3::Y);
        let forward = Vec3::new(0.0, 0.0, -1.0);
        let camera_direction = camera.rotation.mul_vec3(forward);
        get_head.look_to(camera_direction, Vec3::Y);
        get_head.rotation = Quat{x:get_head.rotation.x, y:-get_head.rotation.y, z: get_head.rotation.z, w: -get_head.rotation.w};
    
        // Umwandeln in Euler-Winkel, Roll ignorieren und Einschränkungen anwenden
        let mut euler = get_head.rotation.to_euler(EulerRot::XYZ);
        //euler.2 = 0.0; // Roll auf Null setzen
        //println!("euler: {:?}", euler.1);
        //euler.0 = euler.0.clamp(-0.2, 0.2); // Pitch (X-Achse) einschränken (oben, unten)
        //euler.1 = euler.1.clamp(-0.2, 0.2); // Yaw (Y-Achse) einschränken (links, rechts)
        //euler.2 += 0.5;
        //euler.1 = -euler.1;
    
        // Zielrotation ohne Roll und mit eingeschränktem Pitch/Yaw
        let target_rotation = Quat::from_euler(EulerRot::XYZ, euler.0, euler.1, euler.2);
    
        // Slerp-Interpolation anwenden
        let t: f32 = 0.1; // Interpolationsfaktor
        //get_head.rotation = get_head.rotation.slerp(target_rotation, t.clamp(0.0, 1.0));
        get_head.rotation = target_rotation;
    
        // Anwenden der inversen Spielerrotation, um die Weltrotation zu berücksichtigen
        get_head.rotation = p_transform.rotation.inverse() * get_head.rotation;
        get_head.scale = Vec3::ONE * 2.2;
    }
}

// Funktion, um einen Punkt innerhalb eines kegelförmigen Bereichs zu beschränken
fn clamp_to_cone(origin: Vec3, target: Vec3, forward: Vec3, max_angle: f32) -> Vec3 {
    let direction_to_target = (target - origin).normalize();

    // Winkel zwischen dem Vorwärtsvektor und der Richtung zum Ziel
    let angle_to_target = forward.angle_between(direction_to_target);

    if angle_to_target > max_angle {
        // Berechnen eines neuen Zielpunktes auf dem Rand des Kegels
        let rotation_to_max_angle = Quat::from_axis_angle(forward.cross(direction_to_target).normalize(), max_angle);
        rotation_to_max_angle.mul_vec3(forward) * (target - origin).length() + origin
    } else {
        // Zielpunkt liegt bereits innerhalb des Kegels
        target
    }
}