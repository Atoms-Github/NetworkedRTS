

use serde::*;
use anymap::AnyMap;
use crate::ecs::comp_store::*;
use serde::ser::SerializeStruct;
use serde::de::Visitor;

#[derive(Debug)]
pub struct SuperbEcs{
    systems: Vec<System>,
    pub c: CompStorage,
    debug_times: EcsDebugTimer,
}
impl SuperbEcs{
    pub fn new(systems: Vec<System>) -> Self{
        Self{
            systems,
            c: Default::default(),
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

        if meta.quality == SimQuality::DETERMA && rand::thread_rng().gen_bool(0.1) && crate::DEBUG_MSGS_ITS_LAGGING{
            self.debug_times.print_all();
            println!("Entities: {}", self.c.get_entity_count());
        }
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







































// ------------------------------
//  GARBAGE BELOW HERE.
// ------------------------------

impl Clone for System {
    fn clone(&self) -> Self {
        Self{
            run: self.run,
            name: self.name.clone(),
        }
    }
}

impl Serialize for SuperbEcs{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = bincode::serialize(&self.c).unwrap();
        serializer.serialize_bytes(bytes.as_slice())
        // let mut state = serializer.serialize_struct("Ecs", 1)?;
        // state.serialize_field("storage", &self.comp_storage)?;
        // state.end()
    }
}



struct ECSVisitor {
}

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

impl ECSVisitor {
    fn new() -> Self {
        ECSVisitor {}
    }
}
trait TraitTest{

}
impl<'de> Visitor<'de> for ECSVisitor
{
    type Value = SuperbEcs;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("An ECS")
    }

    fn visit_bytes<E>(self, bytes: &[u8]) -> Result<Self::Value, E>
    {
        let comp_store = bincode::deserialize::<CompStorage>(bytes).unwrap();

        return Ok(SuperbEcs{
            systems: crate::rts::compsys::jigsaw::jigsaw_game_state::global_get_systems(),
            c: comp_store,
            debug_times: Default::default()
        });
    }
}

impl<'de> Deserialize<'de> for SuperbEcs
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(ECSVisitor::new())
    }
}