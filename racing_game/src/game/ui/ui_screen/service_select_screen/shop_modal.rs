use std::rc::Rc;

use crate::engine::common::{IVec2, Math};
use crate::engine::ui::font::*;
use crate::engine::ui::*;
use crate::game::*;
use crate::game::city_map::services::*;

use super::*;

pub struct ShopModal {
    buy_gas_amount : u32,
    selected_service : ServiceId,
    font : Rc<Font>,
    pub modal : ModalPage
}

impl ShopModal {
    pub fn new(resolution : &IVec2, font : Rc<Font>) -> ShopModal {
        let modal = ModalPage::new(IVec2::new(100, 100), IVec2::new(200, 100), Some(Rgb([150, 150, 150])));
        ShopModal { 
            buy_gas_amount : 0,
            selected_service : ServiceId(0),
            font,
            modal
        }
    }
}

impl ServiceModal for ShopModal {  
    fn unfold(&mut self, game : &Game) { self.modal.start_anim_unfold(1000.0); }

    fn update(&mut self, game : &Game, input : &Vec<(InputEvent, EventType)>, delta_time : f32) -> Vec<ServiceModalEvent> {
        Vec::new()
    }

    fn select_service(&mut self, id: ServiceId) { self.selected_service = id; }

    fn is_busy(&self) -> bool { self.modal.anim_state != ModalAnim::Void }

    fn draw(&self, buffer : &mut RgbImage) { self.modal.draw(buffer); }
}