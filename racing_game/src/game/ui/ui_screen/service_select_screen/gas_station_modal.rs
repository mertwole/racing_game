use std::rc::Rc;

use crate::engine::common::{IVec2, Math};
use crate::engine::ui::font::*;
use crate::engine::ui::*;
use crate::game::*;
use crate::game::city_map::services::*;

use super::*;

pub struct GasStationModal{
    buy_gas_amount : u32,
    selected_station : ServiceId,
    font : Rc<Font>,
    pub modal : ModalPage
}

impl GasStationModal {
    pub fn new(resolution : &IVec2, font : Rc<Font>) -> GasStationModal {
        let modal = ModalPage::new(IVec2::new(100, 100), IVec2::new(200, 100), Some(Rgb([150, 150, 150])));
        GasStationModal { 
            buy_gas_amount : 0,
            selected_station : ServiceId(0),
            font,
            modal
        }
    }
}

impl ServiceModal for GasStationModal {
    fn opened(&mut self, game : &Game) {
        
    }
    
    fn update(&mut self, game : &Game, input : &Vec<(InputEvent, EventType)>, delta_time : f32) -> Vec<ServiceModalEvent> {
        let gas_station = game.city_map.get_service::<GasStation>(self.selected_station);
        let player_money = game.player.money;

        for (event, event_type) in input {
            match (event, event_type) {
                (InputEvent::UIDown, EventType::Pressed) => { 
                    if self.buy_gas_amount >= 1 { 
                        self.buy_gas_amount -= 1;
                    } 
                }
                (InputEvent::UIUp, EventType::Pressed) => { 
                    self.buy_gas_amount = Math::min(gas_station.get_max_gas_amount(player_money), self.buy_gas_amount + 1);
                }
                (InputEvent::UISelect, EventType::Pressed) => { 
                    return vec![
                        ServiceModalEvent::UIEvent(
                            UIEvent::ServiceAction(self.selected_station, ServiceAction::BuyGas(self.buy_gas_amount))
                        )
                    ]; 
                }
                (InputEvent::UIBack, EventType::Pressed) => { 
                    self.modal.start_anim_fold(1000.0);
                    self.buy_gas_amount = 0;
                    return vec![ServiceModalEvent::Close];
                }
                _ => { }
            }
        }

        self.modal.clear_controls();
        let cost = gas_station.get_cost(self.buy_gas_amount);
        let buy_string = self.buy_gas_amount.to_string() + "L. FOR " + cost.to_string().as_ref() + "$";
        let text = UIText::new(self.font.clone(), buy_string);
        let text_props = ControlProperties { binding : Binding::Center, pivot : Pivot::Center, position : IVec2::zero() };
        self.modal.add_control(Box::from(text), text_props);
        self.modal.update(delta_time);

        return Vec::new();
    }

    fn select_service(&mut self, id: ServiceId) {
        self.selected_station = id;
    }

    fn modal_mut(&mut self) -> &mut ModalPage { &mut self.modal } 
    fn modal(&self) -> &ModalPage { &self.modal } 
}