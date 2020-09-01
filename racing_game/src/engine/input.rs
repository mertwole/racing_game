use std::collections::HashMap;
use std::iter;

use crate::engine::window::*;

#[derive(Copy, Clone)]
pub enum EventType{
    Pressed,
    Released
}

pub struct Input<T : Sized + Copy + Clone + PartialEq>{
    key_bindings : HashMap<Key, Vec<T>>
}

impl<T : Sized + Copy + Clone + PartialEq> Input<T>{
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

    pub fn override_action_binding(&mut self, action : T, key : Key) {
        'outer : for (key, actions) in &mut self.key_bindings {
            for i in 0..actions.len() {
                if actions[i] == action { actions.remove(i); break 'outer; }
            }
        }

        self.bind_action(action, key);
    }

    pub fn get_action_key(&self, action : T) -> Option<Key> {
        for (key, actions) in &self.key_bindings {
            for act in actions {
                if *act == action { return Some(*key); }
            }
        }

        None
    }

    pub fn process(&mut self, events : Vec<(Key, EventType)>) -> Vec<(T, EventType)> {
        events
        .into_iter()
        .map(|(key, event_type)| (self.key_bindings.get(&key), event_type))
        .filter(|(actions, _)| actions.is_some())
        .map(|(actions, event_type)| ((*actions.unwrap()).clone(), event_type))
        .map(|(actions, event_type)| { let actions_len = actions.len(); actions.into_iter().zip(iter::repeat(event_type).take(actions_len)) })
        .fold(Vec::<(T, EventType)>::new(), |mut acc, x| { acc.append(&mut x.collect()); acc })
    }
}
