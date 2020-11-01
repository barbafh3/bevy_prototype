use super::super::managers::events::HealthIsFive;
use bevy::prelude::{EventReader, Events, Local, Query, Res, ResMut};

pub struct Health {
    pub value: i32,
}

pub fn change_health(mut query: Query<&mut Health>) {
    for mut health in &mut query.iter() {
        if health.value <= 5 {
            health.value += 1;
        }
    }
}

pub fn health_changed_dispatcher(
    mut events: ResMut<Events<HealthIsFive>>,
    mut query: Query<&Health>,
) {
    for health in &mut query.iter() {
        if health.value == 5 {
            events.send(HealthIsFive);
        }
    }
}

pub fn health_changed_listener(
    events: Res<Events<HealthIsFive>>,
    mut event_reader: Local<EventReader<HealthIsFive>>,
) {
    for _event in event_reader.iter(&events) {
        println!("Health reached 5!");
    }
}
