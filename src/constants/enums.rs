use std::fmt;

#[derive(Debug)]
pub enum ConstructionResources {
    Wood,
    Stone,
    Plank,
    StonBrick,
}

impl fmt::Display for ConstructionResources {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum JobTypes {
    Hauler,
    Builder,
    Carpenter,
    Woodcutter,
}

impl fmt::Display for JobTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum TaskTypes {
    Haul,
    Build,
}

impl fmt::Display for TaskTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
