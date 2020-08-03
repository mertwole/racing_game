use rand::{RngCore, rngs::StdRng};

mod gas_station;
mod hostel;
mod repair_station;
mod shop;

pub use gas_station::*;
pub use hostel::*;
pub use repair_station::*;
pub use shop::*;

pub struct CityServicesSubset {
    gas_station_ids : Vec<usize>,
    hostel_ids : Vec<usize>,
    repair_station_ids : Vec<usize>,
    shop_ids : Vec<usize>
}

pub struct Services {
    gas_stations : Vec<GasStation>,
    hostels : Vec<Hostel>,
    repair_stations : Vec<RepairStation>,
    shops : Vec<Shop>
}

impl Services {
    pub fn generate(rng : &mut StdRng) -> Services {
        let gas_stations = Vec::new();
        let hostels = Vec::new();
        let repair_stations = Vec::new();
        let shops = Vec::new();

        

        Services { gas_stations, hostels, repair_stations, shops }
    }
}