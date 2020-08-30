use std::rc::Rc;
use std::collections::HashMap;

use crate::engine::common::{IVec2, Math};
use crate::engine::ui::font::*;
use crate::engine::ui::*;
use crate::game::*;
use crate::game::city_map::services::*;
use crate::game::ride::car::*;

use super::*;

pub struct RepairStationModal {
    selected_service : ServiceId,
    font : Rc<Font>,
    modal : ModalPage,
    diagnosed_modal : ModalPage,
    diagnosed : bool,
    unfold_diagnosed : bool
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
            diagnosed : false,
            unfold_diagnosed : false
        }
    }
}

impl ServiceModal for RepairStationModal {
    fn unfold(&mut self, game : &Game) { 
        self.diagnosed = false;

        let repair_station = game.city_map.get_service::<RepairStation>(self.selected_service);

        let diagnose_cost = UIText::new(self.font.clone(), format!("DIAGNOSE FOR {}$", repair_station.get_diagnosis_cost()));
        self.modal.clear_controls();
        self.modal.add_control(Box::from(diagnose_cost), ControlProperties { position : IVec2::zero(), pivot : Pivot::Center, binding : Binding::Center });
        
        let mut fix_cost_items : Vec<UISelectorItem<CarSystem>> = Vec::new();
        let mut i = 0;
        for (system, fix_cost) in repair_station.repair_costs.iter() {
            let system_name = match system {
                CarSystem::Wheels => { "WHEELS" }
                CarSystem::Transmission => { "TRANSMISSION" }
                CarSystem::Chase => { "CHASE" }
                CarSystem::Engine => { "ENGINE" }
                CarSystem::Brake => { "BRAKE" }
                CarSystem::Starter => { "STARTER" }
            };

            let fix_cost_str = format!("FIX {} FOR {}$", system_name, fix_cost);
            let fix_cost_control = Box::from(UIText::new(self.font.clone(), fix_cost_str));

            fix_cost_items.push(UISelectorItem::new(
                fix_cost_control, 
                ControlProperties { position : IVec2::new(0, 0 - i * 20), pivot : Pivot::Center, binding : Binding::Center },
                *system
            ));
            i += 1;
        }

        let pointer_image = Game::load_image_rgba("ui/pointer.png");
        let pointer_offset = IVec2::new(-(pointer_image.width() as isize), 0);
        let system_fix_selector = UISelector::new(fix_cost_items, SelectionType::Vertical, pointer_image, pointer_offset, SCREEN_RESOLUTION, None);

        self.diagnosed_modal.clear_controls();
        self.diagnosed_modal.add_control(
            Box::from(system_fix_selector), 
            ControlProperties { position : IVec2::zero(), pivot : Pivot::Center, binding : Binding::Center }
        );

        self.modal.start_anim_unfold(1000.0);
    }
    
    fn update(&mut self, game : &Game, input : &Vec<(InputEvent, EventType)>, delta_time : f32) -> Vec<ServiceModalEvent> {
        if self.diagnosed && self.unfold_diagnosed && self.modal.anim_state == ModalAnim::Void {
            self.unfold_diagnosed = false;
            self.diagnosed_modal.start_anim_unfold(1000.0);
        }

        for (event, event_type) in input {
            match (event, event_type) {
                (InputEvent::UIUp, EventType::Pressed) => { 
                    if self.diagnosed {
                        unsafe {
                            let ui_select = &mut *(self.diagnosed_modal.get_control_mut(0) as *mut dyn UIControl as *mut UISelector<CarSystem>);
                            ui_select.select_next_in_direction(&IVec2::new(0, 1));
                        }
                    }
                }
                (InputEvent::UIDown, EventType::Pressed) => { 
                    if self.diagnosed {
                        unsafe {
                            let ui_select = &mut *(self.diagnosed_modal.get_control_mut(0) as *mut dyn UIControl as *mut UISelector<CarSystem>);
                            ui_select.select_next_in_direction(&IVec2::new(0, -1));
                        }
                    }
                }
                (InputEvent::UISelect, EventType::Pressed) => { 
                    if !self.diagnosed {
                        self.modal.start_anim_fold(1000.0);
                        self.diagnosed = true;
                        self.unfold_diagnosed = true;
                        // TODO : pay for diagnosis.
                    } else {
                        let selected = unsafe {
                            let ui_select = &mut *(self.diagnosed_modal.get_control_mut(0) as *mut dyn UIControl as *mut UISelector<CarSystem>);
                            ui_select.select_current()
                        };

                        let system_name = match selected {
                            CarSystem::Wheels => { "WHEELS" }
                            CarSystem::Transmission => { "TRANSMISSION" }
                            CarSystem::Chase => { "CHASE" }
                            CarSystem::Engine => { "ENGINE" }
                            CarSystem::Brake => { "BRAKE" }
                            CarSystem::Starter => { "STARTER" }
                        };
    
                        println!("fixed {}", system_name);

                        return vec![
                            ServiceModalEvent::UIEvent(
                                // TODO : determine percent?
                                UIEvent::ServiceAction(self.selected_service, ServiceAction::FixCarSystem(selected, Percent(100.0)))
                            )
                        ]; 
                    }
                }
                (InputEvent::UIBack, EventType::Pressed) => { 
                    if self.diagnosed { &mut self.diagnosed_modal } else { &mut self.modal }.start_anim_fold(1000.0);
                    
                    return vec![ServiceModalEvent::Close];
                }
                _ => { }
            }
        }
        
        self.modal.update(delta_time);
        self.diagnosed_modal.update(delta_time);

        Vec::new()
    }

    fn select_service(&mut self, id: ServiceId) { self.selected_service = id; }

    fn is_busy(&self) -> bool { self.modal.anim_state != ModalAnim::Void }

    fn draw(&self, buffer : &mut RgbImage) { 
        self.modal.draw(buffer); 
        self.diagnosed_modal.draw(buffer);
    }
}