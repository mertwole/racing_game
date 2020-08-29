use std::rc::Rc;

use crate::engine::common::{IVec2, Math};
use crate::engine::ui::font::*;
use crate::engine::ui::*;
use crate::game::*;
use crate::game::city_map::services::*;

use super::*;

pub struct RepairStationModal {
    selected_service : ServiceId,
    font : Rc<Font>,
    modal : ModalPage,
    diagnosed_modal : ModalPage,
    diagnosed : bool
}

impl RepairStationModal {
    pub fn new(resolution : &IVec2, font : Rc<Font>) -> RepairStationModal {
        let modal = ModalPage::new(IVec2::new(100, 100), IVec2::new(200, 100), Some(Rgb([150, 150, 150])));
        let diagnosed_modal = ModalPage::new(IVec2::new(100, 120), IVec2::new(200, 100), Some(Rgb([100, 100, 100])));
        RepairStationModal { 
            selected_service : ServiceId(0),
            font,
            modal,
            diagnosed_modal,
            diagnosed : false
        }
    }
}

impl ServiceModal for RepairStationModal {
    fn opened(&mut self, game : &Game) {
        let repair_station = game.city_map.get_service::<RepairStation>(self.selected_service);
        let diagnose_cost = UIText::new(self.font.clone(), format!("DIAGNOSE FOR {}$", repair_station.get_diagnosis_cost()));
        self.modal.add_control(Box::from(diagnose_cost), ControlProperties { position : IVec2::zero(), pivot : Pivot::Center, binding : Binding::Center });
    }
    
    fn update(&mut self, game : &Game, input : &Vec<(InputEvent, EventType)>, delta_time : f32) -> Vec<ServiceModalEvent> {
        for (event, event_type) in input {
            match (event, event_type) {
                (InputEvent::UISelect, EventType::Pressed) => { 
                    
                }
                (InputEvent::UIBack, EventType::Pressed) => { 
                    self.modal.start_anim_fold(1000.0);
                    return vec![ServiceModalEvent::Close];
                }
                _ => { }
            }
        }
        
        self.modal.update(delta_time);

        Vec::new()
    }

    fn select_service(&mut self, id : ServiceId) {
        self.selected_service = id;
    }

    fn modal(&self) -> &ModalPage { &self.modal }
    fn modal_mut(&mut self) -> &mut ModalPage { &mut self.modal }
}