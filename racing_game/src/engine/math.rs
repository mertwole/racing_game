pub struct Math{

}

impl Math {
    pub fn lerp(a : f32, b : f32, t : f32) -> f32 {
        a + (b - a) * t
    }

    pub fn min<T>(a : T, b : T) -> T where T : std::cmp::PartialOrd {
        match a.partial_cmp(&b) {
            Some(std::cmp::Ordering::Greater) => { b },
            _ => { a }
        }
    }

    pub fn max<T>(a : T, b : T) -> T where T : std::cmp::PartialOrd {
        match a.partial_cmp(&b) {
            Some(std::cmp::Ordering::Less) => { b },
            _ => { a }
        }
    }
}