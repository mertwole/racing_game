use image::{RgbaImage, RgbImage};

use super::*;
use crate::engine::common::ImageOps;

pub struct MenuItem<E : Clone> {
    control : Box<dyn UIControl>,
    control_properties : ControlProperties,
    event : E
}

impl<E : Clone> MenuItem<E> {
    pub fn new(control : Box<dyn UIControl>, control_properties : ControlProperties, event : E) -> MenuItem<E> {
        MenuItem { control, event, control_properties }
    } 
}

pub struct SelectorMenu<E : Clone> {
    page : UIPage,
    control_events : Vec<Box<E>>,
    pointer_positions : Vec<IVec2>,
    pointer_img : RgbaImage,
    selected_item : usize
}

impl<E : Clone> SelectorMenu<E> {
    pub fn new(items : Vec<MenuItem<E>>, pointer_image : RgbaImage, resolution : IVec2) -> SelectorMenu<E> {
        let mut pointer_positions : Vec<IVec2> = Vec::with_capacity(items.len());
        let pointer_offset = IVec2::new(-(pointer_image.width() as isize), 0);
        let mut page = UIPage::new(resolution, None);
        let mut control_events : Vec<Box<E>> = Vec::with_capacity(items.len());

        for item in items {
            control_events.push(Box::from(item.event));
            page.add_control(Box::from(item.control), &item.control_properties);          
        }

        for control in &page.controls {
            let pointer_pos = &control.get_position() + &pointer_offset;
            pointer_positions.push(pointer_pos);
        }

        SelectorMenu { page, control_events, pointer_positions, pointer_img : pointer_image, selected_item : 0 }
    }

    pub fn select_next_in_direction(&mut self, direction : &IVec2) {
        // TODO : fair method of selection.
        let mut selected = self.selected_item as isize + direction.y;
        if selected < 0 { selected = self.pointer_positions.len() as isize - 1 };
        if selected >= self.pointer_positions.len() as isize { selected = 0 };
        self.selected_item = selected as usize;
    }

    pub fn select_current(&mut self) -> E {
        self.control_events[self.selected_item].as_ref().clone()
    }

    pub fn render(&self, buffer : &mut RgbImage) {
        self.page.draw(buffer);

        let pointer_pos = self.pointer_positions[self.selected_item];
        ImageOps::overlay_rgba(buffer, &self.pointer_img, &pointer_pos);
    }
}