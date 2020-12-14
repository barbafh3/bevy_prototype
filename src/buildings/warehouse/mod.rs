pub mod states;

pub struct Warehouse {
    pub is_sprite_set: bool,
}

impl Warehouse {
    pub fn new() -> Warehouse {
        let warehouse = Warehouse {
            is_sprite_set: false,
        };
        return warehouse;
    }
}
