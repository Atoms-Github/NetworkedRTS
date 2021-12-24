

use serde::*;
use anymap::AnyMap;
use crate::ecs::comp_store::*;
use serde::ser::SerializeStruct;
use serde::de::Visitor;
use std::fmt::{Write, Debug};
use std::fmt;
use crate::utils::{TypeIdNum, gett};
use crate::rts::compsys::jigsaw::jigsaw_game_state::UsingRenderResources;
use std::marker::PhantomData;
use crate::ecs::GlobalEntityID;
use std::slice::Iter;
use crate::ecs::pending_entity::PendingEntity;
use crate::pub_types::{SimMetadata, SimQuality};
use crate::ecs::ecs_debug_timer::EcsDebugTimer;
use rand::Rng;
use std::hash::{Hash, Hasher};
use crate::bibble::data::data_types::__private::Formatter;
use std::collections::HashMap;
use crate::ecs::bblocky::comp_registration::{SuperbFunctions, FunctionMap};

#[derive(Debug)]
pub struct SuperbEcs<C>{
    pub c: CompStorage<C>,
    debug_times: EcsDebugTimer,
}
impl<C : EcsConfig> SuperbEcs<C>{
    pub fn new() -> Self{
        Self{
            c: Default::default(),
            debug_times: Default::default()
        }
    }
    pub fn sim_systems(&mut self, meta: &SimMetadata){
        let mut pending_changes = EntStructureChanges{
            new_entities: vec![],
            deleted_entities: vec![]
        };

        for system in &self.systems{
            if meta.quality == SimQuality::DETERMA{
                self.debug_times.start_timer(String::from(system.name));
            }
            (system.run)(&mut self.c, &mut pending_changes, meta);

            if meta.quality == SimQuality::DETERMA{
                self.debug_times.stop_timer(String::from(system.name));
            }
        }
        pending_changes.apply(&mut self.c);

        if meta.quality == SimQuality::DETERMA && rand::thread_rng().gen_bool(0.1) && crate::DEBUG_MSGS_ITS_LAGGING{
            self.debug_times.print_all();
            println!("Entities: {}", self.c.get_entity_count());
        }
    }

}
impl<C> Hash for SuperbEcs<C>{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.c.hash(state);
    }
}
pub struct System<C>{
    pub run: fn(&mut CompStorage<C> /* Could add read only version here. */, &mut EntStructureChanges<C>, &SimMetadata),
    pub name: &'static str,
}
impl<C> Debug for System<C>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("System").field("Name: ", &self.name).finish()
    }
}
pub struct EntStructureChanges<C>{
    pub new_entities: Vec<PendingEntity<C>>,
    pub deleted_entities: Vec<GlobalEntityID>,
}
impl<C> EntStructureChanges<C>{
    pub fn apply(self, storage: &mut CompStorage<C>){
        for new_entity in self.new_entities{
            storage.create_entity(new_entity);
        }
        for del_entity in self.deleted_entities{
            storage.delete_entity(del_entity);
        }
    }
    pub fn move_into(self, other: &mut Self){
        for new_entity in self.new_entities{
            other.new_entities.push(new_entity);
        }
        for del_entity in self.deleted_entities{
            other.deleted_entities.push(del_entity);
        }
    }
}

pub trait EcsConfig{
    fn get_systems<C>() -> Vec<System<C>>;
    fn get_functions() -> &'static FunctionMap;
}









