use bevy::{
    prelude::*,
};
use bevy_rapier3d::prelude::*;

pub fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {


    let scene: Handle<Scene> = asset_server.load("scene.gltf#Scene0");
    //let mesh: Handle<Mesh> = asset_server.load("scene.gltf#Mesh0/Primitive0");
    let raw_mesh = Mesh::from(shape::Plane { size: 10.0, subdivisions: 32 });
    let m = meshes.add(raw_mesh.clone());

    commands.spawn((
        PbrBundle {
            mesh: m,
            material: materials.add(Color::rgb_u8(124, 144, 255).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        }, 
        RigidBody::Fixed, 
        Collider::from_bevy_mesh(&raw_mesh, &ComputedColliderShape::TriMesh).unwrap(),
    ));

    //let m = &meshes.get(&mesh).unwrap();
    commands.spawn((SceneBundle {
        scene,
        transform: Transform::from_xyz(-12.0, -1009.4, -1196.4),
        ..Default::default()
    }, Collider::cuboid(10.0, 1.0, 10.0)));

}