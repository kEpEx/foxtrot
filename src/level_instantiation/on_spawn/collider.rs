use crate::{movement::physics::CollisionLayer, GameState};
use anyhow::Context;
use bevy::prelude::*;
use bevy_mod_sysfail::prelude::*;
use bevy_xpbd_3d::prelude::{Collider as XpbdCollider, *};
use oxidized_navigation::NavMeshAffector;
use serde::{Deserialize, Serialize};
use std::iter;

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
struct Collider;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Collider>()
        .add_systems(Update, spawn.run_if(in_state(GameState::Playing)));
}

#[sysfail(Log<anyhow::Error, Error>)]
fn spawn(
    collider_marker: Query<Entity, With<Collider>>,
    mut commands: Commands,
    children: Query<&Children>,
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("read_colliders").entered();
    for parent in collider_marker.iter() {
        for child in iter::once(parent).chain(children.iter_descendants(parent)) {
            let Ok(mesh_handle) = mesh_handles.get(child) else {
                continue;
            };
            // Cannot fail: we already load all the meshes at startup.
            let mesh = meshes.get(mesh_handle).unwrap();
            let collider = XpbdCollider::trimesh_from_mesh(mesh)
                .context("Failed to create collider from mesh")?;
            commands.entity(child).insert((
                collider,
                CollisionLayers::new(
                    [CollisionLayer::Terrain, CollisionLayer::CameraObstacle],
                    [CollisionLayer::Character],
                ),
                NavMeshAffector,
            ));
        }
        commands
            .entity(parent)
            .remove::<Collider>()
            .insert(RigidBody::Static);
    }
}
