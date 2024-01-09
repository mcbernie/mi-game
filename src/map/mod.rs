use bevy::{
    prelude::*, utils::HashMap,
};
use bevy_rapier3d::{prelude::*, rapier::geometry::ColliderShape};

use crate::game::GameResources;

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
    game_assets: Res<GameResources>,
    //mut materials: ResMut<Assets<StandardMaterial>>,
    //mut meshes: ResMut<Assets<Mesh>>,
) {



    //let m = &meshes.get(&mesh).unwrap();
    commands.spawn((
        //RigidBody::Fixed, 
        SceneBundle {
            scene: game_assets.map.clone(),
            transform: Transform::from_xyz(18.0, 0.0, -2394.0).with_scale(Vec3::new(2.0, 2.0, 2.0)),
            ..Default::default()
        }, 
        AsyncSceneCollider {
            shape: Some(ComputedColliderShape::TriMesh),
            ..Default::default()
        }
    ));
    commands.spawn(
        (
            //RigidBody::Fixed,
            //Collider::halfspace(Vec3::new(0.0, 1.0, 0.0)).unwrap(),
            Collider::cuboid(1.0, 0.01, 1.0),
            TransformBundle {
                local: Transform::from_xyz(6.0, 2013.0, 12.0),
                ..Default::default()
            },
        )
    );

}