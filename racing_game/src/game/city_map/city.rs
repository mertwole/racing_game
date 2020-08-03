use crate::engine::common::IVec2;
use super::services::*;

pub enum CityDescription {
    Start,
    Finish,
    Intermediate
}

pub struct City {
    pub position : IVec2,
    pub description : CityDescription,
    pub services : CityServicesSubset
}

impl City {
    pub fn new(position : IVec2, description : CityDescription, services : CityServicesSubset) -> City {
        City { position, description, services }
    }
}