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
    commands.spawn(
        PointLightBundle {
            transform: Transform::from_xyz(26.0, 2012.57, 3.723),
            point_light: PointLight {
                intensity: 50.0,
                range: 100.0,
                color: Color::YELLOW,
                ..Default::default()
            },
            ..Default::default()
        }
    );
    commands.spawn(
        PointLightBundle {
            transform: Transform::from_xyz(-7.0, 2015.7, 29.9),
            point_light: PointLight {
                intensity: 50.0,
                range: 100.0,
                color: Color::YELLOW,
                ..Default::default()
            },
            ..Default::default()
        }
    );
    commands.spawn(
        PointLightBundle {
            transform: Transform::from_xyz(-34.718, 2017.0, 38.428),
            point_light: PointLight {
                intensity: 50.0,
                range: 100.0,
                color: Color::YELLOW,
                ..Default::default()
            },
            ..Default::default()
        }
    );
    commands.spawn(
        PointLightBundle {
            transform: Transform::from_xyz(-35.0, 2019.8, 38.7),
            point_light: PointLight {
                intensity: 50.0,
                range: 50.0,
                color: Color::YELLOW,
                ..Default::default()
            },
            ..Default::default()
        }
    );
    commands.spawn(
        PointLightBundle {
            transform: Transform::from_xyz(32.084, 2016.0, 56.18),
            point_light: PointLight {
                intensity: 50.0,
                range: 50.0,
                color: Color::YELLOW,
                ..Default::default()
            },
            ..Default::default()
        }
    );
    commands.spawn(
        PointLightBundle {
            transform: Transform::from_xyz(52.575, 2010.874, 66.183),
            point_light: PointLight {
                intensity: 50.0,
                range: 50.0,
                color: Color::YELLOW,
                ..Default::default()
            },
            ..Default::default()
        }
    );
    commands.spawn(
        PointLightBundle {
            transform: Transform::from_xyz(-11.341, 2015.874, 41.377),
            point_light: PointLight {
                intensity: 50.0,
                range: 50.0,
                color: Color::YELLOW,
                ..Default::default()
            },
            ..Default::default()
        }
    );

}