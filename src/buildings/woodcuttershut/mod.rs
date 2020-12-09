pub struct WoodcuttersHut {
    input_capacity: i32,
    storage_capacity: i32,
}

impl WoodcuttersHut {
    pub fn new(input_capacity: i32, storage_capacity: i32) -> WoodcuttersHut {
        WoodcuttersHut {
            input_capacity,
            storage_capacity,
        }
    }
}
