use ggez::event::{KeyCode, MouseButton};
use bibble::::RtsMouseEvent::{NoMouse, MouseUp};
use bibble::::RtsKeyEvent::NoKey;
use game::pub_types::PointFloat;

use bibble::::*;
use std::collections::HashMap;
use std::hash::Hash;

pub const RESOURCES_COUNT: usize = 3;
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct ResourceBlock<T : Hash + Serialize + DeserializeOwned + Copy> {
    pub resources: HashMap<T, f32>
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct OwnsResourcesComp<T : Hash + Serialize + DeserializeOwned + Copy> {
    resources: ResourceBlock<T>
}
impl<T : Hash + Serialize + DeserializeOwned + Copy> OwnsResourcesComp<T> {
    pub fn gain_block(&mut self, block: &ResourceBlock<T>, delta: f32){
        for (k, v) in &block.resources{
            *self.resources.resources.entry(k).or_insert(0.0) += v;
        }
    }
    pub fn gain(&mut self, res_type: T, amount: f32){
        *self.resources.resources.entry(&res_type).or_insert(0.0) += v;
    }
    pub fn pay(&mut self, res_type: T, amount: f32){
        *self.resources.resources.entry(&res_type).or_insert(0.0) -= v;
    }

    pub fn try_pay(&mut self, res_type: T, amount: f32) -> bool{
        if self.get_count(res_type) >= amount{
            self.pay(res_type, amount);
            return true;
        }
        return false;
    }
    pub fn zeroify(&mut self){
        for (k, v) in &mut self.resources.resources{
            *v = 0.0;
        }
    }
    pub fn get_count(&mut self, res_type: T) -> f32{
        return *self.resources.resources.entry(&res_type).or_insert(0.0);
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ResourceType{
    LIGHTNESS,
    DARKNESS,
    BLUENESS,
}