use crate::engine::common::IVec2;

#[readonly::make]
pub struct Road {
    pub source_pos : IVec2,
    pub destination_pos : IVec2
}

impl Road {
    pub fn new(source_pos : IVec2, destination_pos : IVec2) -> Road {
        Road { source_pos, destination_pos }
    }
}