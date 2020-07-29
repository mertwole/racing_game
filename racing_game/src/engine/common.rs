use image::{RgbImage, RgbaImage, Rgb};
use std::ops;

pub struct ImageOps { }
pub struct Math{ }
pub struct Geometry { }

struct FloodFillRange {
    x_left : isize,
    x_right : isize,
    y : isize
}

impl FloodFillRange {
    fn fill(&self, buffer : &mut RgbImage, color : &Rgb<u8>) {
        for x in self.x_left..self.x_right + 1{
            buffer.put_pixel(x as u32, self.y as u32, *color);
        }
    }
}

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

    // region floodfill
    fn floodfill(buffer : &mut RgbImage, point : &IVec2, color : &Rgb<u8>) {
        let mut starting_range = FloodFillRange {x_left : point.x, x_right : point.x, y : point.y };
        // Find left FloodFillRange's border.
        loop {
            if buffer.get_pixel(starting_range.x_left as u32 - 1, starting_range.y as u32) == color { break; }
            starting_range.x_left -= 1;
        }
        // Find right FloodFillRange's border.
        loop {
            if buffer.get_pixel(starting_range.x_right as u32 + 1, starting_range.y as u32) == color { break; }
            starting_range.x_right += 1;
        }

        starting_range.fill(buffer, color);
        ImageOps::floodfill_step(buffer, &starting_range, color);
    }

    fn floodfill_step(buffer : &mut RgbImage, current_fill : &FloodFillRange, color : &Rgb<u8>) {        
        ImageOps::floodfill_line(buffer, &current_fill, color, current_fill.y + 1); 
        ImageOps::floodfill_line(buffer, &current_fill, color, current_fill.y - 1);
    }

    fn floodfill_line(buffer : &mut RgbImage, current_fill : &FloodFillRange, color : &Rgb<u8>, y : isize) {
        let mut range_filling = false;
        let mut new_range = FloodFillRange { x_left : 0, x_right : 0, y : y };

        let mut x_left = current_fill.x_left;
        while buffer.get_pixel(x_left as u32, y as u32) != color { x_left -= 1; }
        let mut x_right = current_fill.x_right;
        while buffer.get_pixel(x_right as u32, y as u32) != color { x_right += 1; }

        for x in x_left..x_right + 1 {
            if !range_filling && buffer.get_pixel(x as u32, y as u32) != color {
                new_range.x_left = x;
                range_filling = true;
            } else if range_filling && buffer.get_pixel(x as u32, y as u32) == color {
                new_range.x_right = x - 1;
                new_range.fill(buffer, color);
                ImageOps::floodfill_step(buffer, &new_range, color);
                range_filling = false;
            }
        }

        if range_filling {
            new_range.x_right = x_right;
            new_range.fill(buffer, color);
            ImageOps::floodfill_step(buffer, &new_range, color);
        }
    }
    // endregion

    // region draw triangle
    fn draw_triangle_universal(buffer : &mut RgbImage, verts : &[IVec2; 3], color : &Rgb::<u8>, draw_half : fn(&mut RgbImage, &Line, &Line, isize, &Rgb<u8>) -> ()) {
        let mut sorted_verts = [IVec2::zero(); 3];
        sorted_verts.clone_from_slice(verts);
        sorted_verts.sort_by(|a, b| a.y.cmp(&b.y));

        // Draw top half.
        let left_triangle_side = Line::new(sorted_verts[0].vec2(), (&sorted_verts[1] - &sorted_verts[0]).vec2());
        let right_triangle_side = Line::new(sorted_verts[0].vec2(), (&sorted_verts[2] - &sorted_verts[0]).vec2());
        for y in sorted_verts[0].y..sorted_verts[1].y { draw_half(buffer, &left_triangle_side, &right_triangle_side, y, color); }
        // Draw bottom half.
        let left_triangle_side = Line::new(sorted_verts[2].vec2(), (&sorted_verts[0] - &sorted_verts[2]).vec2());
        let right_triangle_side = Line::new(sorted_verts[2].vec2(), (&sorted_verts[1] - &sorted_verts[2]).vec2());
        for y in sorted_verts[1].y..sorted_verts[2].y { draw_half(buffer, &left_triangle_side, &right_triangle_side, y, color); }
    }

    fn draw_triangle_half(buffer : &mut RgbImage, left_side : &Line, right_side : &Line, y : isize, color : &Rgb<u8>) {
        let horz_line = Line::new(Vec2::new(0.0, y as f32), Vec2::new(1.0, 0.0));
        let mut min_intersection = Geometry::line_intersect(&left_side, &horz_line);
        let mut max_intersection = Geometry::line_intersect(&right_side, &horz_line);
        if min_intersection.x > max_intersection.x { 
            let temp = min_intersection; 
            min_intersection = max_intersection; 
            max_intersection = temp; 
        }

        for x in min_intersection.x as u32..max_intersection.x as u32 + 1{
            buffer.put_pixel(x, y as u32, *color);
        }
    }

    fn draw_triangle_half_aa(buffer : &mut RgbImage, left_side : &Line, right_side : &Line, y : isize, color : &Rgb<u8>) {
        let horz_line = Line::new(Vec2::new(0.0, y as f32), Vec2::new(1.0, 0.0));
        let mut min_intersection = Geometry::line_intersect(&left_side, &horz_line);
        let mut max_intersection = Geometry::line_intersect(&right_side, &horz_line);
        if min_intersection.x > max_intersection.x { 
            let temp = min_intersection; 
            min_intersection = max_intersection; 
            max_intersection = temp; 
        }

        let min_intersection_x_rounded = min_intersection.x as u32;
        let max_intersection_x_rounded = max_intersection.x as u32;

        let min_aa = min_intersection_x_rounded as f32 - min_intersection.x + 0.5;
        let max_aa = max_intersection.x - max_intersection_x_rounded as f32 + 0.5;

        for x in min_intersection_x_rounded + 1..max_intersection_x_rounded{
            buffer.put_pixel(x, y as u32, *color);
        }

        let min_pixel = buffer.get_pixel_mut(min_intersection_x_rounded, y as u32);
        let min_pixel_add = Rgb([(color[0] as f32 * min_aa) as u8, (color[1] as f32 * min_aa) as u8, (color[2] as f32 * min_aa) as u8]);
        for i in 0..3{
            min_pixel[i] = Math::min(min_pixel[i] as u32 + min_pixel_add[i] as u32, 255) as u8;
        }

        let max_pixel = buffer.get_pixel_mut(max_intersection_x_rounded, y as u32);
        let max_pixel_add = Rgb([(color[0] as f32 * max_aa) as u8, (color[1] as f32 * max_aa) as u8, (color[2] as f32 * max_aa) as u8]);
        for i in 0..3{
            max_pixel[i] = Math::min(max_pixel[i] as u32 + max_pixel_add[i] as u32, 255) as u8;
        }
    }

    pub fn draw_triange(buffer : &mut RgbImage, verts : &[IVec2; 3], color : &Rgb::<u8>) {
        ImageOps::draw_triangle_universal(buffer, verts, color, ImageOps::draw_triangle_half);
    }

    pub fn draw_triange_aa(buffer : &mut RgbImage, verts : &[IVec2; 3], color : &Rgb::<u8>) {
        ImageOps::draw_triangle_universal(buffer, verts, color, ImageOps::draw_triangle_half_aa);
    }
    // endregion

    // Draw only lines that are fully inside the buffer.
    pub fn draw_line(buffer : &mut RgbImage, start : &IVec2, end : &IVec2, color : &Rgb::<u8>, width : u32 ) {
        let direction = &(end - start).vec2().normalized() * width as f32;
        let orth_direction = IVec2::new((-direction.y * 0.5) as isize, (direction.x * 0.5) as isize);

        let line_points = [start + &orth_direction, end + &orth_direction, end - &orth_direction, start - &orth_direction];

        ImageOps::draw_triange(buffer, &[line_points[0], line_points[1], line_points[2]], color);
        ImageOps::draw_triange(buffer, &[line_points[0], line_points[2], line_points[3]], color);
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
#[derive(Clone, Copy)]
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