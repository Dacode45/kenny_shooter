use bevy::{
    asset::{HandleId, LoadState},
    prelude::*,
    render::{
        camera::{Camera, VisibleEntities},
        mesh::shape,
    },
    sprite::TextureAtlasBuilder,
};

use crate::components::*;
use crate::util::*;

/// camera move
pub fn camera_move(
    target: Res<CameraTarget>,
    mut query: Query<(&Camera, &Scale, &mut Translation)>,
) {
    for (camera, scale, mut translation) in &mut query.iter() {
        // Create a camera bounds
        let inv = camera.projection_matrix.inverse();
        let t4 = Vec4::new(translation.0.x(), translation.0.y(), translation.0.z(), 0.0);
        let tr = (inv * Vec4::new(0.4, 0.4, 0.0, 0.0) * scale.0) + t4;
        let bl = (inv * Vec4::new(-0.4, -0.4, 0.0, 0.0) * scale.0) + t4;
        let x = target.0.x();
        let y = target.0.y();
        // Check if point is out of bounds
        if x < bl.x() || x > tr.x() || y < bl.y() || y > tr.y() {
            let intercect = Vec4::new(
                x.max(bl.x()).min(tr.x()),
                y.max(bl.y()).min(tr.y()),
                0.0,
                0.0,
            ) - t4;
            let intercect = camera.projection_matrix * intercect;
            let distance = intercect.length() / scale.0;

            // min distance is 0.4
            let t = smoothstep(0.3, 0.5, distance) * 0.01;

            // lerping is an awesome way for quick and dirty velocities over distances.
            // Notice how I don't even have to use time here. Every frame I get a bit closer.
            // Bad for physics, great for everything else.
            translation.0 = Vec3::new(
                lerp(translation.0.x(), target.0.x(), t),
                lerp(translation.0.y(), target.0.y(), t),
                lerp(translation.0.z(), target.0.z(), t),
            );
        }
    }
}

pub fn bounds_check(
    mut cmds: Commands,
    mut frame: Res<Frame>,
    mut camera_q: Query<(&Camera, &Translation, &Scale)>,
    mut ent_q: Query<(Entity, &mut DieOutOfBounds, &Translation)>,
) {
    if frame.0 % 10 == 0 {
        return;
    }
    for (c, ct, cs) in &mut camera_q.iter() {
        for (e, mut die, et) in &mut ent_q.iter() {
            let pos = et.0;
            let pos = c.projection_matrix * Vec4::new(pos.x(), pos.y(), pos.z(), 0.0);
            let pos = pos / cs.0;

            cmds.despawn(e);
        }
    }
}
