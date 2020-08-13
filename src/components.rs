use bevy::{
    asset::{HandleId, LoadState},
    prelude::*,
    render::{
        camera::{Camera, VisibleEntities},
        mesh::shape,
    },
    sprite::TextureAtlasBuilder,
};

#[derive(Default)]
pub struct Frame(pub u32);

#[derive(Default)]
pub struct SpaceSpriteHandles {
    pub handles: Vec<HandleId>,
    pub atlas_loaded: bool,
    pub bullet_mat: Handle<ColorMaterial>,
}

#[derive(Default)]
pub struct CursorState {
    pub cursor_moved_event_reader: EventReader<CursorMoved>,
    pub cursor_position: Vec2,
}

#[derive(Default)]
pub struct MouseScreenSpace(pub Vec2);

#[derive(Default)]
pub struct MouseWorldSpace(pub Vec2);

pub struct Player {
    pub speed: f32,
}

#[derive(Default)]
pub struct Velocity(pub Vec3);

#[derive(Default)]
pub struct CameraTarget(pub Vec3);

#[derive(Default)]
pub struct DieOutOfBounds(pub u32);
