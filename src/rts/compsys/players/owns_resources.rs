use crate::netcode::InputState;
use ggez::event::{KeyCode, MouseButton};
use crate::rts::compsys::RtsMouseEvent::{NoMouse, MouseUp};
use crate::rts::compsys::RtsKeyEvent::NoKey;
use crate::pub_types::PointFloat;

pub const RESOURCES_COUNT: usize = 3;

pub struct OwnsResourcesComp {
    pub resources: [i32; RESOURCES_COUNT]
}
impl OwnsResourcesComp{
    pub fn gain(&mut self, res_type: ResourceType, amount: i32){
        self.resources[res_type as usize] += amount;
    }
    pub fn pay(&mut self, res_type: ResourceType, amount: i32){
        self.resources[res_type as usize] -= amount;
    }
    pub fn try_pay(&mut self, res_type: ResourceType, amount: i32) -> bool{
        if self.get_count(res_type) >= amount{
            self.resources[res_type as usize] -= amount;
            return true;
        }
        return false;
    }
    pub fn get_count(&mut self, res_type: ResourceType) -> i32{
        return self.resources[res_type as usize];
    }
    pub fn get_counti(&mut self, res_type: usize) -> i32{
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