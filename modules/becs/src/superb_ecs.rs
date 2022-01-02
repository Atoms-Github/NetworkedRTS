

use serde::*;
use crate::comp_store::*;
use serde::ser::SerializeStruct;
use serde::de::Visitor;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use crate::ecs_debug_timer::EcsDebugTimer;
use crate::bblocky::comp_registration::EcsConfig;
use crate::pending_entity::PendingEntity;
use crate::GlobalEntityID;

use std::fmt;
use netcode::{SimMetadata, SimQuality};

#[derive(Debug, Serialize, Deserialize)]
pub struct SuperbEcs{
    #[serde(skip)]
    systems: Vec<System>,
    pub c: CompStorage,
    #[serde(skip)]
    debug_times: EcsDebugTimer,
}
impl SuperbEcs{
    pub fn post_deserialize(&mut self, config: EcsConfig){
        self.systems = config.systems.clone();
        self.c.post_deserialize(config);
    }
    pub fn new(config: EcsConfig) -> Self{
        Self{
            systems: config.systems,
            c: CompStorage::new(config.functions),
            debug_times: Default::default()
        }
    }
    pub fn set_systems(&mut self, systems: Vec<System>){
        self.systems = systems;
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

        self.debug_times.print_all();
    }

}
impl Hash for SuperbEcs{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.c.hash(state);
    }
}
impl Clone for SuperbEcs{
    fn clone(&self) -> Self {
        Self{
            systems: self.systems.clone(),
            c: self.c.clone(),
            debug_times: self.debug_times.clone()
        }
    }
}
#[derive(Clone)]
pub struct System{
    pub run: fn(&mut CompStorage /* Could add read only version here. */, &mut EntStructureChanges, &SimMetadata),
    pub name: &'static str,
}
impl Debug for System{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("System").field("Name: ", &self.name).finish()
    }
}
pub struct EntStructureChanges{
    pub new_entities: Vec<PendingEntity>,
    pub deleted_entities: Vec<GlobalEntityID>,
}
impl EntStructureChanges{
    pub fn apply(self, storage: &mut CompStorage){
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