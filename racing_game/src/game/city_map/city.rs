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