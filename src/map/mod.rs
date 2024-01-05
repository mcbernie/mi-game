use bevy::{
    prelude::*, utils::HashMap,
};
use bevy_rapier3d::{prelude::*, rapier::geometry::ColliderShape};

#[derive(Resource, Default)]
pub struct MapGenerationColliderStatus {
    pub already_generated: bool,
}


#[derive(Component, Default)]
pub struct LevelMap {
    pub colliders_generated: bool
}

pub fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {


    let scene: Handle<Scene> = asset_server.load("de_dust2.glb#Scene0");

    //let m = &meshes.get(&mesh).unwrap();
    commands.spawn((
        RigidBody::Fixed, 
        SceneBundle {
            scene,
            transform: Transform::from_xyz(18.0, 0.0, -2394.0).with_scale(Vec3::new(2.0, 2.0, 2.0)),
            ..Default::default()
        }, 
        AsyncSceneCollider {
            shape: Some(ComputedColliderShape::TriMesh),
            ..Default::default()
        }
    ));

}