use crate::netcode::InputState;
use ggez::event::{KeyCode, MouseButton};
use crate::rts::compsys::RtsMouseEvent::{NoMouse, MouseUp};
use crate::rts::compsys::RtsKeyEvent::NoKey;
use crate::pub_types::PointFloat;

use crate::rts::compsys::*;

pub const RESOURCES_COUNT: usize = 3;
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct ResourceBlock {
    pub resource_counts: [f32; RESOURCES_COUNT],
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct OwnsResourcesComp {
    pub resources: [f32; RESOURCES_COUNT]
}
impl OwnsResourcesComp{
    pub fn gain_block(&mut self, block: &ResourceBlock, delta: f32){
        for i in 0..RESOURCES_COUNT{
            self.gaini(i, block.resource_counts[i] * delta);
        }
    }
    pub fn gaini(&mut self, res_type: usize, amount: f32){
        self.resources[res_type as usize] += amount;
    }
    pub fn gain(&mut self, res_type: ResourceType, amount: f32){
        self.gaini(res_type as usize, amount);
    }
    pub fn payi(&mut self, res_type: usize, amount: f32){
        self.resources[res_type as usize] -= amount;
    }
    pub fn pay(&mut self, res_type: ResourceType, amount: f32){
        self.payi(res_type as usize, amount);
    }

    pub fn try_pay(&mut self, res_type: ResourceType, amount: f32) -> bool{
        if self.get_count(res_type) >= amount{
            self.resources[res_type as usize] -= amount;
            return true;
        }
        return false;
    }
    pub fn reset(&mut self){
        for i in 0..RESOURCES_COUNT{
            self.resources[i] = 0.0;
        }
    }
    pub fn get_count(&mut self, res_type: ResourceType) -> f32{
        return self.resources[res_type as usize];
    }
    pub fn get_counti(&mut self, res_type: usize) -> f32{
        return self.resources[res_type];
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ResourceType{
    LIGHTNESS,
    DARKNESS,
    BLUENESS,
}