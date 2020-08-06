use std::rc::Rc;

use image::{RgbImage, RgbaImage, Rgb};

use crate::engine::common::{IVec2, Math};
use crate::engine::ui::font::*;
use crate::engine::ui::*;
use crate::engine::ui::selector_menu::*;
use crate::game::*;
use crate::game::city_map::services::*;
use crate::game::ui::{UIEvent, Screen};

use super::UIScreen;

#[derive(Copy, Clone)]
enum MenuEvents {
    Refuel(usize),
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
    gas_stations : Vec<ServiceId>,
    game : Option<Rc<Game>>,
    state : State,
    buy_gas_amount : u32,
    selected_station : ServiceId,
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
            MenuEvents::Refuel(0)
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
            gas_stations : Vec::new(), 
            buy_gas_modal, 
            state : State::SelectingGasStation, 
            buy_gas_amount : 0, 
            selected_station : ServiceId(0),
            game : None,
            font : font.clone() 
        }
    }

    fn get_max_gas_amount(&self) -> u32 {
        let gas_station = self.game.as_ref().unwrap().city_map.get_gas_station(self.selected_station);
        let player_money = self.game.as_ref().unwrap().player.money;
        gas_station.get_max_gas_amount(player_money)
    }

    fn get_gas_cost(&self, amount : u32) -> f32 {
        let gas_station = self.game.as_ref().unwrap().city_map.get_gas_station(self.selected_station);
        gas_station.get_cost(amount)
    }

    fn referesh_modal(&mut self, delta_time : f32) {
        self.buy_gas_modal.clear_controls();
        self.buy_gas_amount = Math::min(self.get_max_gas_amount(), self.buy_gas_amount);
        let cost = self.get_gas_cost(self.buy_gas_amount);
        let buy_string = self.buy_gas_amount.to_string() + "L. FOR " + cost.to_string().as_ref() + "$";
        let text = UIText::new(self.font.clone(), buy_string);
        let text_props = ControlProperties { binding : Binding::Center, pivot : Pivot::Center, position : IVec2::zero() };
        self.buy_gas_modal.add_control(Box::from(text), text_props);

        self.buy_gas_modal.update(delta_time);
    }
}

impl UIScreen for GasStationsScreen {
    fn init(&mut self, game : &Game) {
        unsafe { self.game = Some(Rc::from_raw(game as *const Game)); }
        self.gas_stations = game.city_map.get_current_city_services().gas_stations
        .iter()
        .map(|gs| gs.0)
        .collect();
    }

    fn update(&mut self, input : &Vec<(InputEvent, EventType)>, delta_time : f32) -> Vec<UIEvent> {
        self.referesh_modal(delta_time);

        match self.state {
            State::SelectingGasStation => {

                for (event, event_type) in input {
                    match (event, event_type) {
                        (InputEvent::UIDown, EventType::Pressed) => { self.menu.select_next_in_direction(&IVec2::new(0, -1)); }
                        (InputEvent::UIUp, EventType::Pressed) => { self.menu.select_next_in_direction(&IVec2::new(0, 1)); }
                        (InputEvent::UISelect, EventType::Pressed) => { 
                            let menu_event = self.menu.select_current();
                            match menu_event {
                                MenuEvents::Refuel(station_id) => { 
                                    self.selected_station = self.gas_stations[station_id];
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
                                self.buy_gas_amount -= 1;
                            } 
                        }
                        (InputEvent::UIUp, EventType::Pressed) => { 
                            if self.buy_gas_amount < self.get_max_gas_amount() {
                                self.buy_gas_amount += 1; 
                            }
                        }
                        (InputEvent::UISelect, EventType::Pressed) => { 
                            return vec![UIEvent::ServiceAction(ServiceAction::BuyGas(self.buy_gas_amount, self.selected_station))]; 
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