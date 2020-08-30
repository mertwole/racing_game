use std::rc::Rc;

use crate::engine::common::{IVec2, Math};
use crate::engine::ui::font::*;
use crate::engine::ui::*;
use crate::game::*;
use crate::game::city_map::services::*;

use super::*;

pub struct HostelModal {
    selected_service : ServiceId,
    font : Rc<Font>,
    resolution : IVec2,
    modal : ModalPage
}

impl HostelModal {
    pub fn new(resolution : &IVec2, font : Rc<Font>) -> HostelModal {
        let modal = ModalPage::new(IVec2::new(100, 100), IVec2::new(200, 100), Some(Rgb([150, 150, 150])));
        HostelModal { 
            selected_service : ServiceId(0),
            font,
            modal,
            resolution : resolution.clone()
        }
    }
}

#[derive(Clone)]
struct OptionSelect(usize);

impl ServiceModal for HostelModal {
    fn unfold(&mut self, game : &Game) { 
        let pointer_image = Game::load_image_rgba("ui/pointer.png");
        let mut menu_items = Vec::new();

        let hostel = game.city_map.get_service::<Hostel>(self.selected_service);
        for i in 0..hostel.options.len() {
            let time = hostel.options[i].time.clone();
            let cost = hostel.options[i].cost;
            menu_items.push(UISelectorItem::new(
                Box::from(UIText::new(self.font.clone(), format!("REST {}H. {}M. FOR {}$", time.hr, time.min, cost))),
                ControlProperties::new(IVec2::new(0, -(i as isize) * 20), Pivot::Center, Binding::Center),
                OptionSelect(i)
            ));
        }

        let pointer_offset = IVec2::new(-(pointer_image.width() as isize), 0);
        let option_selector = UISelector::<OptionSelect>::new(menu_items, SelectionType::Vertical, pointer_image, pointer_offset, self.resolution.clone(), None);
        self.modal.clear_controls();
        self.modal.add_control(Box::from(option_selector), ControlProperties { position : IVec2::zero(), pivot : Pivot::LeftBottom, binding : Binding::LeftBottom });

        self.modal.start_anim_unfold(1000.0); 
    }
    
    fn update(&mut self, game : &Game, input : &Vec<(InputEvent, EventType)>, delta_time : f32) -> Vec<ServiceModalEvent> {
        let hostel = game.city_map.get_service::<Hostel>(self.selected_service);
        let player_money = game.player.money;

        for (event, event_type) in input {
            match (event, event_type) {
                (InputEvent::UIDown, EventType::Pressed) => { 
                    unsafe {
                        let ui_select = &mut *(self.modal.get_control_mut(0) as *mut dyn UIControl as *mut UISelector<OptionSelect>);
                        ui_select.select_next_in_direction(&IVec2::new(0, -1));
                    }
                }
                (InputEvent::UIUp, EventType::Pressed) => { 
                    unsafe {
                        let ui_select = &mut *(self.modal.get_control_mut(0) as *mut dyn UIControl as *mut UISelector<OptionSelect>);
                        ui_select.select_next_in_direction(&IVec2::new(0, 1));
                    }
                }
                (InputEvent::UISelect, EventType::Pressed) => { 
                    let selected = unsafe {
                        let ui_select = &mut *(self.modal.get_control_mut(0) as *mut dyn UIControl as *mut UISelector<OptionSelect>);
                        ui_select.select_current()
                    };

                    return vec![
                        ServiceModalEvent::UIEvent(
                            UIEvent::ServiceAction(self.selected_service, ServiceAction::RestInHostel(selected.0 as u32))
                        )
                    ]; 
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

    fn select_service(&mut self, id: ServiceId) { self.selected_service = id; }

    fn is_busy(&self) -> bool { self.modal.anim_state != ModalAnim::Void }

    fn draw(&self, buffer : &mut RgbImage) { self.modal.draw(buffer); }
}