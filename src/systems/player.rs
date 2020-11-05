use bevy::prelude::{Input, KeyCode, Query, Res, Time, Transform};

pub struct Player {
    pub name: String,
    pub speed: f32,
}

pub fn move_player(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in query.iter_mut() {
        let mut direction_x: f32 = 0.0;
        let mut direction_y: f32 = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            direction_x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction_x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction_y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction_y += 1.0;
        }
        let translation = &mut transform.translation;
        *translation.x_mut() += direction_x * player.speed * time.delta_seconds;
        *translation.y_mut() += direction_y * player.speed * time.delta_seconds;
    }
}
