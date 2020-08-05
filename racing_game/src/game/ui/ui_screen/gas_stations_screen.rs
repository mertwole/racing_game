use std::rc::Rc;

use image::{RgbImage, RgbaImage, Rgb};

use crate::engine::common::{IVec2};
use crate::engine::ui::font::*;
use crate::engine::ui::*;
use crate::engine::ui::selector_menu::*;
use crate::game::*;
use crate::game::city_map::services::*;
use crate::game::ui::{UIEvent, Screen};

use super::UIScreen;

#[derive(Copy, Clone)]
enum MenuEvents {
    Refuel,
    Back
}

enum State{
    SelectingGasStation,
    OpeningModalWindow,
    ByingGas,
    ClosingModalWindow
}

pub struct GasStationsScreen{
    menu : SelectorMenu<MenuEvents>,
    buy_gas_modal : ModalPage,
    player : Option<Player>,
    gas_stations : Vec<ServiceId>,
    state : State,
    buy_gas_amount : u32,
    font : Rc<Font>
}

impl GasStationsScreen {
    pub fn new(resolution : &IVec2, font : Rc<Font>) -> GasStationsScreen {
        let pointer_image = Game::load_image_rgba("ui/pointer.png");

        let gas_stations_label = UIText::new(font.clone(), String::from("REFUEL"));
        let next_label = UIText::new(font.clone(), String::from("BACK"));

        let refuel_item = MenuItem::new(
            Box::from(gas_stations_label), 
            ControlProperties { 
                pivot : Pivot::LeftTop, 
                position : IVec2::new(20, -20), 
                binding : Binding::LeftTop 
            }, 
            MenuEvents::Refuel
        );

        let back_item = MenuItem::new(
            Box::from(next_label), 
            ControlProperties { 
                pivot : Pivot::LeftBottom, 
                position : IVec2::new(20, 20), 
                binding : Binding::LeftBottom 
            }, 
            MenuEvents::Back
        );

        let menu = SelectorMenu::new(vec![refuel_item, back_item], pointer_image, resolution.clone());

        let buy_gas_modal = ModalPage::new(IVec2::new(100, 100), IVec2::new(200, 100), Some(Rgb([150, 150, 150])));

        GasStationsScreen { 
            menu, 
            player : None, 
            gas_stations : Vec::new(), 
            buy_gas_modal, 
            state : State::SelectingGasStation, 
            buy_gas_amount : 0, 
            font : font.clone() 
        }
    }

    fn referesh_modal(&mut self) {
        self.buy_gas_modal.clear_controls();
        let text = UIText::new(self.font.clone(), self.buy_gas_amount.to_string());
        let mut text_props = ControlProperties { binding : Binding::Center, pivot : Pivot::Center, position : IVec2::zero() };
        self.buy_gas_modal.add_control(Box::from(text), text_props);
    }
}

impl UIScreen for GasStationsScreen {
    fn init(&mut self, game : &Game) {
        self.player = Some(game.player.clone());
        self.gas_stations = game.city_map.get_current_city_services().gas_stations
        .iter()
        .map(|gs| gs.0)
        .collect();

        self.referesh_modal();
    }

    fn update(&mut self, input : &Vec<(InputEvent, EventType)>, delta_time : f32) -> Vec<UIEvent> {
        self.buy_gas_modal.update(delta_time);

        match self.state {
            State::SelectingGasStation => {

                for (event, event_type) in input {
                    match (event, event_type) {
                        (InputEvent::UIDown, EventType::Pressed) => { self.menu.select_next_in_direction(&IVec2::new(0, -1)); }
                        (InputEvent::UIUp, EventType::Pressed) => { self.menu.select_next_in_direction(&IVec2::new(0, 1)); }
                        (InputEvent::UISelect, EventType::Pressed) => { 
                            let menu_event = self.menu.select_current();
                            match menu_event {
                                MenuEvents::Refuel => { 
                                    self.buy_gas_modal.start_anim_unfold(100.0);
                                    self.state = State::OpeningModalWindow;
                                },
                                MenuEvents::Back => { 
                                    return vec![UIEvent::ChangeScreen(Screen::Services)]; 
                                } 
                            }
                        }
                        _ => { }
                    }
                }

            }
            State::ByingGas => {

                for (event, event_type) in input {
                    match (event, event_type) {
                        (InputEvent::UIDown, EventType::Pressed) => { 
                            if self.buy_gas_amount >= 1 { 
                                self.buy_gas_amount -= 1; self.referesh_modal(); 
                            } 
                        }
                        (InputEvent::UIUp, EventType::Pressed) => { 
                            self.buy_gas_amount += 1; 
                            self.referesh_modal();
                        }
                        (InputEvent::UISelect, EventType::Pressed) => { 
                            return vec![UIEvent::ServiceAction(ServiceAction::BuyGas(self.buy_gas_amount, self.gas_stations[0]))]; 
                        }
                        (InputEvent::UIBack, EventType::Pressed) => { 
                            self.buy_gas_modal.start_anim_fold(100.0);
                            self.state = State::ClosingModalWindow;
                        }
                        _ => { }
                    }
                }
            }
            State::OpeningModalWindow => {
                if self.buy_gas_modal.anim_state == ModalAnim::Void { self.state = State::ByingGas; }
            }
            State::ClosingModalWindow => {
                if self.buy_gas_modal.anim_state == ModalAnim::Void { self.state = State::SelectingGasStation; }
            }

            _ => { }
        }

        Vec::new()
    }  

    fn render(&self, buffer : &mut RgbImage) {
        self.menu.render(buffer);
        self.buy_gas_modal.draw(buffer);
    }
}