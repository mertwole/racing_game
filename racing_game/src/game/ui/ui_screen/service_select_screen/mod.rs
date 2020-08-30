use std::rc::Rc;
use std::any::TypeId;

use image::{RgbImage, RgbaImage, Rgb};

use crate::engine::common::{IVec2, Math};
use crate::engine::ui::font::*;
use crate::engine::ui::*;
use crate::game::*;
use crate::game::city_map::services::*;
use crate::game::ui::{UIEvent, Screen};

use super::UIScreen;

mod gas_station_modal;
mod hostel_modal;
mod shop_modal;
mod repair_station_modal;

use gas_station_modal::*;
use hostel_modal::*;
use shop_modal::*;
use repair_station_modal::*;

pub trait ServiceModal { 
    fn update(&mut self, game : &Game, input : &Vec<(InputEvent, EventType)>, delta_time : f32) -> Vec<ServiceModalEvent>;
    fn select_service(&mut self, id : ServiceId);
    fn unfold(&mut self, game : &Game);
    fn is_busy(&self) -> bool;
    fn draw(&self, buffer : &mut RgbImage);
}

pub enum ServiceModalEvent {
    Close,
    UIEvent(UIEvent)
}

#[derive(Copy, Clone)]
enum MenuEvents {
    Select(ServiceId),
    Back
}

#[derive(PartialEq)]
enum State{
    SelectingService,
    OpeningModalWindow,
    ActionsInService,
    ClosingModalWindow
}

pub struct ServiceSelectScreen<T> where T : Service{
    _type : std::marker::PhantomData<T>,
    menu : Option<UISelector<MenuEvents>>,
    service_modal : Box<dyn ServiceModal>,
    game : Option<Rc<Game>>,
    state : State,
    font : Rc<Font>,
    resolution : IVec2
}

impl<T> ServiceSelectScreen<T> where T : Service {
    pub fn new(resolution : &IVec2, font : Rc<Font>) -> ServiceSelectScreen<T> where T : Service + 'static {
        let service_id = TypeId::of::<T>();

        let service_modal = if service_id == TypeId::of::<GasStation>() {
            Box::<dyn ServiceModal>::from(Box::from(GasStationModal::new(resolution, font.clone())))
        } else if service_id == TypeId::of::<Hostel>() {
            Box::<dyn ServiceModal>::from(Box::from(HostelModal::new(resolution, font.clone())))
        } else if service_id == TypeId::of::<RepairStation>() {
            Box::<dyn ServiceModal>::from(Box::from(RepairStationModal::new(resolution, font.clone())))
        } else if service_id == TypeId::of::<Shop>() {
            Box::<dyn ServiceModal>::from(Box::from(ShopModal::new(resolution, font.clone())))
        } else { panic!("incorrect service type") };

        ServiceSelectScreen::<T> { 
            _type : std::marker::PhantomData::<T>,
            menu : None,
            service_modal, 
            state : State::SelectingService,
            game : None,
            font,
            resolution : resolution.clone()
        }
    }
}

impl<T> UIScreen for ServiceSelectScreen<T> where T : Service + 'static {
    fn init(&mut self, game : &Game) {
        unsafe { self.game = Some(Rc::from_raw(game as *const Game)); }
        let service_ids = game.city_map.get_current_city_services_subset().get_of_type::<T>();

        let mut menu_items = Vec::new();

        let mut i = 0;
        for id in service_ids {
            let service = self.game.as_ref().unwrap().city_map.get_service::<T>(id);
            let logo = UIImage::new(service.get_logo());
            let station_item = UISelectorItem::new(
                Box::from(logo), 
                ControlProperties { 
                    pivot : Pivot::LeftTop, 
                    position : IVec2::new(20, -20 - 20 * i), 
                    binding : Binding::LeftTop 
                }, 
                MenuEvents::Select(id)
            );

            menu_items.push(station_item);

            i += 1;
        }

        let back_label = UIText::new(self.font.clone(), String::from("BACK"));
        let back_item = UISelectorItem::new(
            Box::from(back_label), 
            ControlProperties { 
                pivot : Pivot::LeftBottom, 
                position : IVec2::new(20, 20), 
                binding : Binding::LeftBottom 
            }, 
            MenuEvents::Back
        );

        menu_items.push(back_item);

        let pointer_image = Game::load_image_rgba("ui/pointer.png");
        let pointer_offset = IVec2::new(-(pointer_image.width() as isize), 0);
        self.menu = Some(UISelector::new(menu_items, SelectionType::Vertical, pointer_image, pointer_offset, self.resolution.clone(), None));
    }

    fn update(&mut self, input : &Vec<(InputEvent, EventType)>, delta_time : f32) -> Vec<UIEvent> {
        let void_input = Vec::new();
        let service_events = self.service_modal.as_mut().update(
            self.game.as_ref().unwrap(), 
            if self.state == State::ActionsInService { input } else { &void_input }, 
            delta_time
        );

        match self.state {
            State::SelectingService => {

                for (event, event_type) in input {
                    match (event, event_type) {
                        (InputEvent::UIDown, EventType::Pressed) => { self.menu.as_mut().unwrap().select_next_in_direction(&IVec2::new(0, -1)); }
                        (InputEvent::UIUp, EventType::Pressed) => { self.menu.as_mut().unwrap().select_next_in_direction(&IVec2::new(0, 1)); }
                        (InputEvent::UISelect, EventType::Pressed) => { 
                            let menu_event = self.menu.as_mut().unwrap().select_current();
                            match menu_event {
                                MenuEvents::Select(id) => { 
                                    self.service_modal.select_service(id);
                                    self.service_modal.unfold(self.game.as_ref().unwrap());
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
            State::ActionsInService => {
                for event in service_events {
                    match event {
                        ServiceModalEvent::Close => { self.state = State::ClosingModalWindow }
                        ServiceModalEvent::UIEvent(ui_event) => { return vec![ui_event]; }
                    }
                }
            }
            State::OpeningModalWindow => {
                if !self.service_modal.is_busy() { 
                    self.state = State::ActionsInService; 
                }
            }
            State::ClosingModalWindow => {
                if !self.service_modal.is_busy() { 
                    self.state = State::SelectingService; 
                }
            }

            _ => { }
        }

        Vec::new()
    }  

    fn render(&self, buffer : &mut RgbImage) {
        self.menu.as_ref().unwrap().draw(buffer);
        self.service_modal.draw(buffer);
    }
}