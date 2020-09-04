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
    resolution : IVec2,
    pub modal : ModalPage
}

#[derive(Clone)]
struct ProductSelect(usize);

impl ShopModal {
    pub fn new(resolution : &IVec2, font : Rc<Font>) -> ShopModal {
        let modal = ModalPage::new(IVec2::new(100, 100), IVec2::new(200, 100), Some(Rgb([150, 150, 150])));
        ShopModal { 
            buy_gas_amount : 0,
            selected_service : ServiceId(0),
            font,
            resolution : *resolution,
            modal
        }
    }
}

impl ServiceModal for ShopModal {  
    fn unfold(&mut self, game : &Game) { 
        let shop = game.city_map.get_service::<Shop>(self.selected_service);
        let mut products = Vec::new();

        for i in 0..shop.assortment.len() {
            let product = &shop.assortment[i];
            let product_name = match product.product_type {
                ProductType::Water(size) => { format!("WATER {}L. FOR {}$", size, product.cost) }
                ProductType::Soda(size) => { format!("SODA {}L. FOR {}$", size, product.cost) }
                ProductType::Food(size) => { format!("FOOD {}KG. FOR {}$", size, product.cost) }
            };

            products.push(UISelectorItem::new(
                Box::from(UIText::new(self.font.clone(), product_name)),
                ControlProperties::new(IVec2::new(0, -(i as isize) * 20), Pivot::Center, Binding::Center),
                ProductSelect(i)
            ));
        }

        let pointer_image = Game::load_image_rgba("ui/pointer.png");
        let pointer_offset = IVec2::new(-(pointer_image.width() as isize), 0);
        let option_selector = UISelector::<ProductSelect>::new(products, SelectionType::Vertical, pointer_image, pointer_offset, self.resolution.clone(), None);
        self.modal.clear_controls();
        self.modal.add_control(Box::from(option_selector), ControlProperties { position : IVec2::zero(), pivot : Pivot::LeftBottom, binding : Binding::LeftBottom });

        self.modal.start_anim_unfold(1000.0); 
    }

    fn update(&mut self, game : &Game, input : &Vec<(InputEvent, EventType)>, delta_time : f32) -> Vec<ServiceModalEvent> {
        self.modal.update(delta_time);

        for (event, event_type) in input {
            match (event, event_type) {
                (InputEvent::UIDown, EventType::Pressed) => { 
                    unsafe {
                        let ui_select = &mut *(self.modal.get_control_mut(0) as *mut dyn UIControl as *mut UISelector<ProductSelect>);
                        ui_select.select_next_in_direction(&IVec2::new(0, -1));
                    }
                }
                (InputEvent::UIUp, EventType::Pressed) => { 
                    unsafe {
                        let ui_select = &mut *(self.modal.get_control_mut(0) as *mut dyn UIControl as *mut UISelector<ProductSelect>);
                        ui_select.select_next_in_direction(&IVec2::new(0, 1));
                    }
                }
                (InputEvent::UISelect, EventType::Pressed) => { 
                    let selected = unsafe {
                        let ui_select = &mut *(self.modal.get_control_mut(0) as *mut dyn UIControl as *mut UISelector<ProductSelect>);
                        ui_select.select_current()
                    };

                    return vec![
                        ServiceModalEvent::UIEvent(
                            UIEvent::ServiceAction(self.selected_service, ServiceAction::BuyProduct(selected.0))
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

        Vec::new()
    }

    fn select_service(&mut self, id: ServiceId) { self.selected_service = id; }

    fn is_busy(&self) -> bool { self.modal.anim_state != ModalAnim::Void }

    fn draw(&self, buffer : &mut RgbImage) { self.modal.draw(buffer); }
}