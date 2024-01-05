use std::time::Duration;

use bevy::prelude::*;
use bevy_tnua::{TnuaAnimatingState, controller::TnuaController, builtins::{TnuaBuiltinJump, TnuaBuiltinJumpState, TnuaBuiltinCrouch, TnuaBuiltinDash, TnuaBuiltinWalk}, TnuaAction, TnuaAnimatingStateDirective};

use super::ani_patcher::AnimationsHandler;

#[derive(Debug)]
pub enum AnimationState {
    Standing,
    Running(f32),
    Jumping,
    Falling,
    Crouching,
    Crawling(f32),
    Dashing,
}

#[derive(Component)]
struct Bla;

pub fn animate(
    mut animations_handlers_query: Query<(
        &mut TnuaAnimatingState<AnimationState>,
        &TnuaController,
        &AnimationsHandler,
    )>,
    mut animation_players_query: Query<&mut AnimationPlayer>,
) {
    for (mut animating_state, controller, handler) in animations_handlers_query.iter_mut() {
        let Ok(mut player) = animation_players_query.get_mut(handler.player_entity) else {
            continue;
        };
        match animating_state.update_by_discriminant({
            match controller.action_name() {
                Some(TnuaBuiltinJump::NAME) => {
                    let (_, jump_state) = controller
                        .concrete_action::<TnuaBuiltinJump>()
                        .expect("action name mismatch");
                    match jump_state {
                        TnuaBuiltinJumpState::NoJump => continue,
                        TnuaBuiltinJumpState::StartingJump { .. } => AnimationState::Jumping,
                        TnuaBuiltinJumpState::SlowDownTooFastSlopeJump { .. } => {
                            AnimationState::Jumping
                        }
                        TnuaBuiltinJumpState::MaintainingJump => AnimationState::Jumping,
                        TnuaBuiltinJumpState::StoppedMaintainingJump => AnimationState::Jumping,
                        TnuaBuiltinJumpState::FallSection => AnimationState::Falling,
                    }
                }
                Some(TnuaBuiltinCrouch::NAME) => {
                    let Some((_, basis_state)) = controller.concrete_basis::<TnuaBuiltinWalk>()
                    else {
                        continue;
                    };
                    let speed =
                        Some(basis_state.running_velocity.length()).filter(|speed| 0.01 < *speed);
                    let is_crouching = basis_state.standing_offset < -0.4;
                    match (speed, is_crouching) {
                        (None, false) => AnimationState::Standing,
                        (None, true) => AnimationState::Crouching,
                        (Some(speed), false) => AnimationState::Running(0.1 * speed),
                        (Some(speed), true) => AnimationState::Crawling(0.1 * speed),
                    }
                }
                Some(TnuaBuiltinDash::NAME) => AnimationState::Dashing,
                Some(other) => panic!("Unknown action {other}"),
                None => {
                    let Some((_, basis_state)) = controller.concrete_basis::<TnuaBuiltinWalk>()
                    else {
                        continue;
                    };
                    if basis_state.standing_on_entity().is_none() {
                        AnimationState::Falling
                    } else {
                        let speed = basis_state.running_velocity.length();
                        if 0.01 < speed {
                            AnimationState::Running(0.4 * speed)
                        } else {
                            AnimationState::Standing
                        }
                    }
                }
            }
        }) {
            TnuaAnimatingStateDirective::Maintain { state } => match state {
                AnimationState::Running(speed) | AnimationState::Crawling(speed) => {
                    player.set_speed(*speed);
                }
                AnimationState::Jumping | AnimationState::Dashing => {
                    if controller.action_flow_status().just_starting().is_some() {
                        player.seek_to(0.0);
                    }
                }
                _ => {}
            },
            TnuaAnimatingStateDirective::Alter {
                old_state: _,
                state,
            } => match state {
                AnimationState::Standing => {

                    //player.start(handler.animations["idle"].clone_weak())
                    //.set_speed(1.0);
                    player
                        .start_with_transition(handler.animations["idle"].clone_weak(), Duration::from_millis(800))
                        .set_speed(1.0)
                        .repeat();
                }
                AnimationState::Running(speed) => {
                    player
                        .start(handler.animations["walking"].clone_weak())
                        //.set_speed(1000.0 * (*speed))
                        .set_speed(*speed)
                        .repeat();
                }
                AnimationState::Jumping => {
                    player
                        .start(handler.animations["walking"].clone_weak())
                        .set_speed(2.0);
                }
                AnimationState::Falling => {
                    player
                        .start(handler.animations["walking"].clone_weak())
                        .set_speed(1.0);
                }
                AnimationState::Crouching => {
                    player
                        .start(handler.animations["crouch"].clone_weak())
                        .set_speed(1.0)
                        .repeat();
                }
                AnimationState::Crawling(speed) => {
                    player
                        .start(handler.animations["Crawling"].clone_weak())
                        .set_speed(*speed)
                        .repeat();
                }
                AnimationState::Dashing => {
                    player
                        .start(handler.animations["Dashing"].clone_weak())
                        .set_speed(10.0);
                }
            },
        }
    }
}