use crate::*;
use ggez::event::{KeyCode, MouseButton};

use crate::bibble::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TechTreeComp {
    pub tree: GameData,
}

pub trait MyGlobalEntityID{
    fn get_owner_tech_tree<'a>(&self, c: &'a CompStorage) -> &'a mut GameData;
    fn get_player_tech_tree<'a>(&self, c: &'a CompStorage) -> &'a mut GameData;

}
impl MyGlobalEntityID for GlobalEntityID{
    fn get_owner_tech_tree<'a>(&self, c: &'a CompStorage) -> &'a mut GameData{
        let owner = c.get::<OwnedComp>(*self).unwrap().owner;
        return &mut c.get_mut::<TechTreeComp>(owner).unwrap().tree;
    }
    fn get_player_tech_tree<'a>(&self, c: &'a CompStorage) -> &'a mut GameData{
        return &mut c.get_mut::<TechTreeComp>(*self).unwrap().tree;
    }
}




/*
    pub fn register_mouse_click_event(&mut self, button: MouseButton, is_down: bool){
        self.active_snapshot.mouse_btns_pressed[get_button_index(button)] = is_down;
        if is_down && self.waiting_on_mouse.is_none(){
            self.waiting_on_mouse = Some(button);
            self.active_snapshot.events.push(InputEvent::MouseDown(button));
        }else if !is_down && self.waiting_on_mouse == Some(button){
            self.waiting_on_mouse = None;
            self.active_snapshot.events.push(InputEvent::MouseUp());
        }
    }
    pub fn register_key_event(&mut self, key: KeyCode, is_down: bool, repeat: bool){
        self.active_snapshot.keys_pressed[key as usize] = is_down;
        if is_down && self.waiting_on_key.is_none(){
            self.waiting_on_key = Some(key);
            self.active_snapshot.events.push(InputEvent::KeyDown(key, repeat));
        }else if !is_down && self.waiting_on_key == Some(key){
            self.waiting_on_key = None;
            self.active_snapshot.events.push(InputEvent::KeyUp());
        }
    }
 */