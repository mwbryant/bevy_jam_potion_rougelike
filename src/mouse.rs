use bevy::{prelude::*, render::camera::RenderTarget};

#[derive(Clone, Copy, Deref, DerefMut, Default)]
pub struct MousePos(pub Vec2);

//Thanks cheatbook! https://bevy-cheatbook.github.io/cookbook/cursor2world.html
pub fn mouse_position(
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut mouse_pos: ResMut<MousePos>,
) {
    let (camera, camera_transform) = q_camera.single();

    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        **mouse_pos = world_pos.truncate();
    }
}
