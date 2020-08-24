use image::{RgbaImage, RgbImage};

use super::*;
use crate::engine::ui::*;
use crate::engine::common::{ImageOps, Math};

pub struct UISelectorItem<E : Clone> {
    control : Box<dyn UIControl>,
    control_properties : ControlProperties,
    event : E
}

impl<E : Clone> UISelectorItem<E> {
    pub fn new(control : Box<dyn UIControl>, control_properties : ControlProperties, event : E) -> UISelectorItem<E> {
        UISelectorItem { control, event, control_properties }
    } 
}

pub struct UISelector<E : Clone> {
    page : UIPage,
    control_events : Vec<Box<E>>,
    pointer_positions : Vec<IVec2>,
    pointer_img : RgbaImage,
    selected_item : usize,
    selection_type : SelectionType,
    resolution : IVec2
}

#[derive(Clone)]
pub enum SelectionType {
    Vertical,
    Horizontal,
    Grid
}

// Fullscreen menu overlays UI.
impl<E : Clone> UISelector<E> {
    pub fn new(items : Vec<UISelectorItem<E>>, selection_type : SelectionType, pointer_image : RgbaImage, resolution : IVec2) -> UISelector<E> {
        let mut pointer_positions : Vec<IVec2> = Vec::with_capacity(items.len());
        let pointer_offset = IVec2::new(-(pointer_image.width() as isize), 0);
        let mut page = UIPage::new(resolution.clone(), None);
        let mut control_events : Vec<Box<E>> = Vec::with_capacity(items.len());

        for item in items {
            control_events.push(Box::from(item.event));
            page.add_control(Box::from(item.control), &item.control_properties);          
        }

        for control in &page.controls {
            let pointer_pos = &control.get_position() + &pointer_offset;
            pointer_positions.push(pointer_pos);
        }

        UISelector { page, control_events, pointer_positions, pointer_img : pointer_image, selected_item : 0, selection_type, resolution }
    }

    pub fn select_next_in_direction(&mut self, direction : &IVec2) {
        match self.selection_type {
            SelectionType::Vertical => { if direction.y == 0 { return; } }
            SelectionType::Horizontal => { if direction.x == 0 { return; } }
            SelectionType::Grid => { }
        }

        let mut next_selected = -1;
        let curr_pointer_pos = self.pointer_positions[self.selected_item];
        let mut min_dot = std::isize::MAX;
        let mut min_dist = 0;

        for i in 0..self.pointer_positions.len() {
            if self.selected_item == i { continue; }

            let dist = &self.pointer_positions[i] - &curr_pointer_pos;
            let dot = dist.dot(&direction);
            if dot <= 0 { continue; }

            if dot < min_dot || (dot == min_dot && dist.sqr_len() < min_dist) {
                min_dot = dot;
                next_selected = i as isize;
                min_dist = dist.sqr_len();
            }
        }

        if next_selected != -1 { 
            self.selected_item = next_selected as usize;
        }
    }

    pub fn select_current(&mut self) -> E {
        self.control_events[self.selected_item].as_ref().clone()
    }
}

impl<E> UIControl for UISelector<E> where E : Clone {
    fn draw(&self, buffer : &mut RgbImage) {
        self.page.draw(buffer);

        let pointer_pos = self.pointer_positions[self.selected_item];
        ImageOps::overlay_rgba(buffer, &self.pointer_img, &pointer_pos);
    }

    fn set_position(&mut self, position : IVec2) { }   
    fn get_position(&self) -> IVec2 { IVec2::zero() }
    fn get_size(&self) -> IVec2 { self.resolution.clone() }   
}