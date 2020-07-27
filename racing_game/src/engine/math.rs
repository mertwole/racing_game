use std::ops;

pub struct Math{ }
pub struct Geometry { }

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

impl Geometry {
    pub fn line_segment_intersect(segment_0 : &LineSegment, segment_1 : &LineSegment) -> bool {
        let line_intersection = Geometry::line_intersect(&segment_0.get_line(), &segment_1.get_line());
        if !segment_0.contains_point(&line_intersection) { return false; }
        if !segment_1.contains_point(&line_intersection) { return false; }
        true
    }

    fn line_intersect(line_0 : &Line, line_1 : &Line) -> Vec2 {
        // y1 = k1 * x + t1
        // y2 = k2 * x + t2
        // k1 * x + t1 = k2 * x + t2
        // x * (k1 - k2) = t2 - t1
        // x = (t2 - t1) / (k1 - k2)
        // y = k1 * x + t1
        let k1 = line_0.direction.y / line_0.direction.x;
        let k2 = line_1.direction.y / line_1.direction.x;
        let t1 = line_0.pass_through.y - k1 * line_0.pass_through.x;
        let t2 = line_1.pass_through.y - k2 * line_1.pass_through.x;

        // Means that line_0 is too vertical.
        if k1 > 1000.0 || k1 < -1000.0 { return Vec2::new(line_0.pass_through.x, k2 * line_0.pass_through.x + t2); }

        // Means that line_1 is too vertical.
        if k2 > 1000.0 || k2 < -1000.0 { return Vec2::new(line_1.pass_through.x, k1 * line_1.pass_through.x + t1); }

        let intersection_x = (t2 - t1) / (k1 - k2);
        Vec2::new(intersection_x, k1 * intersection_x + t1)
    }
}

// region LineSegment 

pub struct LineSegment {
    start : Vec2,
    end : Vec2
}

impl LineSegment{
    pub fn new(start : Vec2, end : Vec2) -> LineSegment {
        LineSegment { start, end }
    }

    pub fn sqr_length(&self) -> f32 {
        (&self.end - &self.start).sqr_len()
    }

    pub fn scale(&mut self, scale : f32) {
        let midpoint = &(&self.start + &self.end) * 0.5;
        let half_new = &(&self.end - &midpoint) * scale;
        self.start = &midpoint - &half_new;
        self.end = &midpoint + &half_new;
    }

    fn get_line(&self) -> Line {
        Line { pass_through : self.start.clone(), direction : &self.end - &self.start }
    }

    fn contains_point(&self, point : &Vec2) -> bool{
        (&self.end - point).dot(&(point - &self.start)) > 0.0
    }
}

// endregion

// region Line

pub struct Line{
    pass_through : Vec2,
    direction : Vec2
}

impl Line { 
    pub fn new(pass_through : Vec2, direction : Vec2) -> Line {
        Line { pass_through, direction }
    }
}

// endregion

// region Vec2 

pub struct Vec2 {
    pub x : f32,
    pub y : f32
}

impl Vec2{
    pub fn new(x : f32, y : f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn clone(&self) -> Vec2 {
        Vec2 { x : self.x, y : self.y }
    }

    pub fn dot(&self, rhs : &Vec2) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn sqr_len(&self) -> f32{
        self.dot(&self)
    }
}

impl ops::Add<&Vec2> for &Vec2 {
    type Output = Vec2;
    fn add(self, rhs: &Vec2) -> Vec2 {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub<&Vec2> for &Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: &Vec2) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Mul<f32> for &Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Vec2 {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl ops::Mul<&Vec2> for f32 {
    type Output = Vec2;
    fn mul(self, rhs: &Vec2) -> Vec2 {
        Vec2::new(self * rhs.x, self * rhs.y)
    }
}

//endregion

// region IVec2 
pub struct IVec2{
    pub x : isize,
    pub y : isize
}

impl IVec2{
    pub fn new(x : isize, y : isize) -> IVec2{
        IVec2 { x, y }
    }

    pub fn zero() -> IVec2{
        IVec2 { x : 0, y : 0 }
    }

    pub fn vec2(&self) -> Vec2{
        Vec2 { x : self.x as f32, y : self.y as f32 }
    }

    pub fn clone(&self) -> IVec2{
        IVec2 { x : self.x, y : self.y }
    }

    pub fn sqr_len(&self) -> isize {
        self.x * self.x + self.y * self.y
    }
}

impl ops::Add<&IVec2> for &IVec2 {
    type Output = IVec2;
    fn add(self, rhs: &IVec2) -> IVec2 {
        IVec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub<&IVec2> for &IVec2 {
    type Output = IVec2;
    fn sub(self, rhs: &IVec2) -> IVec2 {
        IVec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

// endregion