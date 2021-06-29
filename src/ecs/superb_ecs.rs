

use serde::*;
use anymap::AnyMap;
use crate::ecs::comp_store::*;
use serde::ser::SerializeStruct;
use serde::de::Visitor;


// TODO: Implement ser and de manually.
pub struct SuperbEcs<R>{
    systems: Vec<System<R>>,
    pub c: CompStorage,
}
impl<R> SuperbEcs<R>{
    pub fn new(systems: Vec<System<R>>) -> Self{
        Self{
            systems,
            c: Default::default(),
        }
    }
    pub fn set_systems(&mut self, systems: Vec<System<R>>){
        self.systems = systems;
    }
    pub fn sim_systems(&mut self, resources: &R){
        let mut pending_changes = EntStructureChanges{
            new_entities: vec![],
            deleted_entities: vec![]
        };
        for system in &self.systems{
            (system.run)(resources, &mut self.c, &mut pending_changes);
        }
        pending_changes.apply(&mut self.c);
    }

}
impl<R> Clone for SuperbEcs<R>{
    fn clone(&self) -> Self {
        Self{
            systems: self.systems.clone(),
            c: self.c.clone(),
        }
    }
}

pub struct System<R>{
    pub run: fn(&R, &mut CompStorage /* Could add read only version here. */, &mut EntStructureChanges),
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
    }
}







































// ------------------------------
//  GARBAGE BELOW HERE.
// ------------------------------

impl<R> Clone for System<R> {
    fn clone(&self) -> Self {
        Self{
            run: self.run
        }
    }
}

impl<R> Serialize for SuperbEcs<R>{
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

use std::fmt::Write;
use std::fmt;
use crate::utils::{TypeIdNum, gett};
use crate::rts::game::game_state::UsingResources;
use std::marker::PhantomData;
use crate::ecs::GlobalEntityID;
use std::slice::Iter;
use crate::ecs::pending_entity::PendingEntity;

impl ECSVisitor {
    fn new() -> Self {
        ECSVisitor {}
    }
}
trait TraitTest{

}
impl<'de> Visitor<'de> for ECSVisitor
{
    type Value = SuperbEcs<UsingResources>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("An ECS")
    }

    fn visit_bytes<E>(self, bytes: &[u8]) -> Result<Self::Value, E>
    {
        let comp_store = bincode::deserialize::<CompStorage>(bytes).unwrap();

        return Ok(SuperbEcs{
            systems: crate::rts::game::game_state::global_get_systems(),
            c: comp_store
        });
    }
}

impl<'de> Deserialize<'de> for SuperbEcs<UsingResources>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(ECSVisitor::new())
    }
}