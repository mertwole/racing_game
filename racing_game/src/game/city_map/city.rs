use crate::engine::common::IVec2;

pub enum CityDescription {
    Start,
    Finish,
    Intermediate
}

pub struct City {
    pub position : IVec2,
    pub description : CityDescription
}

impl City {
    pub fn new(position : IVec2, description : CityDescription) -> City {
        City { position, description }
    }
}