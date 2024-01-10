use bevy::{
    input::mouse::{MouseMotion, self},
    prelude::*, window::{PrimaryWindow, CursorGrabMode},
};

use std::f32::consts::PI;

use crate::{player::{self, MainPlayer}, AppState};

pub struct ThirdPersonCameraPlugin;

impl Plugin for ThirdPersonCameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Offset>()
        .register_type::<Zoom>()
        .register_type::<ThirdPersonCamera>()
        .register_type::<ThirdPersonCameraTarget>()
        .add_systems(Update, (
            orbit_mouse.run_if(orbit_condition),
            sync_player_camera.after(orbit_mouse),
            toggle_cursor,
        ).run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ThirdPersonCamera {
    pub offset_enabled: bool,
    pub offset: Offset,
    pub zoom_enabled: bool,
    pub zoom: Zoom,
    pub zoom_sensitivity: f32,
    pub focus: Vec3,
    pub mouse_sensitivity: f32,
    pub snap_mouse: bool,
}


impl Default for ThirdPersonCamera {
    fn default() -> Self {
        Self {
            offset_enabled: true,
            offset: Offset::new(0.5, 0.2),
            zoom_enabled: true,
            zoom: Zoom::new(2.0, 3.0),
            zoom_sensitivity: 1.0,
            focus: Vec3::new(0.0, 0.3, -0.1),
            mouse_sensitivity: 4.0,
            snap_mouse: false,
        }
    }
}


#[derive(Reflect)]
pub struct Offset {
    pub position: (f32, f32),
    position_copy: (f32, f32),
    is_in_transition: bool,
}

impl Offset {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: (x, y),
            position_copy: (x, y),
            is_in_transition: false,
        }
    }
}


/// Sets the zoom bounds (min & max)
#[derive(Reflect)]
pub struct Zoom {
    pub min: f32,
    pub max: f32,
    radius: f32,
    radius_copy: Option<f32>,
}

impl Zoom {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min,
            max,
            radius: (min + max) / 2.0,
            radius_copy: None,
        }
    }
}


#[derive(Component, Reflect)]
pub struct ThirdPersonCameraTarget;


fn sync_player_camera(
    player_query: Query<(&Transform, &MainPlayer), With<ThirdPersonCameraTarget>>, // query all players with an ThirdPersonCameraTarget as Component
    mut camera_query: Query<(&mut ThirdPersonCamera, &mut Transform), Without<ThirdPersonCameraTarget>>, // get all ThirdPersonCameras
) {

    // get player transformation and camera with camera transformation. if one of them is missing, we return here
    let Ok((player, player_details), ) = player_query.get_single() else { return };
    let Ok((cam, mut cam_t)) = camera_query.get_single_mut() else { return };

    // get current quat rotation from the camera
    let rotation_matrix = Mat3::from_quat(cam_t.rotation);

    let mut offset = Vec3::ZERO;

    // offset can be disabled... dont know why
    offset = rotation_matrix.mul_vec3(Vec3::new(cam.offset.position.0, cam.offset.position.1, 0.0)); // okay, hier muss ich nochmal die blöden matrix multiplikationen durchgehen
    // ich rechne hier: matrix * vector = offset

    let desired_translation =
        cam.focus + rotation_matrix.mul_vec3(Vec3::new(0.0,0.0, cam.zoom.radius)) + offset;
    
    //info!("look_at: {:?}", cam.focus);

    let delta = player.translation + cam.focus;
    cam_t.translation = desired_translation + delta;
    //info!("cam_t.translation: {:?}", cam_t.translation);

}

// heavily referenced https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html
pub fn orbit_mouse(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut cam_q: Query<(&ThirdPersonCamera, &mut Transform), With<ThirdPersonCamera>>,
    mouse: Res<Input<MouseButton>>,
    mut mouse_evr: EventReader<MouseMotion>,
) {
    let mut rotation = Vec2::ZERO;
    for ev in mouse_evr.read() {
        rotation = ev.delta;
    }

    let Ok((cam, mut cam_transform)) = cam_q.get_single_mut() else { return };

    //if cam.mouse_orbit_button_enabled && !mouse.pressed(cam.mouse_orbit_button) {
    //    return;
    //}

    rotation *= cam.mouse_sensitivity;

    if rotation.length_squared() > 0.0 {
        let window = window_q.get_single().unwrap();
        let delta_x = {
            let delta = rotation.x / window.width() * std::f32::consts::PI;
            delta
        };

        let delta_y = rotation.y / window.height() * PI;
        let yaw = Quat::from_rotation_y(-delta_x);
        let pitch = Quat::from_rotation_x(-delta_y);
        cam_transform.rotation = yaw * cam_transform.rotation; // rotate around global y axis

        // Calculate the new rotation without applying it to the camera yet
        let new_rotation = cam_transform.rotation * pitch;

        // check if new rotation will cause camera to go beyond the 180 degree vertical bounds
        let up_vector = new_rotation * Vec3::Y;
        if up_vector.y > 0.0 {
            cam_transform.rotation = new_rotation;
        }
    }

    let rot_matrix = Mat3::from_quat(cam_transform.rotation);
    cam_transform.translation =
        cam.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam.zoom.radius));
}

fn toggle_cursor(
    mut cam_q: Query<&mut ThirdPersonCamera>,
    keys: Res<Input<KeyCode>>,
    mut window_q: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(mut cam) = cam_q.get_single_mut() else { return };

    if keys.just_pressed(KeyCode::Escape) {
        cam.snap_mouse = !cam.snap_mouse;
    }

    let mut window = window_q.get_single_mut().unwrap();
    if cam.snap_mouse {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    } else {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}

//fn toggle_cursor_condition(cam_q: Query<&ThirdPersonCamera>) -> bool {
//    let Ok(cam) = cam_q.get_single() else { return true };
//    cam.snap_mouse
//}

// only run the orbit system if the cursor lock is disabled
fn orbit_condition(cam_q: Query<&ThirdPersonCamera>) -> bool {
    let Ok(cam) = cam_q.get_single() else { return true };
    return cam.snap_mouse;
}