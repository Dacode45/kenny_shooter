use bevy::{
    asset::{HandleId, LoadState},
    prelude::*,
    render::{
        camera::{Camera, VisibleEntities},
        mesh::shape,
    },
    sprite::TextureAtlasBuilder,
};

mod camera;
mod components;
mod util;

use camera::*;
use components::*;
use util::*;

fn main() {
    App::build()
        .init_resource::<SpaceSpriteHandles>()
        .init_resource::<MouseScreenSpace>()
        .init_resource::<MouseWorldSpace>()
        .init_resource::<CameraTarget>()
        .init_resource::<Frame>()
        .add_default_plugins()
        .add_startup_system(load_assets.system())
        .add_startup_system(setup.system())
        // regular systems
        .add_system(frame.system())
        .add_system(set_mouse_pos.system())
        .add_system(set_mouse_world_space.system())
        .add_system(player_move.system())
        .add_system(camera_move.system())
        .add_system(spawn_bullets.system())
        .add_system(newton.system())
        .add_system(bounds_check.system())
        .run()
}

fn frame(mut f: ResMut<Frame>) {
    f.0 += 1;
}

fn load_assets(
    mut space_sprite_handles: ResMut<SpaceSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    space_sprite_handles.handles = asset_server.load_asset_folder("assets").unwrap();
}

fn setup(
    mut commands: Commands,
    mut sprites: ResMut<SpaceSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_texture = asset_server
        .get_handle("assets/Ships/spaceShips_001.png")
        .unwrap();

    let bullet_texture = asset_server
        .get_handle("assets/Missiles/spaceMissiles_001.png")
        .unwrap();

    sprites.bullet_mat = materials.add(bullet_texture.into());

    commands.spawn(Camera2dComponents {
        scale: Scale(5.0),
        ..Default::default()
    });

    commands
        .spawn(SpriteComponents {
            material: materials.add(player_texture.into()),
            ..Default::default()
        })
        .with(Player { speed: 10000.0 })
        .with(Velocity::default());

    commands.spawn(SpriteComponents {
        translation: Vec3::new(200.0, 200.0, 0.0).into(),
        material: materials.add(player_texture.into()),
        ..Default::default()
    });
}

fn set_mouse_pos(
    mut state: Local<CursorState>,
    windows: Res<Windows>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut mouse_pos: ResMut<MouseScreenSpace>,
) {
    if let Some(cursor_moved) = state.cursor_moved_event_reader.latest(&cursor_moved_events) {
        state.cursor_position = cursor_moved.position;
    }

    // point at the mouse\
    let window = windows.get_primary().unwrap();
    mouse_pos.0 = (state.cursor_position / Vec2::new(window.width as f32, window.height as f32))
        * 2.0
        - Vec2::new(1.0, 1.0);
}

fn set_mouse_world_space(
    mouse_pos: Res<MouseScreenSpace>,
    mut mouse_pos_world: ResMut<MouseWorldSpace>,
    camera: &Camera,
    scale: &Scale,
    translation: &Translation,
) {
    let world =
        camera.projection_matrix.inverse() * Vec4::new(mouse_pos.0.x(), mouse_pos.0.y(), 0.0, 1.0);

    mouse_pos_world.0 = (Vec2::new(world.x(), world.y()) * scale.0)
        + Vec2::new(translation.0.x(), translation.0.y());
}

/// move player
fn player_move(
    mouse_pos_world: Res<MouseWorldSpace>,
    mut target: ResMut<CameraTarget>,
    mut query: Query<(&Player, &Translation, &mut Rotation, &mut Velocity)>,
) {
    // remember that sprite points down not up
    for (player, translation, mut rotation, mut vel) in &mut query.iter() {
        // point at the mouse
        let dir = mouse_pos_world.0 - Vec2::new(translation.0.x(), translation.0.y());
        let angle = dir.y().atan2(dir.x());
        // - 90 degrees because the sprite points down
        *rotation = Rotation::from_rotation_z(angle + std::f32::consts::FRAC_PI_2);

        // Have a "deadzone" around player
        let motion_vec = if dir.length() < 10.0 || dir.length() > 5000.0 {
            Vec2::zero()
        } else {
            Vec2::new(
                dir.x().abs().min(3000.0) / 3000.0 * dir.x().signum(),
                dir.y().abs().min(3000.0) / 3000.0 * dir.y().signum(),
                // smoothstep(0.0, 3000.0, dir.x().abs()).max(0.1) * dir.x().signum(),
                // smoothstep(0.0, 3000.0, dir.y().abs()).max(0.1) * dir.y().signum(),
            )
        };
        let motion = motion_vec * player.speed;
        vel.0 = Vec3::new(motion.x(), motion.y(), 0.0);

        // set camera target
        target.0 = translation.0;
    }
}

/// move with velocity
fn newton(time: Res<Time>, mut query: Query<(&mut Translation, &Velocity)>) {
    for (mut translation, vel) in &mut query.iter() {
        translation.0 = translation.0 + vel.0 * time.delta_seconds;
    }
}

fn spawn_bullets(
    mut cmds: Commands,
    mut frame: Res<Frame>,
    mouse_pos: Res<MouseWorldSpace>,
    player: Res<CameraTarget>,
    button: Res<Input<MouseButton>>,
    sprites: Res<SpaceSpriteHandles>,
) {
    if frame.0 % 10 != 0 {
        return;
    }
    if button.pressed(MouseButton::Left) {
        let dir = Vec3::new(mouse_pos.0.x(), mouse_pos.0.y(), 0.0) - player.0;
        let dir = dir.normalize();
        // sprite faces up
        let angle = dir.y().atan2(dir.x()) - std::f32::consts::FRAC_PI_2;

        cmds.spawn(SpriteComponents {
            translation: player.0.into(),
            rotation: Rotation::from_rotation_z(angle),
            material: sprites.bullet_mat.clone(),
            ..Default::default()
        })
        .with(Velocity(dir * 10000.0))
        .with(DieOutOfBounds::default());
    }
}
