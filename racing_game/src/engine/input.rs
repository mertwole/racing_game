use std::collections::HashMap;
use std::iter;

use crate::engine::window::*;

#[derive(Copy, Clone)]
pub enum EventType{
    Pressed,
    Released
}

pub struct Input<T : Sized + Copy + Clone>{
    key_bindings : HashMap<Key, Vec<T>>
}

impl<T : Sized + Copy + Clone> Input<T>{
    pub fn new() -> Input<T> {
        let key_bindings = HashMap::<Key, Vec<T>>::new();
        Input { key_bindings }
    }

    pub fn bind_action(&mut self, action : T, key : Key) {
        let binding = self.key_bindings.get_mut(&key);
        match binding {
            Some(actions) => { actions.push(action); }
            None => { self.key_bindings.insert(key, vec![action]); }
        }
    }

    pub fn process(&mut self, window : &mut Window) -> Vec<(T, EventType)> {
        window
        .get_events()
        .into_iter()
        .map(|(key, event_type)| (self.key_bindings.get(&key), event_type))
        .filter(|(actions, _)| actions.is_some())
        .map(|(actions, event_type)| ((*actions.unwrap()).clone(), event_type))
        .map(|(actions, event_type)| { let actions_len = actions.len(); actions.into_iter().zip(iter::repeat(event_type).take(actions_len)) })
        .fold(Vec::<(T, EventType)>::new(), |mut acc, x| { acc.append(&mut x.collect()); acc })
    }
}
