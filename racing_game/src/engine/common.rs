use image::{RgbImage, RgbaImage, Rgb};
use std::ops;

pub struct ImageOps { }
pub struct Math{ }
pub struct Geometry { }

impl ImageOps {
    pub fn overlay_rgba(bottom : &mut RgbImage, top : &RgbaImage, position : &IVec2) {
        for x in Math::max(0, -position.x)..Math::min(top.width() as isize, bottom.width() as isize - position.x) {
            for y in Math::max(0, -position.y)..Math::min(top.height() as isize, bottom.height() as isize - position.y){
                let image_pixel = top.get_pixel(x as u32, top.height() - y as u32 - 1);
                if image_pixel[3] == 0 { continue; }
                bottom.put_pixel((position.x + x) as u32, (position.y + y) as u32, Rgb([image_pixel[0], image_pixel[1], image_pixel[2]]));
            }
        }
    }

    // Draw only lines that are fully inside the buffer.
    pub fn draw_line_one_pixel(buffer : &mut RgbImage, start : &IVec2, end : &IVec2, color : &Rgb::<u8>) {
        if start.x < 0 || start.x >= buffer.width() as isize { return; }
        if start.y < 0 || start.y >= buffer.height() as isize { return; }
        if end.x < 0 || end.x >= buffer.width() as isize { return; }
        if end.y < 0 || end.y >= buffer.height() as isize { return; }

        for pixel_pos in Geometry::one_pixel_line_pixels(start, end) {
            buffer.put_pixel(pixel_pos.x as u32, pixel_pos.y as u32, *color);
        }
    }

    // Draw only lines that are fully inside the buffer.
    pub fn draw_line(buffer : &mut RgbImage, start : &IVec2, end : &IVec2, color : &Rgb::<u8>, width : u32 ) {
        let direction = &(end - start).vec2().normalized() * width as f32;
        let orth_direction = IVec2::new(-direction.y as isize, direction.x as isize);
        let mut orth_direction_points : Vec<IVec2> = Geometry::one_pixel_line_pixels(&IVec2::zero(), &(&IVec2::zero() - &orth_direction));
        orth_direction_points.append(&mut Geometry::one_pixel_line_pixels(&IVec2::zero(), &orth_direction));

        for point_pos in Geometry::one_pixel_line_pixels(start, end) {
            for orth_dir_point in &orth_direction_points {
                let global_point_pos = &point_pos + orth_dir_point;
                buffer.put_pixel(global_point_pos.x as u32, global_point_pos.y as u32, *color);
            }
        }
    } 

    fn draw_rect_outline_one_pixel(buffer : &mut RgbImage, aabb : &IAABB, color : &Rgb::<u8>) {   
        ImageOps::draw_line_one_pixel(buffer, &IVec2::new(aabb.min.x, aabb.max.y), &IVec2::new(aabb.max.x, aabb.max.y), color); // Top.    
        ImageOps::draw_line_one_pixel(buffer, &IVec2::new(aabb.max.x, aabb.max.y), &IVec2::new(aabb.max.x, aabb.min.y), color); // Right.     
        ImageOps::draw_line_one_pixel(buffer, &IVec2::new(aabb.max.x, aabb.min.y), &IVec2::new(aabb.min.x, aabb.min.y), color); // Bottom.
        ImageOps::draw_line_one_pixel(buffer, &IVec2::new(aabb.min.x, aabb.min.y), &IVec2::new(aabb.min.x, aabb.max.y), color); // Left.
    }

    pub fn draw_rect_outline(buffer : &mut RgbImage, aabb : &IAABB, color : &Rgb::<u8>, width : u32 ) {
        let mut one_line_aabb = aabb.clone();
        for _i in 0..width as isize {
            ImageOps::draw_rect_outline_one_pixel(buffer, &one_line_aabb, color);
            one_line_aabb.expand(IVec2::new(1, 1));
        }
    }
}

impl Math {
    pub fn lerp(a : f32, b : f32, t : f32) -> f32 {
        a + (b - a) * t
    }

    pub fn sgn_isize(a : isize) -> isize {
        if a < 0 { -1 } else if a == 0 { 0 } else { 1 }
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

    pub fn triangle_area_ivec2(vert_0 : &IVec2, vert_1 : &IVec2, vert_2 : &IVec2) -> isize {
        // Gauss's area formula.
        // Area = x0 * y1 + x1 * y2 + x2 * y0 - x1 * y0 - x2 * y1 - x0 * y2.
        // Area = x0 * (y1 - y2) + x1 * (y2 - y0) + x2 * (y0 - y1).
        vert_0.x * (vert_1.y - vert_2.y) + vert_1.x * (vert_2.y - vert_0.y) + vert_2.x * (vert_0.y - vert_1.y)
    }

    pub fn one_pixel_line_pixels(start : &IVec2, end : &IVec2) -> Vec<IVec2> {
        let delta = end - start;
        let sector = IVec2::new(Math::sgn_isize(delta.x), Math::sgn_isize(delta.y));

        if sector.x == 0 { 
            return (Math::min(start.y, end.y)..Math::max(start.y, end.y) + 1).map(|y| IVec2::new(start.x, y)).collect();
        }

        if sector.y == 0 { 
            return (Math::min(start.x, end.x)..Math::max(start.x, end.x) + 1).map(|x| IVec2::new(x, start.y)).collect();
        }

        let mut points : Vec<IVec2> = Vec::with_capacity((delta.x.abs() + delta.y.abs()) as usize);
        let mut next_point = start.clone();
        loop {
            points.push(next_point.clone());
            // Select 3 possible points, compare distances and select the closest to line. 
            // As start and end points are constant for all points compare only triangle areas.
            let possible_points = vec![&next_point + &IVec2::new(0, sector.y), &next_point + &IVec2::new(sector.x, 0), &next_point + &sector];
            next_point = possible_points.into_iter().min_by_key(|point| Geometry::triangle_area_ivec2(&point, end, start).abs()).unwrap();

            if next_point.x == end.x && next_point.y == end.y { return points; }
        }
    }
}

// region IAABB
pub struct IAABB{
    pub min : IVec2,
    pub max : IVec2
}

impl IAABB {
    pub fn new(min : IVec2, max : IVec2) -> IAABB {
        IAABB { min, max }
    } 

    pub fn clone(&self) -> IAABB {
        IAABB { min : self.min.clone(), max : self.max.clone() }
    }

    pub fn expand(&mut self, value : IVec2) {
        self.min = &self.min - &value;
        self.max = &self.max + &value;
    }
}

// endregion

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

    pub fn len(&self) -> f32 {
        self.sqr_len().sqrt()
    } 

    pub fn normalized(&self) -> Vec2 {
        let len = self.len();
        Vec2 { x : self.x / len, y : self.y / len }
    }

    pub fn ivec2(&self) -> IVec2 {
        IVec2 { x : self.x as isize, y : self.y as isize }
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

impl ops::Div<isize> for &IVec2 {
    type Output = IVec2;
    fn div(self, rhs: isize) -> IVec2 {
        IVec2::new(self.x / rhs, self.y / rhs)
    }
}

// endregion