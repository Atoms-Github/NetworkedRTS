

use serde::*;
use anymap::AnyMap;
use crate::ecs::comp_store::*;
use serde::ser::SerializeStruct;
use serde::de::Visitor;


// TODO: Implement ser and de manually.
pub struct SuperbEcs<R>{
    systems: Vec<System<R>>,
    pub comp_storage: CompStorage,
}
impl<R> SuperbEcs<R>{
    pub fn new(systems: Vec<System<R>>) -> Self{
        Self{
            systems: vec![],
            comp_storage: Default::default(),
        }
    }
    pub fn set_systems(&mut self, systems: Vec<System<R>>){
        self.systems = systems;
    }
    pub fn sim_systems(&mut self, resources: R){
        for system in &self.systems{
            (system.run)(&resources, &mut self.comp_storage);
        }
    }

}
impl<R> Clone for SuperbEcs<R>{
    fn clone(&self) -> Self {
        Self{
            systems: self.systems.clone(),
            comp_storage: self.comp_storage.clone(),
        }
    }
}

pub struct System<R>{
    pub run: fn(&R, &mut CompStorage /* Could add read only version here. */),
}

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
        let bytes = bincode::serialize(&self.comp_storage).unwrap();
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
use crate::utils::TypeIdNum;
use crate::rts::game::game_state::UsingResources;

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
            systems: vec![],
            comp_storage: comp_store
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