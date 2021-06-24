

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
            systems: systems,
            c: Default::default(),
        }
    }
    pub fn set_systems(&mut self, systems: Vec<System<R>>){
        self.systems = systems;
    }
    pub fn sim_systems(&mut self, resources: R){
        for system in &self.systems{
            (system.run)(&resources, &mut self.c);
        }
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
    pub run: fn(&R, &mut CompStorage /* Could add read only version here. */),
}


pub struct CompIter3<'a, A : 'static, B : 'static, C : 'static> {
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    ecs: &'a CompStorage,
    vec: Vec<GlobalEntityID>
}// C:/Users/tomul/.rustup/toolchains/nightly-x86_64-pc-windows-gnu/lib/rustlib/src/rust/library/core/src/slice/iter.rs:66
impl<'a, A : 'static, B : 'static, C : 'static> CompIter3<'a, A, B, C>{
    pub fn new(ecs: &'a CompStorage) -> Self{
        let mut my_vec = ecs.query(vec![gett::<A>(), gett::<B>(), gett::<C>()]).iter().as_slice().to_vec();
        my_vec.reverse();
        Self{
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            ecs,
            vec: my_vec,
        }
    }
}
impl<'a, A, B, C> Iterator for CompIter3<'a, A, B, C>{
    type Item = (&'a mut A, &'a mut B, &'a mut C);
    fn next(&mut self) -> Option<Self::Item> {
        let entity_id = self.vec.pop()?;

        return Some((self.ecs.get_mut::<A>(entity_id).unwrap(),
                    self.ecs.get_mut::<B>(entity_id).unwrap(),
                    self.ecs.get_mut::<C>(entity_id).unwrap()
        )
        );
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