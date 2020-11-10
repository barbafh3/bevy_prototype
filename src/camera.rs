use bevy::{
    ecs::Entity,
    ecs::Query,
    ecs::Res,
    ecs::ResMut,
    math::Vec2,
    prelude::EventReader,
    prelude::{Events, Transform},
    window::{CursorMoved, Windows},
};

#[derive(Default)]
pub struct CameraData {
    pub position: Vec2,
}

pub struct CustomCursorState {
    pub cursor: EventReader<CursorMoved>,
    // need to identify the main camera
    pub camera_e: Entity,
}

pub fn sys_cursor_position(
    mut state: ResMut<CustomCursorState>,
    mut camera_position: ResMut<CameraData>,
    ev_cursor: Res<Events<CursorMoved>>,
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera components
    q_camera: Query<&Transform>,
) {
    let camera_transform = q_camera.get(state.camera_e).unwrap();

    for ev in state.cursor.iter(&ev_cursor) {
        // get the size of the window that the event is for
        let wnd = wnds.get(ev.id).unwrap();
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = ev.position - size / 2.0;

        // apply the camera transform
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        camera_position.position = Vec2::new(pos_wld.x(), pos_wld.y());
    }
}
